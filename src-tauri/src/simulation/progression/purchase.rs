use super::super::game_state::GameState;
use super::upgrade_registry::{
    all_upgrade_definitions, find_upgrade, is_upgrade_unlocked, skill_id_for_upgrade,
    total_level_requirement, UpgradeDefinition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PurchaseError {
    UnknownUpgrade,
    AlreadyPurchased,
    NotUnlocked(String),
    InsufficientTokens,
}

impl std::fmt::Display for PurchaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownUpgrade => write!(f, "unknown upgrade"),
            Self::AlreadyPurchased => write!(f, "upgrade already purchased"),
            Self::NotUnlocked(reason) => write!(f, "{reason}"),
            Self::InsufficientTokens => write!(f, "not enough Tokens"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeOffer {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: u64,
    pub is_purchased: bool,
    pub can_purchase: bool,
    pub lock_reason: Option<String>,
}

pub fn purchase_upgrade(state: &mut GameState, upgrade_id: &str) -> Result<(), PurchaseError> {
    let upgrade = find_upgrade(upgrade_id).ok_or(PurchaseError::UnknownUpgrade)?;

    if state.purchased_upgrades.contains(upgrade_id) {
        return Err(PurchaseError::AlreadyPurchased);
    }

    if !is_upgrade_unlocked(state, &upgrade) {
        return Err(PurchaseError::NotUnlocked(lock_reason(state, &upgrade)));
    }

    if state.tokens < upgrade.cost {
        return Err(PurchaseError::InsufficientTokens);
    }

    state.tokens -= upgrade.cost;
    state.purchased_upgrades.insert(upgrade_id.to_string());
    Ok(())
}

pub fn upgrade_snapshots(state: &GameState) -> Vec<UpgradeOffer> {
    all_upgrade_definitions()
        .into_iter()
        .filter(|upgrade| is_upgrade_visible(state, upgrade))
        .map(|upgrade| to_offer(state, upgrade))
        .collect()
}

fn is_upgrade_visible(state: &GameState, upgrade: &UpgradeDefinition) -> bool {
    is_upgrade_unlocked(state, upgrade) || state.purchased_upgrades.contains(&upgrade.id)
}

fn to_offer(state: &GameState, upgrade: UpgradeDefinition) -> UpgradeOffer {
    let is_purchased = state.purchased_upgrades.contains(&upgrade.id);
    let unlocked = is_upgrade_unlocked(state, &upgrade);
    let can_afford = state.tokens >= upgrade.cost;

    let lock_reason = if is_purchased {
        None
    } else if !unlocked {
        Some(lock_reason(state, &upgrade))
    } else if !can_afford {
        Some("Not enough Tokens".to_string())
    } else {
        None
    };

    UpgradeOffer {
        id: upgrade.id,
        name: upgrade.name,
        description: upgrade.description,
        cost: upgrade.cost,
        is_purchased,
        can_purchase: !is_purchased && unlocked && can_afford,
        lock_reason,
    }
}

fn lock_reason(state: &GameState, upgrade: &UpgradeDefinition) -> String {
    if let Some(skill_id) = skill_id_for_upgrade(upgrade) {
        let required = upgrade
            .kind
            .clone();
        let milestone = match required {
            super::upgrade_registry::UpgradeKind::SkillXp { milestone, .. }
            | super::upgrade_registry::UpgradeKind::SkillTokenYield { milestone, .. } => milestone,
            _ => unreachable!(),
        };
        return format!("Requires {skill_id} Lv {milestone}");
    }

    if let Some(required_total) = total_level_requirement(upgrade) {
        return format!("Requires Total Level {required_total}");
    }

    let _ = state;
    "Locked".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dev_menu::apply_skill_level;
    use crate::simulation::GameState;

    #[test]
    fn purchase_deducts_tokens_and_records_upgrade() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 10).expect("level scraping");
        state.tokens = 200;

        purchase_upgrade(&mut state, "scraping-10").expect("purchase");

        assert_eq!(state.tokens, 100);
        assert!(state.purchased_upgrades.contains("scraping-10"));
    }

    #[test]
    fn purchase_rejects_unaffordable_upgrade() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 10).expect("level scraping");
        state.tokens = 50;

        let error = purchase_upgrade(&mut state, "scraping-10").expect_err("too poor");
        assert_eq!(error, PurchaseError::InsufficientTokens);
        assert!(!state.purchased_upgrades.contains("scraping-10"));
    }

    #[test]
    fn purchase_rejects_locked_upgrade() {
        let mut state = GameState::new_scraping_start(0);
        state.tokens = 1_000;

        let error = purchase_upgrade(&mut state, "scraping-10").expect_err("locked");
        assert!(matches!(error, PurchaseError::NotUnlocked(_)));
    }

    #[test]
    fn purchase_rejects_already_purchased_upgrade() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 10).expect("level scraping");
        state.tokens = 500;
        state.purchased_upgrades.insert("scraping-10".to_string());

        let error = purchase_upgrade(&mut state, "scraping-10").expect_err("duplicate");
        assert_eq!(error, PurchaseError::AlreadyPurchased);
    }

    #[test]
    fn infrastructure_purchase_requires_total_level() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 19).expect("level scraping");
        state.tokens = 2_000;

        let error =
            purchase_upgrade(&mut state, "expanded-context-window").expect_err("total level");
        assert!(matches!(error, PurchaseError::NotUnlocked(_)));

        apply_skill_level(&mut state, "scraping", 20).expect("reach total level gate");

        purchase_upgrade(&mut state, "expanded-context-window").expect("purchase infra");
        assert!(state
            .purchased_upgrades
            .contains("expanded-context-window"));
    }

    #[test]
    fn snapshots_list_unlocked_upgrades_with_costs() {
        let mut state = GameState::new_scraping_start(0);
        apply_skill_level(&mut state, "scraping", 10).expect("level scraping");
        state.tokens = 50;

        let offers = upgrade_snapshots(&state);
        let scraping_ten = offers
            .iter()
            .find(|offer| offer.id == "scraping-10")
            .expect("scraping-10 offer");

        assert_eq!(scraping_ten.cost, 100);
        assert!(!scraping_ten.is_purchased);
        assert!(!scraping_ten.can_purchase);
        assert_eq!(scraping_ten.lock_reason.as_deref(), Some("Not enough Tokens"));
    }
}
