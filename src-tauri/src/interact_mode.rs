//! Interact Mode — toggles Overlay click-through vs click capture.

use std::sync::Mutex;

/// Global hotkey that toggles Interact Mode (system-wide).
pub fn interact_mode_hotkey() -> &'static str {
    "Alt+Shift+I"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractMode {
    Off,
    On,
}

impl InteractMode {
    pub fn default() -> Self {
        Self::Off
    }
}

pub struct InteractModeState(pub Mutex<InteractMode>);

impl InteractModeState {
    pub fn new(initial: InteractMode) -> Self {
        Self(Mutex::new(initial))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractModeEffects {
    pub next_mode: InteractMode,
    /// When true, mouse events pass through the Overlay to apps below.
    pub ignore_cursor_events: bool,
}

pub fn overlay_ignore_cursor_events(mode: InteractMode) -> bool {
    matches!(mode, InteractMode::Off)
}

pub fn toggle_interact_mode(current: InteractMode) -> InteractModeEffects {
    let next_mode = match current {
        InteractMode::Off => InteractMode::On,
        InteractMode::On => InteractMode::Off,
    };

    InteractModeEffects {
        next_mode,
        ignore_cursor_events: overlay_ignore_cursor_events(next_mode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interact_mode_state_starts_off() {
        let state = InteractModeState::new(InteractMode::default());
        assert_eq!(*state.0.lock().unwrap(), InteractMode::Off);
    }

    #[test]
    fn interact_mode_defaults_to_off_and_click_through() {
        let mode = InteractMode::default();
        assert_eq!(mode, InteractMode::Off);
        assert!(overlay_ignore_cursor_events(mode));
    }

    #[test]
    fn toggle_enables_click_capture() {
        let effects = toggle_interact_mode(InteractMode::Off);
        assert_eq!(effects.next_mode, InteractMode::On);
        assert!(!effects.ignore_cursor_events);
    }

    #[test]
    fn toggle_again_restores_click_through() {
        let effects = toggle_interact_mode(InteractMode::On);
        assert_eq!(effects.next_mode, InteractMode::Off);
        assert!(effects.ignore_cursor_events);
    }

    #[test]
    fn double_toggle_returns_to_default() {
        let first = toggle_interact_mode(InteractMode::default());
        let second = toggle_interact_mode(first.next_mode);
        assert_eq!(second.next_mode, InteractMode::Off);
        assert!(second.ignore_cursor_events);
    }

    #[test]
    fn hotkey_binding_is_configured() {
        assert!(
            !interact_mode_hotkey().is_empty(),
            "global interact hotkey should be defined"
        );
        assert!(
            interact_mode_hotkey().contains('+'),
            "hotkey should use modifier+key form"
        );
    }
}
