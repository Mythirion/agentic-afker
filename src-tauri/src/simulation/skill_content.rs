use super::game_state::{GameState, SkillId, SkillState};

pub const LABELLING_UNLOCK_SCRAPING_LEVEL: u32 = 5;

pub const ALPHA_PIPELINE_SKILLS: [&str; 2] = ["scraping", "labelling"];

pub fn labelling_prerequisite_text() -> &'static str {
    "Requires Scraping Lv 5"
}

pub fn is_labelling_unlocked(state: &GameState) -> bool {
    state.scraping().level >= LABELLING_UNLOCK_SCRAPING_LEVEL
}

pub fn is_skill_locked(state: &GameState, skill_id: &str) -> bool {
    match skill_id {
        "scraping" => false,
        "labelling" => !is_labelling_unlocked(state),
        _ => false,
    }
}

pub fn is_known_skill(skill_id: &str) -> bool {
    ALPHA_PIPELINE_SKILLS.contains(&skill_id)
}

pub fn refresh_skill_unlocks(state: &mut GameState) {
    if is_labelling_unlocked(state) && !state.skills.contains_key(&SkillId::labelling()) {
        state
            .skills
            .insert(SkillId::labelling(), SkillState::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev_menu::apply_skill_level;
    use crate::simulation::GameState;

    #[test]
    fn labelling_locked_before_scraping_reaches_threshold() {
        let state = GameState::new_fresh_start(0);

        assert!(!is_labelling_unlocked(&state));
        assert!(is_skill_locked(&state, "labelling"));
    }

    #[test]
    fn labelling_unlocks_when_scraping_reaches_threshold() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(&mut state, "scraping", LABELLING_UNLOCK_SCRAPING_LEVEL)
            .expect("set scraping level");

        refresh_skill_unlocks(&mut state);

        assert!(is_labelling_unlocked(&state));
        assert!(!is_skill_locked(&state, "labelling"));
        assert!(state.skills.contains_key(&SkillId::labelling()));
    }
}
