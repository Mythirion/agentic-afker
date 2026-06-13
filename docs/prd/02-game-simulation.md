# PRD: Game Simulation

## Problem Statement

Agentic Afker is an idle game — progression must continue while the Overlay is visible or the app is trayed, and resume with offline catch-up when the app is relaunched. Players need Skills to gain XP, Production Skills to generate Tokens, and the simulation to respect unlock gates and upgrade modifiers. Without a reliable tick engine, there is no game.

## Solution

A Rust game simulation engine that runs a deterministic tick loop independent of the webview render frame rate. The engine processes active Skill XP gain, Token generation from Production Skills, resource consumption along production chains, and offline catch-up on launch (capped by Offline Cap). State changes are emitted to the frontend and persisted after each tick batch.

## User Stories

1. As a player, I want my active Skill to gain XP continuously while the app is open (Overlay or Tray), so that I make progress without micromanaging.
2. As a player, I want Production Skills to generate Tokens passively while active, so that I earn currency to spend on upgrades.
3. As a player, I want Gathering Skills to produce raw inputs for Production Skills, so that the gathering → production loop works.
4. As a player, I want to switch my active Skill, so that I can train different capabilities.
5. As a player, I want only one Skill to be active at a time (Alpha), so that progression is simple and focused.
6. As a player, I want XP gain rate to be affected by my Skill Upgrades, so that spending Tokens feels impactful.
7. As a player, I want Token yield to be affected by Skill Upgrades and Infrastructure, so that global investments pay off.
8. As a player, I want Fine-Tuning to require labelled data from Labelling, so that the production chain has meaningful gates.
9. As a player, I want Labelling to consume scraped data from Scraping, so that Gathering feeds Production.
10. As a player, I want to be unable to activate a Skill I have not unlocked, so that progression order is enforced.
11. As a player, I want the simulation to keep ticking when minimised to Tray, so that I can work without the Overlay visible.
12. As a player, I want offline catch-up when I relaunch the app after closing it, so that short absences are not wasted.
13. As a player, I want offline catch-up capped at my Offline Cap (8 hours initially), so that the game rewards regular engagement without infinite accumulation.
14. As a player, I want fractional XP and Tokens to accumulate precisely, so that I do not lose progress to rounding.
15. As a developer, I want the tick loop to run in Rust, not JavaScript, so that timing is reliable when the webview is backgrounded.
16. As a developer, I want simulation logic to be pure and deterministic given a state + elapsed time, so that offline catch-up replays identically to live ticking.

## Implementation Decisions

### Modules

- **TickEngine** — Owns the game loop timer, calculates `delta_ms` since last tick, batches ticks, and invokes processors. Deep module: single entry point `advance(state, elapsed) -> state`.
- **SkillProcessor** — Given active Skill and elapsed time, computes XP gained and resources produced/consumed. Applies Skill Upgrade and Infrastructure modifiers.
- **ProductionChain** — Defines Domain recipes: which inputs a Production Skill requires and what outputs it produces (XP, Tokens, intermediate resources).
- **UnlockEvaluator** — Determines which Skills are available based on prerequisite Skill levels and (Beta+) cross-Domain resource availability.
- **OfflineCatchUp** — On launch, reads `last_tick_at` from Save, computes elapsed time capped by Offline Cap, replays ticks via TickEngine.
- **MilestoneDetector** — Compares state before/after tick batch; emits Milestone events (level up, Form Stage unlock, Skill Upgrade available).

### Core state shape (from design)

```
GameState:
  skills: Map<SkillId, SkillState>       // level, xp, resources held
  active_skill: SkillId
  tokens: Decimal
  upgrades: Set<UpgradeId>               // purchased Skill Upgrades + Infrastructure
  form_stage: FormStageId
  last_tick_at: Timestamp
  offline_cap_hours: u32                  // default 8, max 72
```

### Tick rate

- Simulation tick: 100ms internal resolution (configurable).
- Batched for performance; frontend receives state snapshots at ~1s intervals or on Milestone.

### Alpha skill chain (Data Pipeline Domain)

| Skill | Type | Input | Output |
|-------|------|-------|--------|
| Scraping | Gathering | — | Raw data |
| Labelling | Production | Raw data | Labelled data, Tokens |
| Fine-Tuning | Production | Labelled data | Tokens (higher yield) |

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Tick loop, single active Skill, Data Pipeline chain, XP + Token generation, offline catch-up (8h cap), Milestone detection |
| **Beta** | Cross-Domain inputs, Orchestration Skill processing, multiple resource types, Skill Upgrade/Infrastructure modifiers fully wired |
| **1.0.0** | Full Domain recipes, Orchestration endgame scaling, Offline Cap upgrades via Infrastructure |

## Testing Decisions

- **TickEngine**, **SkillProcessor**, **ProductionChain**, **OfflineCatchUp**, and **UnlockEvaluator** are all pure functions — heavily unit tested.
- Tests assert external behaviour: given state S and elapsed T, expect state S' with specific levels, tokens, and resources. No testing of internal tick loop timers.
- Property-based tests for offline catch-up: `advance(live, T) == advance(offline_replay, T)` for same initial state.
- MilestoneDetector tested by feeding before/after states and asserting emitted events.
- Prior art: none (greenfield).

## Out of Scope

- UI rendering of simulation state (Overlay Presentation, Game Window PRDs)
- Save file I/O (Persistence PRD)
- Desktop shell lifecycle (Desktop Shell PRD)
- Skill content definitions beyond Data Pipeline (Skill Content PRD)
- Orchestration mechanics detail (Orchestration PRD)

## Further Notes

- ADR-0001: simulation lives in Rust, not the webview.
- Alpha validates one complete gathering → production chain before any content expansion.
