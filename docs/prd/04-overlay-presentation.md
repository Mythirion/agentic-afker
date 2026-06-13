# PRD: Overlay Presentation

## Problem Statement

The Overlay is the primary interaction surface — the Agent window on the player's desktop. Players need to see their Agent performing its active Skill, wearing Equipment earned from Skill levels, transforming at Form Stage thresholds, and celebrating Milestones. Without compelling Overlay presentation, the companion fantasy fails even if the simulation is correct.

## Solution

A PixiJS rendering layer in the Overlay webview that displays a layered pixel art Agent (body + Equipment + Form Stage sprite set), animates the active Skill activity, shows brief Milestone popups, and (from Beta) a minimal progress HUD. Alpha uses placeholder art; Beta introduces AI-generated sprites per style guide; 1.0 refines with human cleanup.

## User Stories

1. As a player, I want to see my Agent animated in the Overlay, so that it feels alive on my desktop.
2. As a player, I want the Agent's animation to reflect my active Skill, so that I can see what it is doing (e.g. scraping, labelling, fine-tuning).
3. As a player, I want Equipment from my Skills to appear on the Agent, so that progression is visible at a glance.
4. As a player, I want Equipment to upgrade visually at Skill level milestones, so that levelling feels rewarding.
5. As a player, I want the Agent to transform at Form Stage unlocks, so that the dumb-to-useful arc is visible.
6. As a player, I want a brief popup when I level up, so that I get satisfying feedback without opening the Game Window.
7. As a player, I want a brief popup when I unlock a Form Stage, so that major milestones feel celebrated.
8. As a player, I want a brief popup when a Skill Upgrade becomes available, so that I know I have something to spend Tokens on.
9. As a player, I want Milestone popups to be visual only (click-through safe), so that they do not block my desktop.
10. As a player, I want an Interact Mode button to appear when Interact Mode is enabled, so that I can open the Game Window.
11. As a player, I want the Overlay to render at roughly 300×200 pixels, so that sprites are readable.
12. As a developer, I want Equipment rendered as layered sprites with z-ordering, so that Form Stage bodies and per-Skill Equipment compose without redrawing.
13. As a developer, I want the renderer to subscribe to simulation state snapshots, so that it is decoupled from the tick engine.
14. As a player (Beta), I want a minimal HUD showing active Skill and Token rate, so that I can glance at progress without opening the Game Window.
15. As a player (Beta), I want pixel art that feels clean but warm — not clinical, not generic AI slop, so that the companion has personality.

## Implementation Decisions

### Modules

- **AgentRenderer** — PixiJS application setup, sprite layer management, render loop. Deep module: owns the stage graph.
- **SpriteCatalog** — Maps (FormStage, Skill, EquipmentTier) to sprite assets. Loads placeholder rectangles in Alpha; real assets in Beta+.
- **AnimationController** — Plays Skill-specific animations (idle, working) on the Agent sprite.
- **EquipmentCompositor** — Layers Equipment sprites onto the Agent body at correct z-indices.
- **MilestonePresenter** — Receives Milestone events; plays popup animations (fade in/out, sparkle).
- **HudPresenter** (Beta) — Renders minimal progress bar and Token rate overlay.

### Layer model

```
Stage (bottom → top):
  1. Form Stage body sprite
  2. Equipment sprites (one per active Skill with unlocked Equipment)
  3. Activity animation overlay (per active Skill)
  4. Milestone popup layer
  5. HUD layer (Beta+)
  6. Interact Mode controls (button)
```

### Asset pipeline by phase

| Phase | Art |
|-------|-----|
| **Alpha** | Coloured placeholder rectangles per layer; distinct colours per Skill/Equipment tier |
| **Beta** | AI-generated pixel art with locked style guide (palette, outline weight, proportions) |
| **1.0.0** | AI-generated base + human cleanup pass; cosmetic DLC sprites optional |

### Communication

- Rust simulation emits state snapshots + Milestone events via Tauri events.
- AgentRenderer listens and updates sprites without polling.

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Placeholder layered sprites, Skill activity animation (simple), Form Stage swap, Milestone popups, Interact Mode button |
| **Beta** | AI-generated sprites, Equipment compositing, HUD (active Skill + Token rate), style guide enforcement |
| **1.0.0** | All Form Stage sprite sets, all Domain Equipment, polished Milestone effects, cosmetic variant support |

## Testing Decisions

- **EquipmentCompositor** and **SpriteCatalog** tested in isolation (given state → correct layer list). No PixiJS canvas needed — test the composition logic.
- **MilestonePresenter** tested: given Milestone event → correct popup queued, auto-dismisses.
- Visual rendering tested manually on Windows.
- Prior art: none (greenfield).

## Out of Scope

- Game Window UI (Game Window PRD)
- Desktop shell window management (Desktop Shell PRD)
- Simulation logic (Game Simulation PRD)
- Art generation tooling (noted in Further Notes)

## Further Notes

- ADR-0001: PixiJS in the Overlay webview, not a game engine.
- Modular Equipment layering is intentional — it reduces art volume and maps to AI-generated asset pipelines.
- Style guide document should be created before Beta art generation begins.
