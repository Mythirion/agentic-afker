//! Overlay window behaviour — click-through vs interact capture.

use crate::interact_mode::{
    overlay_ignore_cursor_events, toggle_interact_mode, InteractMode, InteractModeState,
};
use crate::window_config::overlay_window_spec;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

pub const INTERACT_MODE_CHANGED_EVENT: &str = "interact-mode-changed";

#[derive(Clone, serde::Serialize)]
pub struct InteractModePayload {
    pub enabled: bool,
}

pub fn apply_overlay_interact_mode(
    app: &AppHandle,
    mode: InteractMode,
) -> tauri::Result<()> {
    if let Some(window) = overlay_window(app) {
        window.set_ignore_cursor_events(overlay_ignore_cursor_events(mode))?;
    }

    let _ = app.emit(
        INTERACT_MODE_CHANGED_EVENT,
        InteractModePayload {
            enabled: mode == InteractMode::On,
        },
    );

    Ok(())
}

pub fn toggle_interact_mode_for_app(app: &AppHandle) {
    let Some(state) = app.try_state::<InteractModeState>() else {
        return;
    };

    let mut mode = state.0.lock().expect("interact mode lock");
    let effects = toggle_interact_mode(*mode);
    *mode = effects.next_mode;

    if let Err(err) = apply_overlay_interact_mode(app, effects.next_mode) {
        eprintln!("failed to apply interact mode: {err}");
    }
}

fn overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(overlay_window_spec().label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interact_mode::InteractMode;

    #[test]
    fn interact_mode_payload_reflects_enabled_state() {
        let on = InteractModePayload { enabled: true };
        let off = InteractModePayload { enabled: false };
        assert!(on.enabled);
        assert!(!off.enabled);
    }

    #[test]
    fn click_through_maps_off_mode_to_ignore_cursor_events() {
        assert!(overlay_ignore_cursor_events(InteractMode::Off));
        assert!(!overlay_ignore_cursor_events(InteractMode::On));
    }
}
