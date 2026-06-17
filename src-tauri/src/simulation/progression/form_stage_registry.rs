use super::super::game_state::GameState;

pub const FORM_STAGE_2_TOTAL_LEVEL: u32 = 30;

pub fn form_stage_for_total_level(total_level: u32) -> u32 {
    if total_level >= FORM_STAGE_2_TOTAL_LEVEL {
        2
    } else {
        1
    }
}

pub fn refresh_form_stage(state: &mut GameState) {
    state.form_stage = form_stage_for_total_level(state.total_level());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::game_state::GameState;

    #[test]
    fn form_stage_stays_at_one_below_threshold() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 29).expect("set level");

        refresh_form_stage(&mut state);

        assert_eq!(state.form_stage, 1);
    }

    #[test]
    fn form_stage_advances_to_two_at_total_level_threshold() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 30).expect("set level");

        refresh_form_stage(&mut state);

        assert_eq!(state.form_stage, 2);
    }
}
