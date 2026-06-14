use super::game_state::{GameState, SkillId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetActiveSkillError {
    SkillNotFound(String),
}

impl std::fmt::Display for SetActiveSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SkillNotFound(skill_id) => write!(f, "unknown skill: {skill_id}"),
        }
    }
}

pub fn set_active_skill(state: &mut GameState, skill_id: &str) -> Result<(), SetActiveSkillError> {
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
    use crate::simulation::{advance, GameState, SkillId};

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

        let error = set_active_skill(&mut state, "labelling").expect_err("unknown skill");
        assert_eq!(error, SetActiveSkillError::SkillNotFound("labelling".to_string()));
    }

    #[test]
    fn active_skill_gains_xp_after_starting_scraping() {
        let mut state = GameState::new_fresh_start(0);
        set_active_skill(&mut state, "scraping").expect("activate scraping");

        advance(&mut state, 1_000);

        assert_eq!(state.scraping().xp, 10);
    }
}
