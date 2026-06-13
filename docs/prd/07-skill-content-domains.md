# PRD: Skill Content & Domains

## Problem Statement

Agentic Afker's theme — training an Agent through the AI boom — needs concrete Skills organised into Domains that each represent a different aspect of AI. Alpha must ship one complete Domain (Data Pipeline) to prove the loop; Beta and 1.0 expand with new Domains and cross-Domain connections. Without structured content definitions, the simulation has nothing to process and the theme is invisible.

## Solution

A content definition system for Domains and their Skills — each with type (Gathering/Production), narrative flavour, resource inputs/outputs, unlock prerequisites, and Equipment themes. Alpha ships the Data Pipeline Domain; Beta adds 1–2 new Domains with cross-Domain inputs; 1.0 completes all planned Domains.

## User Stories

### Alpha — Data Pipeline Domain

1. As a player, I want a Scraping Skill (Gathering), so that my Agent collects raw data as its first activity.
2. As a player, I want Scraping to be available from the start, so that I can begin progressing immediately.
3. As a player, I want a Labelling Skill (Production) that consumes scraped data, so that I experience the gathering → production loop.
4. As a player, I want Labelling to unlock at a Scraping level threshold, so that I must train Gathering first.
5. As a player, I want Labelling to produce Tokens, so that the production economy begins.
6. As a player, I want a Fine-Tuning Skill (Production) that consumes labelled data, so that the chain completes.
7. As a player, I want Fine-Tuning to unlock at a Labelling level threshold, so that the chain has depth.
8. As a player, I want Fine-Tuning to produce Tokens at a higher rate than Labelling, so that advancing the chain feels rewarding.
9. As a player, I want each Skill to have AI-boom-themed Equipment cosmetics, so that the theme is visible (placeholder art in Alpha).
10. As a player, I want the Data Pipeline Domain to be self-contained in Alpha, so that I can experience a complete loop without waiting for more content.

### Beta — Domain expansion

11. As a player, I want a Prompt Economy Domain (e.g. Prompt Crafting → Response Generation), so that I can explore another aspect of AI.
12. As a player, I want an Office Assistant Domain (e.g. Research → Summarisation → Report Writing), so that the Agent feels like a knowledge worker.
13. As a player, I want Production Skills in later Domains to require outputs from earlier Domains, so that Domains feel interconnected.
14. As a player, I want to see cross-Domain prerequisites in the Game Window, so that I understand what resources I need.
15. As a developer, I want new Domains added via content config without simulation code changes, so that content expansion is fast.

### 1.0.0 — Full content

16. As a player, I want a Content Creator Domain (e.g. Writing → Design → Publishing), so that creative AI aspects are represented.
17. As a player, I want 15+ Skills across all Domains, so that the game has Melvor-scale depth.
18. As a player, I want each Domain to have a distinct visual Equipment theme, so that Domains are distinguishable on the Agent.
19. As a player, I want all gathering → production chains to eventually feed toward Orchestration, so that the endgame has a clear direction.

## Implementation Decisions

### Modules

- **DomainRegistry** — Defines Domains and their metadata (name, description, theme, unlock conditions). Deep module.
- **SkillDefinition** — Per-Skill config: id, domain, type (Gathering/Production), inputs, outputs, unlock prereqs, Equipment theme.
- **ContentLoader** — Loads Domain/Skill definitions from config files at startup.

### Alpha: Data Pipeline Domain

| Skill | Type | Unlock | Inputs | Outputs |
|-------|------|--------|--------|---------|
| Scraping | Gathering | Start | — | Raw data |
| Labelling | Production | Scraping Lv 5 | Raw data | Labelled data, Tokens |
| Fine-Tuning | Production | Labelling Lv 10 | Labelled data | Tokens (×2 yield) |

### Beta: Planned Domains (tentative)

| Domain | Example chain | Cross-Domain input |
|--------|--------------|-------------------|
| Prompt Economy | Prompt Crafting → Response Generation | Fine-Tuned model from Data Pipeline |
| Office Assistant | Research → Summarisation → Report Writing | Generated responses from Prompt Economy |

### 1.0.0: Planned Domains (tentative)

| Domain | Example chain |
|--------|--------------|
| Content Creator | Writing → Design → Publishing |

### Content file structure (conceptual)

```
domains/
  data-pipeline.toml
  prompt-economy.toml      # Beta
  office-assistant.toml    # Beta
  content-creator.toml     # 1.0
```

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Data Pipeline Domain (3 Skills), self-contained, placeholder Equipment themes |
| **Beta** | + Prompt Economy, + Office Assistant (or subset), cross-Domain input definitions, AI-generated Equipment art |
| **1.0.0** | + Content Creator, all Domains complete, all Equipment themes, full cross-Domain graph |

## Testing Decisions

- **DomainRegistry** and **ContentLoader** tested: load config → correct Skill count, types, prerequisites, inputs/outputs.
- Cross-Domain prerequisite validation: no circular dependencies, all referenced Skills exist.
- Prior art: none (greenfield).

## Out of Scope

- Orchestration Skill mechanics (Orchestration PRD)
- Progression curves and upgrade costs (Progression & Economy PRD)
- Art asset creation (Overlay Presentation PRD)
- Simulation tick processing (Game Simulation PRD)

## Further Notes

- Exact level thresholds for unlocks are tuning decisions — values above are starting points.
- Domain names and Skill names should use AI-boom vocabulary, not fantasy (per CONTEXT.md).
- Beta Domain selection may be trimmed to 2 Domains if 3 is too much content — Prompt Economy is the strongest second Domain thematically.
