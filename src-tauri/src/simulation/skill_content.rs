use super::game_state::{GameState, SkillId, SkillState};

pub const LABELLING_UNLOCK_SCRAPING_LEVEL: u32 = 5;
pub const FINE_TUNING_UNLOCK_LABELLING_LEVEL: u32 = 10;

pub const ALPHA_PIPELINE_SKILLS: [&str; 3] = ["scraping", "labelling", "fine-tuning"];

pub fn labelling_prerequisite_text() -> &'static str {
    "Requires Scraping Lv 5"
}

pub fn fine_tuning_prerequisite_text() -> &'static str {
    "Requires Labelling Lv 10"
}

pub fn is_labelling_unlocked(state: &GameState) -> bool {
    state.scraping().level >= LABELLING_UNLOCK_SCRAPING_LEVEL
}

pub fn is_fine_tuning_unlocked(state: &GameState) -> bool {
    state
        .skills
        .get(&SkillId::labelling())
        .is_some_and(|skill| skill.level >= FINE_TUNING_UNLOCK_LABELLING_LEVEL)
}

pub fn is_skill_locked(state: &GameState, skill_id: &str) -> bool {
    match skill_id {
        "scraping" => false,
        "labelling" => !is_labelling_unlocked(state),
        "fine-tuning" => !is_fine_tuning_unlocked(state),
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

    if is_fine_tuning_unlocked(state) && !state.skills.contains_key(&SkillId::fine_tuning()) {
        state
            .skills
            .insert(SkillId::fine_tuning(), SkillState::new());
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

    #[test]
    fn fine_tuning_locked_before_labelling_reaches_threshold() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(&mut state, "scraping", LABELLING_UNLOCK_SCRAPING_LEVEL)
            .expect("set scraping level");
        refresh_skill_unlocks(&mut state);
        apply_skill_level(&mut state, "labelling", 9).expect("set labelling level");

        assert!(!is_fine_tuning_unlocked(&state));
        assert!(is_skill_locked(&state, "fine-tuning"));
    }

    #[test]
    fn fine_tuning_unlocks_when_labelling_reaches_threshold() {
        let mut state = GameState::new_fresh_start(0);
        apply_skill_level(&mut state, "scraping", LABELLING_UNLOCK_SCRAPING_LEVEL)
            .expect("set scraping level");
        refresh_skill_unlocks(&mut state);
        apply_skill_level(&mut state, "labelling", FINE_TUNING_UNLOCK_LABELLING_LEVEL)
            .expect("set labelling level");

        refresh_skill_unlocks(&mut state);

        assert!(is_fine_tuning_unlocked(&state));
        assert!(!is_skill_locked(&state, "fine-tuning"));
        assert!(state.skills.contains_key(&SkillId::fine_tuning()));
    }
}
