//! Desktop shell state for Overlay vs Tray.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppShellState {
    OverlayVisible,
    TrayOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellAction {
    MinimizeToTray,
    RestoreOverlay,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellEffects {
    pub next_state: AppShellState,
    pub show_overlay: bool,
    pub hide_overlay: bool,
    pub exit_process: bool,
}

pub fn apply_shell_action(state: AppShellState, action: ShellAction) -> ShellEffects {
    match (state, action) {
        (AppShellState::OverlayVisible, ShellAction::MinimizeToTray) => ShellEffects {
            next_state: AppShellState::TrayOnly,
            show_overlay: false,
            hide_overlay: true,
            exit_process: false,
        },
        (AppShellState::TrayOnly, ShellAction::RestoreOverlay) => ShellEffects {
            next_state: AppShellState::OverlayVisible,
            show_overlay: true,
            hide_overlay: false,
            exit_process: false,
        },
        (_, ShellAction::Quit) => ShellEffects {
            next_state: state,
            show_overlay: false,
            hide_overlay: false,
            exit_process: true,
        },
        (current, _) => ShellEffects {
            next_state: current,
            show_overlay: false,
            hide_overlay: false,
            exit_process: false,
        },
    }
}

pub fn process_alive_in_state(state: AppShellState) -> bool {
    matches!(
        state,
        AppShellState::OverlayVisible | AppShellState::TrayOnly
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_to_tray_hides_overlay_and_keeps_process_alive() {
        let effects = apply_shell_action(
            AppShellState::OverlayVisible,
            ShellAction::MinimizeToTray,
        );

        assert_eq!(effects.next_state, AppShellState::TrayOnly);
        assert!(effects.hide_overlay);
        assert!(!effects.show_overlay);
        assert!(!effects.exit_process);
        assert!(process_alive_in_state(effects.next_state));
    }

    #[test]
    fn tray_click_restores_overlay() {
        let effects = apply_shell_action(AppShellState::TrayOnly, ShellAction::RestoreOverlay);

        assert_eq!(effects.next_state, AppShellState::OverlayVisible);
        assert!(effects.show_overlay);
        assert!(!effects.hide_overlay);
        assert!(!effects.exit_process);
    }

    #[test]
    fn quit_exits_without_hiding_overlay_first() {
        let effects = apply_shell_action(AppShellState::TrayOnly, ShellAction::Quit);

        assert!(effects.exit_process);
        assert!(!effects.show_overlay);
        assert!(!effects.hide_overlay);
    }

    #[test]
    fn restore_while_overlay_visible_is_no_op() {
        let effects = apply_shell_action(
            AppShellState::OverlayVisible,
            ShellAction::RestoreOverlay,
        );

        assert_eq!(effects.next_state, AppShellState::OverlayVisible);
        assert!(!effects.show_overlay);
        assert!(!effects.hide_overlay);
    }
}
