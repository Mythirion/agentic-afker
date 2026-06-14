//! Tauri commands invoked from the frontend.

use tauri::AppHandle;

#[tauri::command]
pub fn open_game_window(app: AppHandle) -> Result<(), String> {
    crate::window_manager::open_game_window(&app).map_err(|err| err.to_string())
}
