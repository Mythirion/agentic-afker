# PRD: Progression & Economy

## Problem Statement

Agentic Afker's long-term engagement depends on a well-tuned progression system: Skills that level with meaningful milestones, Tokens that create spending tension, Infrastructure that rewards Total Level investment, and Form Stages that mark the dumb-to-useful arc. Without explicit rules for how these systems interact, the idle loop will feel flat or exploitable.

## Solution

A declarative progression configuration layer that defines XP curves, Token yields, Skill Upgrade tiers, Infrastructure gates, Form Stage thresholds, and Equipment milestones — consumed by the Game Simulation engine and presented by the Game Window. Rules are data-driven so new Skills and Domains can be added without rewriting simulation logic.

## User Stories

1. As a player, I want Skills to level from 1 upward with increasing XP requirements, so that early progress is fast and late progress is earned.
2. As a player, I want Total Level to be the sum of all my Skill levels, so that breadth of training is rewarded.
3. As a player, I want Form Stage 2 unlocked at a Total Level threshold, so that the Agent's transformation feels earned.
4. As a player, I want Equipment cosmetic tiers at Skill levels 10, 20, 30, etc., so that within-Skill progression is visible.
5. As a player, I want Skill Upgrades available at the same milestones (10, 20, 30…), so that I have something to spend Tokens on when I level.
6. As a player, I want Skill Upgrades to improve XP rate or Token yield for that Skill, so that spending feels impactful.
7. As a player, I want Infrastructure upgrades gated by Total Level thresholds, so that global investment has clear goals.
8. As a player, I want Infrastructure to provide global buffs (e.g. expanded context window = +X% all production), so that Total Level breadth matters.
9. As a player, I want an Offline Cap Infrastructure upgrade purchasable with Tokens, so that I can extend offline progress from 8 hours toward 3 days.
10. As a player, I want to not be able to purchase an upgrade I cannot afford or have not unlocked, so that the economy is fair.
11. As a developer, I want progression rules defined as data (not hardcoded), so that adding Beta/1.0 Skills does not require simulation rewrites.
12. As a developer, I want XP curves and Token yields to be tunable constants, so that balance passes are configuration changes.

## Implementation Decisions

### Modules

- **ProgressionConfig** — Data definitions for all Skills, Upgrades, Infrastructure, Form Stages, XP curves. Deep module: single source of truth for progression rules.
- **XpCurve** — `xp_for_level(level) -> u64` and inverse. Standard idle curve (e.g. Melvor-style polynomial/exponential).
- **UpgradeRegistry** — Skill Upgrade definitions: skill, milestone level, cost, effect.
- **InfrastructureRegistry** — Infrastructure definitions: total_level_gate, cost, effect, max rank.
- **FormStageRegistry** — Form Stage definitions: total_level_gate, sprite set id.
- **EquipmentRegistry** — Equipment tier definitions: skill, milestone level, sprite id.
- **EffectResolver** — Given purchased upgrades + infrastructure, computes effective modifiers on XP rate and Token yield.

### Alpha configuration

| Element | Alpha values (initial tuning) |
|---------|----------------------------|
| Skills | Scraping, Labelling, Fine-Tuning |
| Form Stages | Stage 1 (start), Stage 2 (Total Level TBD — suggest 30) |
| Skill Upgrade milestones | 10, 20, 30 |
| Infrastructure | 1–2 entries (e.g. "Expanded Context Window" at Total Level 20) |
| Offline Cap | 8h base, 1 upgrade step toward 3d max |
| XP curve | Melvor-inspired; exact constants tuned in playtesting |

### Effect types

```
Effect:
  - XpMultiplier(skill_id, factor)
  - TokenYieldMultiplier(skill_id, factor)
  - GlobalProductionMultiplier(factor)
  - OfflineCapHours(hours)
```

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | 3 Skills, 2 Form Stages, Skill Upgrades at 10/20/30, 1–2 Infrastructure entries, Equipment tiers, XP curve, Token yields |
| **Beta** | Expanded registries for 8–10 Skills, 3–4 Form Stages, cross-Domain gate definitions, Orchestration prerequisites |
| **1.0.0** | Full registry for 15+ Skills, all Form Stages, full Infrastructure tree, Offline Cap upgrade chain, balance pass |

## Testing Decisions

- **XpCurve**, **EffectResolver**, **UpgradeRegistry** tested as pure functions.
- Given purchased upgrades + skill state → correct effective XP rate and Token yield (external behaviour).
- Form Stage unlock: given Total Level → correct stage id.
- Equipment availability: given Skill level → correct Equipment tier.
- Prior art: none (greenfield).

## Out of Scope

- Skill narrative/content flavour text (Skill Content PRD)
- Orchestration-specific rules (Orchestration PRD)
- UI presentation of upgrades (Game Window PRD)
- Simulation tick processing (Game Simulation PRD)

## Further Notes

- Exact XP curve constants and Total Level thresholds for Form Stages are tuning decisions — start with Melvor-like curves and adjust in Alpha playtesting.
- All registries should be loadable from config files (TOML/JSON) to support balance iteration without recompilation.
