# PRD: Game Window

## Problem Statement

The Overlay is intentionally minimal — players need a dedicated Game Window to switch active Skills, view their Skill tree, purchase Skill Upgrades and Infrastructure, and understand what to work toward next. Without a management UI, the idle loop is opaque and upgrade spending is impossible.

## Solution

An HTML/CSS application surface (in a separate Tauri webview) that presents the player's Skills, Domains, upgrade shop, and active Skill selector. The Game Window reads simulation state from Rust and sends player actions (switch Skill, purchase upgrade) back as commands. Clean, Melvor-inspired layout — not clinical, not generic.

## User Stories

1. As a player, I want to see all my Skills and their current levels, so that I know my progression status.
2. As a player, I want to see which Skill is currently active, so that I know what my Agent is doing.
3. As a player, I want to switch my active Skill, so that I can train a different capability.
4. As a player, I want to see which Skills are locked and their prerequisites, so that I know what to work toward.
5. As a player, I want to see my Token balance, so that I know what I can afford.
6. As a player, I want to purchase Skill Upgrades when I reach level milestones (10, 20, 30…), so that I can boost my active Skills.
7. As a player, I want to see available Skill Upgrades and their costs, so that I can plan my spending.
8. As a player, I want to purchase Infrastructure upgrades when I meet Total Level thresholds, so that I can invest in global buffs.
9. As a player, I want to see my Total Level and current Form Stage, so that I understand my overall progress.
10. As a player, I want to see the gathering → production chain for my Domain, so that I understand resource flow.
11. As a player, I want the Game Window to open alongside the Overlay, so that my Agent stays visible while I manage Skills.
12. As a player, I want the UI to feel clean and readable, so that managing Skills is pleasant not tedious.
13. As a player (Beta), I want to see multiple Domains and cross-Domain dependencies, so that I understand how Skill branches connect.
14. As a player (Beta), I want to see Orchestration progress and requirements, so that I know how close I am to the hub Skill.
15. As a player (1.0), I want to purchase Offline Cap Infrastructure upgrades, so that I can extend my offline progress window.
16. As a developer, I want player actions sent as commands to the simulation engine, so that the UI cannot cheat state.

## Implementation Decisions

### Modules

- **GameWindowApp** — Frontend application shell (routing, layout, theme). Thin; delegates to views.
- **SkillTreeView** — Renders Skills grouped by Domain, shows levels, lock states, prerequisites, active indicator.
- **UpgradeShopView** — Lists available Skill Upgrades and Infrastructure purchases with costs and effects.
- **ResourceFlowView** — Visualises gathering → production chain for the active Domain (Alpha: Data Pipeline diagram).
- **CommandBridge** — Sends player actions to Rust (`set_active_skill`, `purchase_upgrade`) and receives state snapshots. Deep module: sole IPC boundary for player intent.
- **GameStateStore** — Frontend reactive store mirroring simulation snapshots.

### Player commands (conceptual)

```
Commands:
  - set_active_skill(skill_id)
  - purchase_skill_upgrade(skill_id, upgrade_tier)
  - purchase_infrastructure(infrastructure_id)

Events (from Rust):
  - state_snapshot(GameState)
  - purchase_result(success | error)
```

### Layout (Alpha)

- Header: Token balance, Total Level, Form Stage
- Main: Skill list for Data Pipeline Domain (Scraping, Labelling, Fine-Tuning) with active selector
- Sidebar: Available Skill Upgrades and Infrastructure
- Footer: Resource flow diagram

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Skill list + active selector, upgrade shop (Skill Upgrades at milestones), Token display, Data Pipeline resource flow, Form Stage / Total Level display |
| **Beta** | Multi-Domain navigation, cross-Domain prerequisite display, Orchestration progress panel, Offline Cap upgrade in Infrastructure shop |
| **1.0.0** | All Domains, full Infrastructure tree, polished layout, settings (hotkey config, export/import triggers) |

## Testing Decisions

- **CommandBridge** tested: given UI action → correct command emitted; given state snapshot → store updated.
- View components tested with rendered output given mock state (external behaviour: "locked Skill shows prerequisite text").
- Do not test CSS layout details; test that correct data is presented.
- E2E manual testing for full purchase flow on Windows.
- Prior art: none (greenfield).

## Out of Scope

- Overlay rendering (Overlay Presentation PRD)
- Simulation/upgrade effect logic (Progression & Economy PRD)
- Save export/import UI (Persistence PRD — Beta adds triggers here)
- Monetisation / store (Distribution PRD)

## Further Notes

- ADR-0001: HTML/CSS for Game Window, PixiJS only for Overlay.
- Melvor Idle is the UX reference — sidebar skill list, central detail panel, shop tab.
- Alpha UI can be functional-first; visual polish is Beta.
