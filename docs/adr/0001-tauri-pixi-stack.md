# Tauri + TypeScript + PixiJS for desktop shell and rendering

Agentic Afker is a desktop companion idle game — not a traditional game. The hard problems are transparent always-on-top overlays, system tray minimisation, multi-window management (overlay + game window), and a background tick loop while trayed. Game engines (Godot, Unity) treat these as second-class concerns.

We chose **Tauri 2** (Rust shell) with a **TypeScript** frontend and **PixiJS** for the overlay Agent rendering. The game simulation, save state, and tick loop live in Rust; UI and pixel art rendering live in the webview.

**Considered options:**
- **Godot/Unity** — strong for game logic and 2D animation, weak for tray/overlay/click-through desktop integration. Would fight the platform for the primary interaction surface.
- **Electron** — proven for desktop companions, but heavier runtime (~150MB+). Tauri offers the same webview model with a smaller footprint and Rust-native tick/save logic.
- **Pure Rust (egui/iced)** — viable for the game window UI, but pixel art sprite animation and Melvor-style HTML layouts are faster to build with web tech.

**Consequences:**
- Cross-platform builds from headless Linux to Windows/macOS are well-supported.
- Alpha vertical slice validates one stack for overlay, game window, and tray — no engine embedding.
- Form Stage sprite work is layered PixiJS sprites, not a Godot scene tree.
- Steam Cloud later wraps the same local save file (SQLite or JSON).
