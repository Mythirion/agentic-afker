# PRD: Distribution & Release

## Problem Statement

Agentic Afker is a solo-developed desktop game targeting Steam on Windows and macOS. To ship Alpha, Beta, and 1.0, the project needs build pipelines, installers, platform packaging, and (at 1.0) monetisation and Steam integration. Without a distribution plan, the game stays dev-only.

## Solution

A release pipeline that produces installable builds for Windows and macOS from headless Linux CI, with phased delivery: Alpha as free dev/test builds, Beta as wider playtest distribution, and 1.0 as a Steam release (~£5 premium or free + cosmetic DLC — decided after playtesting).

## User Stories

### Alpha

1. As a developer, I want to build a Windows installer from Linux, so that I can test on my Windows machine.
2. As a playtester, I want to install Alpha without technical setup, so that I can give feedback.
3. As a developer, I want Alpha builds versioned (e.g. 0.1.0-alpha), so that playtesters know which build they have.

### Beta

4. As a developer, I want macOS builds, so that Mac playtesters can participate.
5. As a playtester, I want auto-update or clear update instructions, so that I stay on the latest Beta.
6. As a developer, I want crash logs or error reporting, so that I can fix Beta issues.

### 1.0.0

7. As a player, I want to buy the game on Steam for ~£5, so that I can support the developer.
8. As a player, I want Steam Cloud save sync, so that my progress follows me across machines.
9. As a player, I want the game to launch from Steam without extra setup, so that installation is seamless.
10. As a developer, I want Steam achievements (optional), so that completionists have goals.
11. As a developer, I want cosmetic DLC support if playtesting favours F2P, so that monetisation is flexible.
12. As a developer, I want CI to produce signed Windows and macOS builds, so that releases are reproducible.

## Implementation Decisions

### Modules

- **BuildPipeline** — CI config (GitHub Actions) for cross-platform Tauri builds. Deep module: single source for release automation.
- **ReleaseVersioning** — Semver with pre-release tags: `0.1.0-alpha`, `0.5.0-beta`, `1.0.0`.
- **SteamIntegration** (1.0) — Steamworks SDK wrapper for Cloud, achievements, DRM-free launch.
- **UpdateChecker** (Beta) — Optional in-app version check pointing to latest release URL.

### Platform targets

| Phase | Windows | macOS | Linux |
|-------|---------|-------|-------|
| Alpha | ✅ test | — | dev only |
| Beta | ✅ | ✅ | — |
| 1.0.0 | ✅ Steam | ✅ Steam | deferred |

### Monetisation (decided at playtesting)

| Model | When |
|-------|------|
| Free Alpha/Beta | Always |
| ~£5 premium on Steam | 1.0 default |
| Free + cosmetic DLC | 1.0 alternative if playtesting suggests |

### Steam Cloud

- Wraps the same SQLite save file from Persistence PRD.
- Conflict resolution: latest `last_tick_at` wins.

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Windows build from Linux CI, manual distribution to playtesters, semver alpha tags |
| **Beta** | macOS builds, itch.io or GitHub Releases distribution, optional update checker, basic crash logging |
| **1.0.0** | Steam store page, Steam Cloud, installer signing, monetisation implementation, cosmetic DLC framework (if chosen) |

## Testing Decisions

- **BuildPipeline** tested by CI itself — build succeeds, artifact produced, app launches on target OS.
- Steam Cloud: manual test — save on machine A, load on machine B.
- No unit tests for Steam SDK wrapper; integration tested manually.
- Prior art: none (greenfield).

## Out of Scope

- Linux desktop release
- Mobile
- Multiplayer / online features
- In-app purchases infrastructure (unless cosmetic DLC model chosen at 1.0)
- Localisation (English only for 1.0)

## Further Notes

- Solo project, AI-developed / human-designed. Distribution overhead should be minimised until Beta.
- Steam App ID and Steamworks setup is a human task (ready-for-human) at 1.0.
- Alpha does not need an installer — a zip with the Tauri bundle is sufficient for initial playtesting.
