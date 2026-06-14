use super::game_state::{GameState, SkillId};
use super::skill_content::refresh_skill_unlocks;
use super::xp_curve::apply_xp;

pub const TICK_MS: u64 = 100;
const SCRAPING_XP_PER_TICK: u64 = 1;
const LABELLING_XP_PER_TICK: u64 = 1;
const LABELLING_TOKENS_PER_TICK: u64 = 1;
const LABELLING_RAW_DATA_PER_TICK: u64 = 1;

pub fn advance(state: &mut GameState, elapsed_ms: u64) {
    let ticks = elapsed_ms / TICK_MS;

    for _ in 0..ticks {
        refresh_skill_unlocks(state);
        process_tick(state);
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
    }
}

fn apply_scraping_tick(state: &mut GameState) {
    let skill_id = SkillId::scraping();
    let skill = state.skills.get_mut(&skill_id).expect("scraping skill exists");
    let (level, xp) = apply_xp(skill.level, skill.xp, SCRAPING_XP_PER_TICK);
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

    let labelling = state
        .skills
        .get_mut(&SkillId::labelling())
        .expect("labelling skill exists");
    let (level, xp) = apply_xp(labelling.level, labelling.xp, LABELLING_XP_PER_TICK);
    labelling.level = level;
    labelling.xp = xp;

    state.tokens = state
        .tokens
        .saturating_add(LABELLING_TOKENS_PER_TICK);
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
}
