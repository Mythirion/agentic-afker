use crate::simulation::{GameState, SkillId, SkillState};
use rusqlite::{params, Connection};
use std::fmt;
use std::path::Path;

pub const SCHEMA_VERSION: i32 = 1;

#[derive(Debug)]
pub enum SaveError {
    Database(rusqlite::Error),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(f, "database error: {error}"),
        }
    }
}

impl From<rusqlite::Error> for SaveError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

pub struct SaveRepository {
    connection: Connection,
}

impl SaveRepository {
    pub fn open(path: &Path) -> Result<Self, SaveError> {
        let connection = Connection::open(path)?;
        let repo = Self { connection };
        repo.init_schema()?;
        Ok(repo)
    }

    pub fn open_in_memory() -> Result<Self, SaveError> {
        let connection = Connection::open_in_memory()?;
        let repo = Self { connection };
        repo.init_schema()?;
        Ok(repo)
    }

    fn init_schema(&self) -> Result<(), SaveError> {
        self.connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS save_meta (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                schema_version INTEGER NOT NULL,
                last_tick_at INTEGER NOT NULL,
                active_skill TEXT NOT NULL,
                tokens INTEGER NOT NULL,
                form_stage INTEGER NOT NULL,
                offline_cap_hours INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS skills (
                skill_id TEXT PRIMARY KEY,
                level INTEGER NOT NULL,
                xp INTEGER NOT NULL,
                resources INTEGER NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn load(&self) -> Result<Option<GameState>, SaveError> {
        let mut meta = self
            .connection
            .prepare(
                "SELECT schema_version, last_tick_at, active_skill, tokens, form_stage, offline_cap_hours
                 FROM save_meta WHERE id = 1",
            )?;

        let meta_row = match meta.query_row([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, u64>(3)?,
                row.get::<_, u32>(4)?,
                row.get::<_, u32>(5)?,
            ))
        }) {
            Ok(row) => row,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(error) => return Err(error.into()),
        };

        let (schema_version, last_tick_at, active_skill, tokens, form_stage, offline_cap_hours) =
            meta_row;

        if schema_version != SCHEMA_VERSION {
            return Err(SaveError::Database(rusqlite::Error::InvalidColumnType(
                0,
                "schema_version".to_string(),
                rusqlite::types::Type::Integer,
            )));
        }

        let mut skills_stmt = self
            .connection
            .prepare("SELECT skill_id, level, xp, resources FROM skills")?;
        let skill_rows = skills_stmt.query_map([], |row| {
            Ok((
                SkillId(row.get::<_, String>(0)?),
                SkillState {
                    level: row.get(1)?,
                    xp: row.get(2)?,
                    resources: row.get(3)?,
                },
            ))
        })?;

        let mut skills = std::collections::HashMap::new();
        for row in skill_rows {
            let (skill_id, skill_state) = row?;
            skills.insert(skill_id, skill_state);
        }

        Ok(Some(GameState {
            skills,
            active_skill: SkillId(active_skill),
            tokens,
            form_stage,
            last_tick_at,
            offline_cap_hours,
        }))
    }

    pub fn save(&self, state: &GameState) -> Result<(), SaveError> {
        let tx = self.connection.unchecked_transaction()?;

        tx.execute(
            "INSERT INTO save_meta (id, schema_version, last_tick_at, active_skill, tokens, form_stage, offline_cap_hours)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
               schema_version = excluded.schema_version,
               last_tick_at = excluded.last_tick_at,
               active_skill = excluded.active_skill,
               tokens = excluded.tokens,
               form_stage = excluded.form_stage,
               offline_cap_hours = excluded.offline_cap_hours",
            params![
                SCHEMA_VERSION,
                state.last_tick_at,
                state.active_skill.0,
                state.tokens,
                state.form_stage,
                state.offline_cap_hours,
            ],
        )?;

        tx.execute("DELETE FROM skills", [])?;

        for (skill_id, skill) in &state.skills {
            tx.execute(
                "INSERT INTO skills (skill_id, level, xp, resources) VALUES (?1, ?2, ?3, ?4)",
                params![skill_id.0, skill.level, skill.xp, skill.resources],
            )?;
        }

        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::{advance, GameState};

    #[test]
    fn round_trip_persists_scraping_progress_and_last_tick_at() {
        let repo = SaveRepository::open_in_memory().expect("in-memory db");
        let mut state = GameState::new_scraping_start(1_000);
        advance(&mut state, 2_500);

        repo.save(&state).expect("save");
        let loaded = repo.load().expect("load").expect("saved state");

        assert_eq!(loaded.scraping().xp, state.scraping().xp);
        assert_eq!(loaded.scraping().level, state.scraping().level);
        assert_eq!(loaded.last_tick_at, 3_500);
        assert_eq!(loaded.active_skill, SkillId::scraping());
    }

    #[test]
    fn load_returns_none_for_empty_database() {
        let repo = SaveRepository::open_in_memory().expect("in-memory db");
        assert!(repo.load().expect("load").is_none());
    }
}
