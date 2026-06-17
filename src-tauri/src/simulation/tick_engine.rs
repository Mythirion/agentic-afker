use super::game_state::{GameState, SkillId};
use super::progression::{refresh_form_stage, token_gain, xp_gain};
use super::skill_content::refresh_skill_unlocks;
use super::xp_curve::apply_xp;

pub const TICK_MS: u64 = 100;
const SCRAPING_XP_PER_TICK: u64 = 1;
const LABELLING_XP_PER_TICK: u64 = 1;
const LABELLING_TOKENS_PER_TICK: u64 = 1;
const LABELLING_RAW_DATA_PER_TICK: u64 = 1;
const FINE_TUNING_XP_PER_TICK: u64 = 1;
const FINE_TUNING_TOKENS_PER_TICK: u64 = 2;
const FINE_TUNING_LABELLED_DATA_PER_TICK: u64 = 1;

pub fn advance(state: &mut GameState, elapsed_ms: u64) {
    let ticks = elapsed_ms / TICK_MS;

    for _ in 0..ticks {
        refresh_skill_unlocks(state);
        process_tick(state);
        refresh_form_stage(state);
        state.last_tick_at += TICK_MS as i64;
    }
}

fn process_tick(state: &mut GameState) {
    if !state.has_active_skill() {
        return;
    }

    if state.active_skill == SkillId::scraping() {
        apply_scraping_tick(state);
    } else if state.active_skill == SkillId::labelling() {
        apply_labelling_tick(state);
    } else if state.active_skill == SkillId::fine_tuning() {
        apply_fine_tuning_tick(state);
    }
}

fn apply_scraping_tick(state: &mut GameState) {
    let xp_amount = xp_gain(state, "scraping", SCRAPING_XP_PER_TICK);
    let skill_id = SkillId::scraping();
    let skill = state.skills.get_mut(&skill_id).expect("scraping skill exists");
    let (level, xp) = apply_xp(skill.level, skill.xp, xp_amount);
    skill.level = level;
    skill.xp = xp;
    skill.resources = skill.resources.saturating_add(1);
}

fn apply_labelling_tick(state: &mut GameState) {
    let scraping_resources = state
        .skills
        .get(&SkillId::scraping())
        .expect("scraping skill exists")
        .resources;

    if scraping_resources < LABELLING_RAW_DATA_PER_TICK {
        return;
    }

    state
        .skills
        .get_mut(&SkillId::scraping())
        .expect("scraping skill exists")
        .resources -= LABELLING_RAW_DATA_PER_TICK;

    let xp_amount = xp_gain(state, "labelling", LABELLING_XP_PER_TICK);
    let token_amount = token_gain(state, "labelling", LABELLING_TOKENS_PER_TICK);

    let labelling = state
        .skills
        .get_mut(&SkillId::labelling())
        .expect("labelling skill exists");
    let (level, xp) = apply_xp(labelling.level, labelling.xp, xp_amount);
    labelling.level = level;
    labelling.xp = xp;
    labelling.resources = labelling.resources.saturating_add(1);

    state.tokens = state.tokens.saturating_add(token_amount);
}

fn apply_fine_tuning_tick(state: &mut GameState) {
    let labelled_data = state
        .skills
        .get(&SkillId::labelling())
        .expect("labelling skill exists")
        .resources;

    if labelled_data < FINE_TUNING_LABELLED_DATA_PER_TICK {
        return;
    }

    state
        .skills
        .get_mut(&SkillId::labelling())
        .expect("labelling skill exists")
        .resources -= FINE_TUNING_LABELLED_DATA_PER_TICK;

    let xp_amount = xp_gain(state, "fine-tuning", FINE_TUNING_XP_PER_TICK);
    let token_amount = token_gain(state, "fine-tuning", FINE_TUNING_TOKENS_PER_TICK);

    let fine_tuning = state
        .skills
        .get_mut(&SkillId::fine_tuning())
        .expect("fine-tuning skill exists");
    let (level, xp) = apply_xp(fine_tuning.level, fine_tuning.xp, xp_amount);
    fine_tuning.level = level;
    fine_tuning.xp = xp;

    state.tokens = state.tokens.saturating_add(token_amount);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::game_state::GameState;

    #[test]
    fn advance_does_not_progress_while_idle_on_fresh_start() {
        let mut state = GameState::new_fresh_start(0);

        advance(&mut state, 5_000);

        assert!(!state.has_active_skill());
        assert_eq!(state.scraping().xp, 0);
        assert_eq!(state.scraping().level, 1);
    }

    #[test]
    fn advance_increases_scraping_xp_over_elapsed_time() {
        let mut state = GameState::new_scraping_start(0);

        advance(&mut state, 1_000);

        assert_eq!(state.scraping().xp, 10);
        assert_eq!(state.scraping().level, 1);
        assert_eq!(state.last_tick_at, 1_000);
    }

    #[test]
    fn advance_levels_scraping_when_xp_threshold_reached() {
        let mut state = GameState::new_scraping_start(0);

        advance(&mut state, 10_000);

        assert_eq!(state.scraping().xp, 100);
        assert_eq!(state.scraping().level, 2);
    }

    #[test]
    fn advance_only_affects_active_scraping_skill() {
        let mut state = GameState::new_scraping_start(0);
        let before = state.scraping().clone();

        advance(&mut state, 500);

        assert_eq!(state.scraping().xp, before.xp + 5);
        assert_eq!(state.active_skill, SkillId::scraping());
    }

    #[test]
    fn labelling_produces_tokens_and_consumes_scraped_data() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 5).expect("unlock scraping");
        crate::simulation::refresh_skill_unlocks(&mut state);
        {
            let scraping = state.skills.get_mut(&SkillId::scraping()).unwrap();
            scraping.resources = 10;
        }
        crate::simulation::set_active_skill(&mut state, "labelling").expect("activate labelling");

        advance(&mut state, 1_000);

        assert_eq!(state.tokens, 10);
        assert_eq!(state.scraping().resources, 0);
        assert_eq!(state.skills.get(&SkillId::labelling()).unwrap().xp, 10);
        assert_eq!(state.skills.get(&SkillId::labelling()).unwrap().resources, 10);
    }

    #[test]
    fn labelling_does_not_produce_tokens_without_raw_data() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 5).expect("unlock scraping");
        crate::simulation::refresh_skill_unlocks(&mut state);
        crate::simulation::set_active_skill(&mut state, "labelling").expect("activate labelling");

        advance(&mut state, 1_000);

        assert_eq!(state.tokens, 0);
    }

    #[test]
    fn fine_tuning_produces_double_tokens_and_consumes_labelled_data() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 5).expect("unlock scraping");
        crate::simulation::refresh_skill_unlocks(&mut state);
        crate::dev_menu::apply_skill_level(&mut state, "labelling", 10).expect("unlock fine-tuning");
        crate::simulation::refresh_skill_unlocks(&mut state);
        {
            let labelling = state.skills.get_mut(&SkillId::labelling()).unwrap();
            labelling.resources = 5;
        }
        crate::simulation::set_active_skill(&mut state, "fine-tuning").expect("activate fine-tuning");

        advance(&mut state, 500);

        assert_eq!(state.tokens, 10);
        assert_eq!(state.skills.get(&SkillId::labelling()).unwrap().resources, 0);
        assert_eq!(state.skills.get(&SkillId::fine_tuning()).unwrap().xp, 5);
    }

    #[test]
    fn full_pipeline_chain_from_scraping_to_fine_tuning() {
        let mut state = GameState::new_scraping_start(0);
        crate::dev_menu::apply_skill_level(&mut state, "scraping", 5).expect("unlock labelling");
        crate::simulation::refresh_skill_unlocks(&mut state);
        {
            let scraping = state.skills.get_mut(&SkillId::scraping()).unwrap();
            scraping.resources = 20;
        }
        crate::simulation::set_active_skill(&mut state, "labelling").expect("activate labelling");

        advance(&mut state, 2_000);

        assert_eq!(state.scraping().resources, 0);
        assert_eq!(state.skills.get(&SkillId::labelling()).unwrap().resources, 20);
        assert_eq!(state.tokens, 20);

        crate::dev_menu::apply_skill_level(&mut state, "labelling", 10).expect("unlock fine-tuning");
        crate::simulation::refresh_skill_unlocks(&mut state);
        crate::simulation::set_active_skill(&mut state, "fine-tuning").expect("activate fine-tuning");

        advance(&mut state, 1_000);

        assert_eq!(state.skills.get(&SkillId::labelling()).unwrap().resources, 10);
        assert_eq!(state.tokens, 40);
        assert_eq!(state.skills.get(&SkillId::fine_tuning()).unwrap().xp, 10);
    }
}
