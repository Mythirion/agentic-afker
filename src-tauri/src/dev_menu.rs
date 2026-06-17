//! Dev menu — playtesting helpers gated by `AGENTIC_AFKER_DEV=1`.

use crate::simulation::{GameState, SkillId, refresh_form_stage, refresh_skill_unlocks, xp_for_level};

pub const DEV_MENU_ENV_VAR: &str = "AGENTIC_AFKER_DEV";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevMenuError {
    DevMenuDisabled,
    SkillNotFound(String),
    InvalidLevel,
}

impl std::fmt::Display for DevMenuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DevMenuDisabled => write!(f, "dev menu is not enabled"),
            Self::SkillNotFound(skill_id) => write!(f, "unknown skill: {skill_id}"),
            Self::InvalidLevel => write!(f, "level must be at least 1"),
        }
    }
}

pub fn dev_menu_enabled() -> bool {
    dev_menu_enabled_for(std::env::var(DEV_MENU_ENV_VAR).ok())
}

fn dev_menu_enabled_for(value: Option<String>) -> bool {
    matches!(value.as_deref(), Some("1"))
}

pub fn apply_set_tokens(state: &mut GameState, amount: u64) {
    state.tokens = amount;
}

pub fn apply_add_tokens(state: &mut GameState, amount: u64) {
    state.tokens = state.tokens.saturating_add(amount);
}

pub fn apply_reset_save(state: &mut GameState, now_ms: i64) {
    *state = GameState::new_fresh_start(now_ms);
}

pub fn apply_skill_level(
    state: &mut GameState,
    skill_id: &str,
    level: u32,
) -> Result<(), DevMenuError> {
    if level < 1 {
        return Err(DevMenuError::InvalidLevel);
    }

    refresh_skill_unlocks(state);

    let id = SkillId(skill_id.to_string());
    let skill = state
        .skills
        .get_mut(&id)
        .ok_or_else(|| DevMenuError::SkillNotFound(skill_id.to_string()))?;

    skill.level = level;
    skill.xp = xp_for_level(level);
    refresh_form_stage(state);

    Ok(())
}

#[tauri::command]
pub fn is_dev_menu_enabled() -> bool {
    dev_menu_enabled()
}

#[tauri::command]
pub fn dev_set_skill_level(
    skill_id: String,
    level: u32,
    runtime: tauri::State<crate::game_runtime::GameRuntime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    runtime.dev_set_skill_level(&app, &skill_id, level)
}

#[tauri::command]
pub fn dev_set_tokens(
    amount: u64,
    runtime: tauri::State<crate::game_runtime::GameRuntime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    runtime.dev_set_tokens(&app, amount)
}

#[tauri::command]
pub fn dev_add_tokens(
    amount: u64,
    runtime: tauri::State<crate::game_runtime::GameRuntime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    runtime.dev_add_tokens(&app, amount)
}

#[tauri::command]
pub fn dev_reset_save(
    runtime: tauri::State<crate::game_runtime::GameRuntime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    runtime.dev_reset_save(&app)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::GameState;

    #[test]
    fn dev_menu_disabled_without_env_value() {
        assert!(!dev_menu_enabled_for(None));
        assert!(!dev_menu_enabled_for(Some("0".to_string())));
        assert!(!dev_menu_enabled_for(Some("true".to_string())));
    }

    #[test]
    fn dev_menu_enabled_when_env_is_one() {
        assert!(dev_menu_enabled_for(Some("1".to_string())));
    }

    #[test]
    fn apply_skill_level_snaps_xp_to_runescape_floor() {
        let mut state = GameState::new_scraping_start(0);

        apply_skill_level(&mut state, "scraping", 10).expect("set level");

        let scraping = state.scraping();
        assert_eq!(scraping.level, 10);
        assert_eq!(scraping.xp, xp_for_level(10));
    }

    #[test]
    fn apply_skill_level_rejects_unknown_skill() {
        let mut state = GameState::new_scraping_start(0);

        let error = apply_skill_level(&mut state, "labelling", 5).expect_err("unknown skill");
        assert_eq!(error, DevMenuError::SkillNotFound("labelling".to_string()));
    }

    #[test]
    fn apply_skill_level_rejects_level_zero() {
        let mut state = GameState::new_scraping_start(0);

        let error = apply_skill_level(&mut state, "scraping", 0).expect_err("invalid level");
        assert_eq!(error, DevMenuError::InvalidLevel);
    }

    #[test]
    fn apply_set_tokens_sets_exact_balance() {
        let mut state = GameState::new_scraping_start(0);
        state.tokens = 42;

        apply_set_tokens(&mut state, 250);

        assert_eq!(state.tokens, 250);
    }

    #[test]
    fn apply_add_tokens_increments_balance() {
        let mut state = GameState::new_scraping_start(0);
        state.tokens = 100;

        apply_add_tokens(&mut state, 35);

        assert_eq!(state.tokens, 135);
    }

    #[test]
    fn apply_reset_save_restores_fresh_alpha_start() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 12).expect("set level");
        state.tokens = 999;
        state.form_stage = 2;
        state.active_skill = crate::simulation::SkillId::scraping();

        apply_reset_save(&mut state, 5_000);

        assert_eq!(state.scraping().level, 1);
        assert_eq!(state.scraping().xp, 0);
        assert_eq!(state.tokens, 0);
        assert_eq!(state.form_stage, 1);
        assert!(!state.has_active_skill());
        assert_eq!(state.last_tick_at, 5_000);
    }
}
