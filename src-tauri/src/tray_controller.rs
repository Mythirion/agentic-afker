//! System tray wiring for minimise-to-tray and restore.

use crate::app_shell_state::{apply_shell_action, AppShellState, ShellAction};
use crate::window_config::overlay_window_spec;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, RunEvent, WebviewWindow, Window, WindowEvent,
};

pub struct ShellState(pub Mutex<AppShellState>);

impl ShellState {
    pub fn new(initial: AppShellState) -> Self {
        Self(Mutex::new(initial))
    }
}

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if window.label() != overlay_window_spec().label {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        minimize_overlay_to_tray(window.app_handle());
    }
}

pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let show_item = MenuItem::with_id(app, "show-overlay", "Show Overlay", true, None::<&str>)?;
    let minimize_item =
        MenuItem::with_id(app, "minimize-to-tray", "Minimise to Tray", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &minimize_item, &quit_item])?;

    let icon = app
        .default_window_icon()
        .ok_or_else(|| tauri::Error::Anyhow(anyhow::anyhow!("missing app icon")))?
        .clone();

    TrayIconBuilder::with_id("agent-tray")
        .icon(icon)
        .tooltip("Agentic Afker")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show-overlay" => restore_overlay_from_tray(app),
            "minimize-to-tray" => minimize_overlay_to_tray(app),
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                restore_overlay_from_tray(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

pub fn handle_exit_requested(app: &AppHandle, event: &RunEvent) {
    if let RunEvent::ExitRequested { api, .. } = event {
        if let Some(shell_state) = app.try_state::<ShellState>() {
            let state = shell_state.0.lock().expect("shell state lock");
            if *state == AppShellState::TrayOnly {
                api.prevent_exit();
            }
        }
    }
}

fn overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(overlay_window_spec().label)
}

fn minimize_overlay_to_tray(app: &AppHandle) {
    let Some(shell_state) = app.try_state::<ShellState>() else {
        return;
    };

    let mut state = shell_state.0.lock().expect("shell state lock");
    let effects = apply_shell_action(*state, ShellAction::MinimizeToTray);

    if effects.hide_overlay {
        if let Some(window) = overlay_window(app) {
            let _ = window.hide();
        }
    }

    *state = effects.next_state;
}

fn restore_overlay_from_tray(app: &AppHandle) {
    let Some(shell_state) = app.try_state::<ShellState>() else {
        return;
    };

    let mut state = shell_state.0.lock().expect("shell state lock");
    let effects = apply_shell_action(*state, ShellAction::RestoreOverlay);

    if effects.show_overlay {
        if let Some(window) = overlay_window(app) {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }

    *state = effects.next_state;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_shell_state::AppShellState;

    #[test]
    fn shell_state_starts_overlay_visible() {
        let shell = ShellState::new(AppShellState::OverlayVisible);
        assert_eq!(*shell.0.lock().unwrap(), AppShellState::OverlayVisible);
    }
}
