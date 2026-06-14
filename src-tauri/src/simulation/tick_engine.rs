use super::game_state::{GameState, SkillId};
use super::xp_curve::apply_xp;

pub const TICK_MS: u64 = 100;
const SCRAPING_XP_PER_TICK: u64 = 1;

pub fn advance(state: &mut GameState, elapsed_ms: u64) {
    let ticks = elapsed_ms / TICK_MS;

    for _ in 0..ticks {
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
}
