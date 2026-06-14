use crate::dev_menu::{apply_skill_level, dev_menu_enabled, DevMenuError};
use crate::persistence::SaveRepository;
use crate::simulation::{advance, GameState, progress_within_level};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager, State};

pub const GAME_STATE_EVENT: &str = "game-state";
const TICK_INTERVAL_MS: u64 = 1_000;
const SAVE_DEBOUNCE_TICKS: u64 = 5;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillSnapshot {
    pub id: String,
    pub level: u32,
    pub xp: u64,
    pub xp_into_level: u64,
    pub xp_to_next_level: u64,
    pub level_progress: f64,
    pub is_active: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateSnapshot {
    pub active_skill: String,
    pub skills: Vec<SkillSnapshot>,
    pub tokens: u64,
    pub total_level: u32,
    pub last_tick_at: i64,
}

pub struct GameRuntime {
    state: Mutex<GameState>,
    save_path: PathBuf,
    ticks_since_save: Mutex<u64>,
}

impl GameRuntime {
    pub fn initialize(app: &AppHandle) -> Result<Self, String> {
        let save_dir = app.path().app_data_dir().map_err(|err| err.to_string())?;
        std::fs::create_dir_all(&save_dir).map_err(|err| err.to_string())?;
        let save_path = save_dir.join("save.db");

        let repo = SaveRepository::open(&save_path).map_err(|err| err.to_string())?;
        let now = now_ms();
        let state = repo
            .load()
            .map_err(|err| err.to_string())?
            .unwrap_or_else(|| GameState::new_fresh_start(now));

        Ok(Self {
            state: Mutex::new(state),
            save_path,
            ticks_since_save: Mutex::new(0),
        })
    }

    pub fn snapshot(&self) -> GameStateSnapshot {
        let state = self.state.lock().expect("game state lock");
        to_snapshot(&state)
    }

    pub fn tick_and_maybe_save(&self, app: &AppHandle) -> Result<(), String> {
        {
            let mut state = self.state.lock().expect("game state lock");
            advance(&mut state, TICK_INTERVAL_MS);
        }

        self.emit_snapshot(app)?;

        let mut ticks = self.ticks_since_save.lock().expect("save ticks lock");
        *ticks += 1;
        if *ticks >= SAVE_DEBOUNCE_TICKS {
            self.persist()?;
            *ticks = 0;
        }

        Ok(())
    }

    pub fn persist(&self) -> Result<(), String> {
        let state = self.state.lock().expect("game state lock");
        let repo = SaveRepository::open(&self.save_path).map_err(|err| err.to_string())?;
        repo.save(&state).map_err(|err| err.to_string())
    }

    pub fn emit_snapshot(&self, app: &AppHandle) -> Result<(), String> {
        app.emit(GAME_STATE_EVENT, self.snapshot())
            .map_err(|err| err.to_string())
    }

    pub fn dev_set_skill_level(
        &self,
        app: &AppHandle,
        skill_id: &str,
        level: u32,
    ) -> Result<(), String> {
        if !dev_menu_enabled() {
            return Err(DevMenuError::DevMenuDisabled.to_string());
        }

        {
            let mut state = self.state.lock().expect("game state lock");
            apply_skill_level(&mut state, skill_id, level).map_err(|err| err.to_string())?;
        }

        self.persist()?;
        *self.ticks_since_save.lock().expect("save ticks lock") = 0;
        self.emit_snapshot(app)
    }
}

pub fn to_snapshot(state: &GameState) -> GameStateSnapshot {
    let mut skills: Vec<SkillSnapshot> = state
        .skills
        .iter()
        .map(|(id, skill)| {
            let (xp_into_level, xp_to_next_level, level_progress) =
                progress_within_level(skill.level, skill.xp);

            SkillSnapshot {
                id: id.0.clone(),
                level: skill.level,
                xp: skill.xp,
                xp_into_level,
                xp_to_next_level,
                level_progress,
                is_active: *id == state.active_skill,
            }
        })
        .collect();
    skills.sort_by(|left, right| left.id.cmp(&right.id));

    GameStateSnapshot {
        active_skill: state.active_skill.0.clone(),
        skills,
        tokens: state.tokens,
        total_level: state.total_level(),
        last_tick_at: state.last_tick_at,
    }
}

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis() as i64
}

pub fn start_tick_loop(app: AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(TICK_INTERVAL_MS));

            let Some(runtime) = app.try_state::<GameRuntime>() else {
                continue;
            };

            if let Err(err) = runtime.tick_and_maybe_save(&app) {
                eprintln!("tick error: {err}");
            }
        }
    });
}

pub fn save_on_exit(app: &AppHandle) {
    let Some(runtime) = app.try_state::<GameRuntime>() else {
        return;
    };

    if let Err(err) = runtime.persist() {
        eprintln!("failed to save on exit: {err}");
    }
}

#[tauri::command]
pub fn get_game_state(runtime: State<'_, GameRuntime>) -> GameStateSnapshot {
    runtime.snapshot()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::{advance, GameState, SkillId};

    #[test]
    fn fresh_start_snapshot_has_no_active_skill() {
        let state = GameState::new_fresh_start(0);
        let snapshot = to_snapshot(&state);

        assert_eq!(snapshot.active_skill, "");
        assert!(snapshot.skills.iter().all(|skill| !skill.is_active));
        assert_eq!(snapshot.total_level, 1);
    }

    #[test]
    fn snapshot_marks_active_scraping_skill() {
        let mut state = GameState::new_scraping_start(0);
        advance(&mut state, 500);

        let snapshot = to_snapshot(&state);
        let scraping = snapshot
            .skills
            .iter()
            .find(|skill| skill.id == "scraping")
            .expect("scraping skill");

        assert!(scraping.is_active);
        assert_eq!(scraping.xp, 5);
        assert_eq!(scraping.xp_into_level, 5);
        assert_eq!(scraping.xp_to_next_level, 83);
        assert!((scraping.level_progress - (5.0 / 83.0)).abs() < f64::EPSILON);
        assert_eq!(snapshot.active_skill, SkillId::scraping().0);
        assert_eq!(snapshot.total_level, 1);
    }
}
