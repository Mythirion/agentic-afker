# PRD: Orchestration

## Problem Statement

As Domains expand in Beta and 1.0, players train Skills across multiple aspects of AI. The game needs a capstone Skill — Orchestration — that consumes outputs from multiple Domains, unlocks mid-game, and scales through endgame. Without Orchestration, Domains feel like disconnected silos and there is no "your Agent is truly useful" moment.

## Solution

Orchestration is a Production Skill in its own Domain that unlocks mid-game (Beta) when the player has reached thresholds across multiple Domains. It consumes cross-Domain resources and produces high Token yields, scaling in complexity and reward through 1.0 endgame. It is the hub Skill that makes the Agent a generalist.

## User Stories

### Beta — Orchestration introduction

1. As a player, I want Orchestration to unlock when I have reached Skill thresholds in at least two Domains, so that I feel ready for cross-Domain play.
2. As a player, I want Orchestration to consume resources from multiple Domains, so that breadth of training is required.
3. As a player, I want Orchestration to produce Tokens at a high rate, so that the hub Skill is the best production target.
4. As a player, I want to see Orchestration requirements clearly in the Game Window, so that I know what Domains to develop.
5. As a player, I want Orchestration to have its own Equipment cosmetics, so that the Agent looks like a coordinator.
6. As a player, I want a Form Stage tied to Orchestration progress, so that the Agent's transformation reflects becoming a generalist.

### 1.0.0 — Orchestration endgame

7. As a player, I want Orchestration to have multiple tiers of recipes requiring increasingly diverse Domain outputs, so that endgame has depth.
8. As a player, I want late Orchestration tiers to require near-maxed Skills across Domains, so that 1.0 completionism is meaningful.
9. As a player, I want the final Orchestration tier to coincide with the final Form Stage, so that "fully useful Agent" is a clear endgame moment.
10. As a developer, I want Orchestration recipes defined in content config, so that tiers can be added without simulation rewrites.

## Implementation Decisions

### Modules

- **OrchestrationConfig** — Defines unlock thresholds, recipe tiers, input requirements, Token yields. Extends ProgressionConfig.
- **OrchestrationUnlockEvaluator** — Checks cross-Domain Skill level thresholds; emits unlock event. Deep module: encapsulates the multi-Domain gate logic.
- **OrchestrationRecipe** — Per-tier definition: required resources from each Domain, Token output, minimum Skill levels.

### Unlock conditions (Beta — tentative)

- Orchestration unlocks when: Total Level ≥ 50 AND Scraping ≥ 20 AND Prompt Crafting ≥ 15 (cross-Domain example; exact values tuned in playtesting).

### Recipe tiers (conceptual)

```
Tier 1 (Beta):  Labelled data + Crafted prompts → Tokens
Tier 2 (1.0):   + Summarised reports → more Tokens
Tier 3 (1.0):   + Published content → highest Tokens
```

### Relationship to Form Stages

- Form Stage 3 (Beta): unlocked at Orchestration unlock.
- Form Stage 4 (1.0): unlocked at Orchestration Tier 3 or max Total Level.

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Not in scope — Data Pipeline is self-contained |
| **Beta** | Orchestration unlocks mid-game, Tier 1 recipe, Equipment, Form Stage 3, Game Window progress panel |
| **1.0.0** | Tiers 2–3, final Form Stage, endgame scaling, full cross-Domain recipe graph |

## Testing Decisions

- **OrchestrationUnlockEvaluator** tested: given Skill levels across Domains → correct locked/unlocked state.
- **OrchestrationRecipe** processing tested: given resources → correct Token output, resources consumed.
- Tier gating: player below tier requirements cannot activate that tier.
- Prior art: none (greenfield).

## Out of Scope

- Alpha content (Skill Content PRD — Orchestration does not exist in Alpha)
- Base Domain Skills (Skill Content PRD)
- Token economy tuning (Progression & Economy PRD)
- Orchestration UI beyond progress panel (Game Window PRD)

## Further Notes

- Orchestration is deliberately absent from Alpha — the Alpha vertical slice proves one Domain without hub complexity.
- The "useful Agent" narrative arc: dumb blob (Stage 1) → competent specialist (Stage 2) → cross-Domain generalist (Stage 3) → fully orchestrated assistant (Stage 4).
