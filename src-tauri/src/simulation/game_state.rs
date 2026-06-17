use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId(pub String);

impl SkillId {
    pub fn scraping() -> Self {
        Self("scraping".to_string())
    }

    pub fn labelling() -> Self {
        Self("labelling".to_string())
    }

    pub fn fine_tuning() -> Self {
        Self("fine-tuning".to_string())
    }

    pub fn idle() -> Self {
        Self(String::new())
    }

    pub fn is_idle(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillState {
    pub level: u32,
    pub xp: u64,
    pub resources: u64,
}

impl SkillState {
    pub fn new() -> Self {
        Self {
            level: 1,
            xp: 0,
            resources: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub skills: HashMap<SkillId, SkillState>,
    pub active_skill: SkillId,
    pub tokens: u64,
    pub form_stage: u32,
    pub last_tick_at: i64,
    pub offline_cap_hours: u32,
}

impl GameState {
    pub fn new_fresh_start(last_tick_at: i64) -> Self {
        let mut skills = HashMap::new();
        skills.insert(SkillId::scraping(), SkillState::new());

        Self {
            skills,
            active_skill: SkillId::idle(),
            tokens: 0,
            form_stage: 1,
            last_tick_at,
            offline_cap_hours: 8,
        }
    }

    pub fn new_scraping_start(last_tick_at: i64) -> Self {
        let mut state = Self::new_fresh_start(last_tick_at);
        state.active_skill = SkillId::scraping();
        state
    }

    pub fn has_active_skill(&self) -> bool {
        !self.active_skill.is_idle()
    }

    pub fn scraping(&self) -> &SkillState {
        self.skills
            .get(&SkillId::scraping())
            .expect("scraping skill should exist")
    }

    pub fn total_level(&self) -> u32 {
        self.skills.values().map(|skill| skill.level).sum()
    }
}
