mod app_shell_state;
mod staging_build;
mod tray_controller;
mod window_config;

use app_shell_state::AppShellState;
use tauri::Manager;
use tray_controller::{build_tray, handle_exit_requested, handle_window_event, ShellState};
use window_config::overlay_window_spec;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let overlay = overlay_window_spec();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ShellState::new(AppShellState::OverlayVisible))
        .on_window_event(|window, event| handle_window_event(window, event))
        .setup(move |app| {
            build_tray(app.handle())?;

            let window = app
                .get_webview_window(overlay.label)
                .expect("overlay window should exist");

            window
                .set_title(overlay.title)
                .expect("set overlay title");

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| handle_exit_requested(app_handle, &event));
}
