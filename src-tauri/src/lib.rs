mod window_config;

use window_config::overlay_window_spec;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let overlay = overlay_window_spec();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let window = app
                .get_webview_window(overlay.label)
                .expect("overlay window should exist");

            window
                .set_title(overlay.title)
                .expect("set overlay title");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
