use super::super::game_state::GameState;
use super::upgrade_registry::{
    find_upgrade, skill_id_for_upgrade, upgrade_bonus_bps, UpgradeKind,
    EXPANDED_CONTEXT_WINDOW_ID,
};

const BASE_BPS: u64 = 100;

pub fn xp_multiplier_bps(state: &GameState, skill_id: &str) -> u64 {
    let bonus = state
        .purchased_upgrades
        .iter()
        .filter_map(|id| find_upgrade(id))
        .filter(|upgrade| {
            matches!(
                &upgrade.kind,
                UpgradeKind::SkillXp { skill_id: id, .. } if id == skill_id
            )
        })
        .count() as u64
        * upgrade_bonus_bps();

    BASE_BPS + bonus
}

pub fn token_multiplier_bps(state: &GameState, skill_id: &str) -> u64 {
    let skill_bonus = state
        .purchased_upgrades
        .iter()
        .filter_map(|id| find_upgrade(id))
        .filter(|upgrade| {
            matches!(
                &upgrade.kind,
                UpgradeKind::SkillTokenYield { skill_id: id, .. } if id == skill_id
            )
        })
        .count() as u64
        * upgrade_bonus_bps();

    let global_bonus = if state.purchased_upgrades.contains(EXPANDED_CONTEXT_WINDOW_ID) {
        upgrade_bonus_bps()
    } else {
        0
    };

    BASE_BPS + skill_bonus + global_bonus
}

pub fn xp_gain(state: &GameState, skill_id: &str, base: u64) -> u64 {
    base.saturating_mul(xp_multiplier_bps(state, skill_id)) / BASE_BPS
}

pub fn token_gain(state: &GameState, skill_id: &str, base: u64) -> u64 {
    base.saturating_mul(token_multiplier_bps(state, skill_id)) / BASE_BPS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::GameState;

    #[test]
    fn xp_multiplier_increases_with_purchased_scraping_upgrades() {
        let mut state = GameState::new_scraping_start(0);
        assert_eq!(xp_multiplier_bps(&state, "scraping"), 100);

        state.purchased_upgrades.insert("scraping-10".to_string());
        assert_eq!(xp_multiplier_bps(&state, "scraping"), 110);

        state.purchased_upgrades.insert("scraping-20".to_string());
        assert_eq!(xp_multiplier_bps(&state, "scraping"), 120);
    }

    #[test]
    fn token_multiplier_stacks_skill_and_infrastructure_upgrades() {
        let mut state = GameState::new_scraping_start(0);
        assert_eq!(token_multiplier_bps(&state, "labelling"), 100);

        state.purchased_upgrades.insert("labelling-10".to_string());
        assert_eq!(token_multiplier_bps(&state, "labelling"), 110);

        state.purchased_upgrades.insert(EXPANDED_CONTEXT_WINDOW_ID.to_string());
        assert_eq!(token_multiplier_bps(&state, "labelling"), 120);
        assert_eq!(token_multiplier_bps(&state, "fine-tuning"), 110);
    }

    #[test]
    fn xp_gain_applies_multiplier_to_base_amount() {
        let mut state = GameState::new_scraping_start(0);
        state.purchased_upgrades.insert("scraping-10".to_string());

        assert_eq!(xp_gain(&state, "scraping", 10), 11);
    }

    #[test]
    fn token_gain_applies_multiplier_to_base_amount() {
        let mut state = GameState::new_scraping_start(0);
        state.purchased_upgrades.insert("fine-tuning-10".to_string());
        state.purchased_upgrades.insert(EXPANDED_CONTEXT_WINDOW_ID.to_string());

        assert_eq!(token_gain(&state, "fine-tuning", 10), 12);
    }
}
