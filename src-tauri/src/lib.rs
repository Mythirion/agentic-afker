mod app_shell_state;
mod commands;
#[cfg(desktop)]
mod hotkey_registry;
mod interact_mode;
mod staging_build;
mod tray_controller;
mod window_config;
mod window_manager;

use app_shell_state::AppShellState;
use commands::open_game_window;
#[cfg(desktop)]
use hotkey_registry::register_interact_mode_hotkey;
use interact_mode::{InteractMode, InteractModeState};
use tauri::Manager;
use tray_controller::{build_tray, handle_exit_requested, handle_window_event, ShellState};
use window_config::overlay_window_spec;
use window_manager::{apply_overlay_interact_mode, handle_game_window_event};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let overlay = overlay_window_spec();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ShellState::new(AppShellState::OverlayVisible))
        .manage(InteractModeState::new(InteractMode::default()));

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_global_shortcut::Builder::new().build());
    }

    builder
        .invoke_handler(tauri::generate_handler![open_game_window])
        .on_window_event(|window, event| {
            handle_window_event(window, event);
            handle_game_window_event(window, event);
        })
        .setup(move |app| {
            build_tray(app.handle())?;

            let window = app
                .get_webview_window(overlay.label)
                .expect("overlay window should exist");

            window
                .set_title(overlay.title)
                .expect("set overlay title");

            apply_overlay_interact_mode(app.handle(), InteractMode::default())?;

            #[cfg(desktop)]
            register_interact_mode_hotkey(app.handle())?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| handle_exit_requested(app_handle, &event));
}
