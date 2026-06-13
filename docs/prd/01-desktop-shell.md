# PRD: Desktop Shell

## Problem Statement

Agentic Afker is a desktop companion idle game. Players need a small always-on-top Overlay showing their Agent, the ability to minimise to the Tray without stopping progression, and a separate Game Window for skill management — all without blocking normal desktop work. Without a purpose-built desktop shell, the core companion fantasy cannot exist.

## Solution

A Tauri 2 desktop shell (Rust) that manages three application states — Overlay, Tray, and Game Window — with correct window flags for transparency, always-on-top, and click-through. A global hotkey toggles Interact Mode on the Overlay; a button (visible in Interact Mode) and tray menu entry open the Game Window. The Overlay remains visible while the Game Window is open.

## User Stories

1. As a player, I want the Agent to appear in a small always-on-top window on my desktop, so that I can see my companion while working.
2. As a player, I want the Overlay to be click-through by default, so that it does not interfere with my other applications.
3. As a player, I want to press a hotkey to toggle Interact Mode, so that I can temporarily click the Overlay when I want to interact with my Agent.
4. As a player, I want a button on the Overlay (visible in Interact Mode) to open the Game Window, so that I can manage Skills without memorising shortcuts.
5. As a player, I want to open the Game Window from the tray menu, so that I can access skill management even if I have not enabled Interact Mode.
6. As a player, I want the Overlay to stay visible when the Game Window is open, so that my Agent remains on the desktop while I browse Skills.
7. As a player, I want to minimise the app to the system tray, so that the Overlay disappears but progression continues.
8. As a player, I want to restore the Overlay from the tray icon, so that I can bring my companion back quickly.
9. As a player, I want the Overlay to be roughly 300×200 pixels, so that the Agent and Milestone popups are readable without dominating my screen.
10. As a player on Windows, I want the Overlay to work correctly with always-on-top and transparency, so that the companion feels native to my desktop.
11. As a player on macOS, I want the Overlay to work via a menu bar extra and compatible window flags, so that I can use the companion on my Mac.
12. As a developer, I want overlay and game window to be separate Tauri windows sharing state, so that each can be configured independently.
13. As a developer, I want global hotkey registration handled in Rust, so that Interact Mode works regardless of which window is focused.
14. As a developer, I want to build Windows targets from headless Linux, so that I can develop in my preferred environment.

## Implementation Decisions

### Modules

- **WindowManager** — Creates, configures, shows, hides, and destroys Overlay and Game Window instances. Owns window flags (transparent, always-on-top, click-through, decorations). Deep module: encapsulates all platform-specific window behaviour behind a small interface (`show_overlay`, `hide_overlay`, `open_game_window`, `set_click_through(enabled)`).
- **TrayController** — Manages system tray icon, menu items (Show Overlay, Open Game Window, Quit), and tray click behaviour. Deep module: isolates platform tray APIs.
- **HotkeyRegistry** — Registers and handles global hotkeys. Emits events to WindowManager (toggle Interact Mode). Deep module: single responsibility for OS hotkey APIs.
- **AppState** — Tracks current shell state: `OverlayVisible | TrayOnly | GameWindowOpen`. Coordinates transitions without coupling windows to each other.

### Interfaces (conceptual)

```
WindowManager:
  - show_overlay() / hide_overlay()
  - open_game_window() / close_game_window() / focus_game_window()
  - set_interact_mode(enabled: bool)  // toggles click-through
  - get_shell_state() -> AppShellState

TrayController:
  - on_show_overlay(callback)
  - on_open_game_window(callback)
  - on_quit(callback)

HotkeyRegistry:
  - register_toggle_interact(hotkey: HotkeyBinding)
```

### Technical clarifications

- Overlay and Game Window are separate webviews, not a single resizable window.
- Interact Mode is a hotkey toggle only (no hotkey for Game Window — button + tray menu).
- Tray keeps the process alive; closing all windows without traying quits the app.
- Alpha targets Windows for testing; macOS compatibility built in but not Alpha-gated.

### Phase scope

| Phase | Scope |
|-------|-------|
| **Alpha** | Overlay window (transparent, always-on-top, click-through toggle), tray minimise/restore, Game Window open/close, Interact Mode hotkey, tray menu |
| **Beta** | macOS menu bar extra polish, window position persistence, multi-monitor awareness |
| **1.0.0** | Steam launch integration, installer packaging, auto-start option |

## Testing Decisions

- Test external behaviour only: given shell state X, action Y produces state Z.
- **WindowManager** and **AppState** are the primary test targets in Rust unit tests (no real windows needed — mock the platform layer).
- **HotkeyRegistry** tested via event emission when hotkey triggered (mock OS layer).
- Tray integration tested manually on Windows; automated tests verify menu callback wiring.
- Prior art: none (greenfield).

## Out of Scope

- Game simulation logic (see Game Simulation PRD)
- Overlay rendering content (see Overlay Presentation PRD)
- Game Window UI content (see Game Window PRD)
- Linux desktop support (deferred)
- Auto-update mechanism (1.0 consideration)

## Further Notes

- ADR-0001: Tauri 2 chosen over Electron/Godot for desktop integration.
- Alpha proves the three-state shell model; everything else depends on this module being stable.
