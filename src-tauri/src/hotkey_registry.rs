//! Global hotkey registration for Interact Mode toggle.

use crate::interact_mode::interact_mode_hotkey;
use crate::window_manager::toggle_interact_mode_for_app;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn register_interact_mode_hotkey(app: &AppHandle) -> tauri::Result<()> {
    let shortcut: Shortcut = interact_mode_hotkey()
        .parse()
        .map_err(|err| tauri::Error::Anyhow(anyhow::anyhow!("invalid hotkey: {err}")))?;

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _, event| {
            if event.state == ShortcutState::Pressed {
                toggle_interact_mode_for_app(app);
            }
        })
        .map_err(|err| tauri::Error::Anyhow(anyhow::anyhow!("{err}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interact_hotkey_parses_as_shortcut() {
        let shortcut: Shortcut = interact_mode_hotkey().parse().expect("valid shortcut");
        assert!(!shortcut.to_string().is_empty());
    }
}
