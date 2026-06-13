# PRDs — Agentic Afker

Component-level PRDs for Alpha, Beta, and 1.0.0. Each PRD is intended to become a parent issue, broken into implementation issues later.

## Deep modules

| # | Component | Deep modules | Primary tests |
|---|-----------|-------------|---------------|
| 01 | [Desktop Shell](./01-desktop-shell.md) | WindowManager, TrayController, HotkeyRegistry, AppState | WindowManager, AppState (Rust unit) |
| 02 | [Game Simulation](./02-game-simulation.md) | TickEngine, SkillProcessor, ProductionChain, OfflineCatchUp, MilestoneDetector | All pure-function modules (Rust unit) |
| 03 | [Persistence](./03-persistence.md) | SaveRepository, SchemaVersion, SaveMigrator | SaveRepository round-trip, SaveMigrator |
| 04 | [Overlay Presentation](./04-overlay-presentation.md) | AgentRenderer, EquipmentCompositor, MilestonePresenter | EquipmentCompositor, SpriteCatalog composition logic |
| 05 | [Game Window](./05-game-window.md) | CommandBridge, GameStateStore | CommandBridge, view data assertions |
| 06 | [Progression & Economy](./06-progression-economy.md) | ProgressionConfig, EffectResolver, XpCurve | EffectResolver, XpCurve, registries |
| 07 | [Skill Content & Domains](./07-skill-content-domains.md) | DomainRegistry, SkillDefinition, ContentLoader | ContentLoader, prerequisite validation |
| 08 | [Orchestration](./08-orchestration.md) | OrchestrationUnlockEvaluator, OrchestrationRecipe | Unlock evaluator, recipe processing |
| 09 | [Distribution & Release](./09-distribution-release.md) | BuildPipeline | CI build verification |

## Phase dependency graph

```
Alpha
├── 01 Desktop Shell
├── 02 Game Simulation
├── 03 Persistence (auto-save only)
├── 04 Overlay Presentation (placeholders)
├── 05 Game Window
├── 06 Progression & Economy (Data Pipeline config)
├── 07 Skill Content (Data Pipeline Domain)
└── 09 Distribution (Windows build)

Beta (requires Alpha)
├── 03 Persistence (+ export/import)
├── 04 Overlay Presentation (+ AI art, HUD)
├── 05 Game Window (+ multi-Domain UI)
├── 06 Progression & Economy (expanded)
├── 07 Skill Content (+ 2 Domains, cross-Domain)
├── 08 Orchestration (Tier 1)
└── 09 Distribution (+ macOS, itch/Releases)

1.0.0 (requires Beta)
├── 04 Overlay Presentation (polished art)
├── 06 Progression & Economy (full tree)
├── 07 Skill Content (all Domains)
├── 08 Orchestration (Tiers 2–3, endgame)
├── 03 Persistence (+ Steam Cloud)
└── 09 Distribution (Steam, monetisation)
```

## Suggested issue breakdown order (Alpha)

1. Desktop Shell — nothing works without windows
2. Game Simulation + Persistence — core loop
3. Progression & Economy + Skill Content — config for Data Pipeline
4. Overlay Presentation — see the Agent
5. Game Window — manage Skills
6. Distribution — ship to playtesters
