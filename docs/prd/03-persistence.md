# PRD: Persistence

## Problem Statement

Players invest hours of idle progression in Agentic Afker. Losing that progress to a crash, reinstall, or machine change would be devastating. The game needs reliable local persistence from day one, with a path to manual backup and eventual Steam Cloud sync.

## Solution

A Rust persistence layer that serialises GameState to a local SQLite database, saves automatically after simulation ticks, supports manual export/import of save files, and is designed so Steam Cloud can wrap the same file format at 1.0.

## User Stories

1. As a player, I want my progress saved automatically, so that I never lose progress to a crash.
2. As a player, I want my save to include all Skill levels, Tokens, upgrades, active Skill, Form Stage, and offline cap, so that everything is restored on relaunch.
3. As a player, I want `last_tick_at` persisted, so that offline catch-up works correctly.
4. As a player, I want to export my save to a file, so that I can back it up manually.
5. As a player, I want to import a save from a file, so that I can restore a backup or move to a new machine.
6. As a player, I want import to validate the save before applying it, so that a corrupt file does not destroy my current progress.
7. As a developer, I want save/load to be a deep module with a simple interface, so that the simulation engine does not know about SQLite.
8. As a developer, I want save schema versioning, so that Beta/1.0 additions do not break Alpha saves.
9. As a developer, I want migration logic for schema upgrades, so that existing players are upgraded seamlessly.

## Implementation Decisions

### Modules

- **SaveRepository** — `load() -> GameState`, `save(state: GameState)`, `export(path)`, `import(path) -> GameState`. Deep module: sole owner of persistence I/O.
- **SchemaVersion** — Tracks current save format version; provides migration chain.
- **SaveMigrator** — Applies sequential migrations from version N to current.

### Interface

```
SaveRepository:
  - load() -> Result<GameState, SaveError>
  - save(state: &GameState) -> Result<(), SaveError>
  - export_to(path: &Path) -> Result<(), SaveError>
  - import_from(path: &Path) -> Result<GameState, SaveError>
```

### Storage

- SQLite single-file database in the app's data directory.
- Auto-save: debounced, triggered after simulation state changes (max 5s delay, immediate on quit).
- Export format: copy of the SQLite file (or JSON export for human readability — decision at implementation).

### Schema versioning

- Version 1 (Alpha): skills, tokens, upgrades, active_skill, form_stage, last_tick_at, offline_cap_hours.
- Future versions add fields via migrations, never in-place edits.

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | SQLite save/load, auto-save, `last_tick_at` for offline catch-up, schema v1 |
| **Beta** | Export/import, schema migrations for new fields (cross-Domain resources, Orchestration state) |
| **1.0.0** | Steam Cloud integration (wraps same save file), save conflict resolution (latest timestamp wins) |

## Testing Decisions

- **SaveRepository** tested with in-memory SQLite — round-trip: save state, load state, assert equality.
- **SaveMigrator** tested with fixture saves at each schema version — assert migrated state matches expected.
- Import validation tested with corrupt/truncated/invalid version files — assert error, original save untouched.
- Do not test SQLite internals; test the repository's external contract.
- Prior art: none (greenfield).

## Out of Scope

- Cloud sync auth (1.0 Steam Cloud)
- Multiple save slots / profiles
- Save encryption
- Simulation logic (Game Simulation PRD)

## Further Notes

- ADR-0001: SQLite chosen for structured queries and migration support over flat JSON.
- Export/import is Beta; Alpha relies on auto-save only. Design the format now so export is trivial later.
