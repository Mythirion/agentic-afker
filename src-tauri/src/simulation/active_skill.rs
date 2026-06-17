use super::game_state::{GameState, SkillId};
use super::skill_content::{is_known_skill, is_skill_locked, refresh_skill_unlocks};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetActiveSkillError {
    SkillNotFound(String),
    SkillLocked(String),
}

impl std::fmt::Display for SetActiveSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SkillNotFound(skill_id) => write!(f, "unknown skill: {skill_id}"),
            Self::SkillLocked(skill_id) => write!(f, "skill is locked: {skill_id}"),
        }
    }
}

pub fn set_active_skill(state: &mut GameState, skill_id: &str) -> Result<(), SetActiveSkillError> {
    if !is_known_skill(skill_id) {
        return Err(SetActiveSkillError::SkillNotFound(skill_id.to_string()));
    }

    refresh_skill_unlocks(state);

    if is_skill_locked(state, skill_id) {
        return Err(SetActiveSkillError::SkillLocked(skill_id.to_string()));
    }

    let id = SkillId(skill_id.to_string());
    if !state.skills.contains_key(&id) {
        return Err(SetActiveSkillError::SkillNotFound(skill_id.to_string()));
    }

    state.active_skill = id;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev_menu::apply_skill_level;
    use crate::simulation::{advance, GameState, SkillId, LABELLING_UNLOCK_SCRAPING_LEVEL};

    #[test]
    fn set_active_skill_starts_scraping_from_idle() {
        let mut state = GameState::new_fresh_start(0);
        assert!(!state.has_active_skill());

        set_active_skill(&mut state, "scraping").expect("activate scraping");

        assert_eq!(state.active_skill, SkillId::scraping());
        assert!(state.has_active_skill());
    }

    #[test]
    fn set_active_skill_rejects_unknown_skill() {
        let mut state = GameState::new_fresh_start(0);

        let error = set_active_skill(&mut state, "orchestration").expect_err("unknown skill");
        assert_eq!(
            error,
            SetActiveSkillError::SkillNotFound("orchestration".to_string())
        );
    }

    #[test]
    fn set_active_skill_rejects_locked_labelling() {
        let mut state = GameState::new_fresh_start(0);

        let error = set_active_skill(&mut state, "labelling").expect_err("locked skill");
        assert_eq!(
            error,
            SetActiveSkillError::SkillLocked("labelling".to_string())
        );
    }

    #[test]
    fn set_active_skill_allows_labelling_when_unlocked() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(
            &mut state,
            "scraping",
            LABELLING_UNLOCK_SCRAPING_LEVEL,
        )
        .expect("set scraping level");
        refresh_skill_unlocks(&mut state);

        set_active_skill(&mut state, "labelling").expect("activate labelling");

        assert_eq!(state.active_skill, SkillId::labelling());
    }

    #[test]
    fn set_active_skill_rejects_locked_fine_tuning() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(
            &mut state,
            "scraping",
            LABELLING_UNLOCK_SCRAPING_LEVEL,
        )
        .expect("set scraping level");
        refresh_skill_unlocks(&mut state);

        let error = set_active_skill(&mut state, "fine-tuning").expect_err("locked skill");
        assert_eq!(
            error,
            SetActiveSkillError::SkillLocked("fine-tuning".to_string())
        );
    }

    #[test]
    fn set_active_skill_allows_fine_tuning_when_unlocked() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(
            &mut state,
            "scraping",
            LABELLING_UNLOCK_SCRAPING_LEVEL,
        )
        .expect("set scraping level");
        refresh_skill_unlocks(&mut state);
        apply_skill_level(&mut state, "labelling", 10).expect("set labelling level");
        refresh_skill_unlocks(&mut state);

        set_active_skill(&mut state, "fine-tuning").expect("activate fine-tuning");

        assert_eq!(state.active_skill, SkillId::fine_tuning());
    }

    #[test]
    fn active_skill_gains_xp_after_starting_scraping() {
        let mut state = GameState::new_fresh_start(0);
        set_active_skill(&mut state, "scraping").expect("activate scraping");

        advance(&mut state, 1_000);

        assert_eq!(state.scraping().xp, 10);
    }
}
