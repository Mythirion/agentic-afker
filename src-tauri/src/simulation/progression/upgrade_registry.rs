use super::super::game_state::GameState;
use super::super::skill_content::ALPHA_PIPELINE_SKILLS;

pub const SKILL_UPGRADE_MILESTONES: [u32; 3] = [10, 20, 30];
pub const EXPANDED_CONTEXT_WINDOW_ID: &str = "expanded-context-window";

const SKILL_UPGRADE_COSTS: [u64; 3] = [100, 250, 500];
const INFRASTRUCTURE_TOTAL_LEVEL_GATE: u32 = 20;
const EXPANDED_CONTEXT_WINDOW_COST: u64 = 1_000;

const UPGRADE_BONUS_BPS: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeKind {
    SkillXp { skill_id: String, milestone: u32 },
    SkillTokenYield { skill_id: String, milestone: u32 },
    GlobalTokenYield,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: u64,
    pub kind: UpgradeKind,
}

pub fn skill_upgrade_id(skill_id: &str, milestone: u32) -> String {
    format!("{skill_id}-{milestone}")
}

pub fn all_upgrade_definitions() -> Vec<UpgradeDefinition> {
    let mut upgrades = Vec::new();

    for skill_id in ALPHA_PIPELINE_SKILLS {
        let (name_prefix, kind_for) = match skill_id {
            "scraping" => ("Scraping", "xp"),
            "labelling" => ("Labelling", "token"),
            "fine-tuning" => ("Fine-Tuning", "token"),
            _ => continue,
        };

        for (index, milestone) in SKILL_UPGRADE_MILESTONES.iter().enumerate() {
            let effect = if kind_for == "xp" {
                format!("+{UPGRADE_BONUS_BPS}% XP rate")
            } else {
                format!("+{UPGRADE_BONUS_BPS}% Token yield")
            };

            let kind = if kind_for == "xp" {
                UpgradeKind::SkillXp {
                    skill_id: skill_id.to_string(),
                    milestone: *milestone,
                }
            } else {
                UpgradeKind::SkillTokenYield {
                    skill_id: skill_id.to_string(),
                    milestone: *milestone,
                }
            };

            upgrades.push(UpgradeDefinition {
                id: skill_upgrade_id(skill_id, *milestone),
                name: format!("{name_prefix} Lv {milestone} Upgrade"),
                description: effect,
                cost: SKILL_UPGRADE_COSTS[index],
                kind,
            });
        }
    }

    upgrades.push(UpgradeDefinition {
        id: EXPANDED_CONTEXT_WINDOW_ID.to_string(),
        name: "Expanded Context Window".to_string(),
        description: format!("+{UPGRADE_BONUS_BPS}% Token yield (all Production Skills)"),
        cost: EXPANDED_CONTEXT_WINDOW_COST,
        kind: UpgradeKind::GlobalTokenYield,
    });

    upgrades
}

pub fn find_upgrade(upgrade_id: &str) -> Option<UpgradeDefinition> {
    all_upgrade_definitions()
        .into_iter()
        .find(|upgrade| upgrade.id == upgrade_id)
}

pub fn skill_level_requirement(upgrade: &UpgradeDefinition) -> Option<u32> {
    match &upgrade.kind {
        UpgradeKind::SkillXp { milestone, .. } | UpgradeKind::SkillTokenYield { milestone, .. } => {
            Some(*milestone)
        }
        UpgradeKind::GlobalTokenYield => None,
    }
}

pub fn skill_id_for_upgrade(upgrade: &UpgradeDefinition) -> Option<&str> {
    match &upgrade.kind {
        UpgradeKind::SkillXp { skill_id, .. } | UpgradeKind::SkillTokenYield { skill_id, .. } => {
            Some(skill_id)
        }
        UpgradeKind::GlobalTokenYield => None,
    }
}

pub fn total_level_requirement(upgrade: &UpgradeDefinition) -> Option<u32> {
    match upgrade.kind {
        UpgradeKind::GlobalTokenYield => Some(INFRASTRUCTURE_TOTAL_LEVEL_GATE),
        _ => None,
    }
}

pub fn is_upgrade_unlocked(state: &GameState, upgrade: &UpgradeDefinition) -> bool {
    if let Some(skill_id) = skill_id_for_upgrade(upgrade) {
        let required_level = skill_level_requirement(upgrade).expect("skill upgrade level");
        return state
            .skills
            .get(&super::super::game_state::SkillId(skill_id.to_string()))
            .is_some_and(|skill| skill.level >= required_level);
    }

    if let Some(required_total) = total_level_requirement(upgrade) {
        return state.total_level() >= required_total;
    }

    false
}

pub fn upgrade_bonus_bps() -> u64 {
    UPGRADE_BONUS_BPS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::GameState;

    #[test]
    fn defines_skill_upgrades_at_milestones_with_costs() {
        let scraping_ten = find_upgrade("scraping-10").expect("scraping-10");
        assert_eq!(scraping_ten.cost, 100);
        assert_eq!(skill_level_requirement(&scraping_ten), Some(10));

        let scraping_thirty = find_upgrade("scraping-30").expect("scraping-30");
        assert_eq!(scraping_thirty.cost, 500);
    }

    #[test]
    fn defines_infrastructure_upgrade_gated_by_total_level() {
        let infra = find_upgrade(EXPANDED_CONTEXT_WINDOW_ID).expect("infrastructure");
        assert_eq!(infra.cost, 1_000);
        assert_eq!(total_level_requirement(&infra), Some(20));
    }

    #[test]
    fn skill_upgrade_unlocked_when_skill_reaches_milestone() {
        let mut state = GameState::new_scraping_start(0);
        let upgrade = find_upgrade("scraping-10").expect("upgrade");

        assert!(!is_upgrade_unlocked(&state, &upgrade));

        let scraping = state
            .skills
            .get_mut(&crate::simulation::SkillId::scraping())
            .unwrap();
        scraping.level = 10;

        assert!(is_upgrade_unlocked(&state, &upgrade));
    }
}
