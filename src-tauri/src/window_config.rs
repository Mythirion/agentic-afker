//! Overlay window configuration for the Agent window.

#[derive(Debug, Clone, PartialEq)]
pub struct OverlayWindowSpec {
    pub label: &'static str,
    pub title: &'static str,
    pub width: f64,
    pub height: f64,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
    pub resizable: bool,
}

pub fn overlay_window_spec() -> OverlayWindowSpec {
    OverlayWindowSpec {
        label: "overlay",
        title: "Agent",
        width: 300.0,
        height: 200.0,
        transparent: true,
        decorations: false,
        always_on_top: true,
        resizable: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn overlay_window_spec_matches_agent_overlay_requirements() {
        let spec = overlay_window_spec();

        assert_eq!(spec.label, "overlay");
        assert_eq!(spec.width, 300.0);
        assert_eq!(spec.height, 200.0);
        assert!(spec.transparent);
        assert!(!spec.decorations);
        assert!(spec.always_on_top);
        assert!(!spec.resizable);
    }

    #[test]
    fn tauri_config_declares_overlay_window() {
        let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json");
        let config: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(config_path).expect("read tauri config"))
                .expect("parse tauri config");

        let windows = config["app"]["windows"]
            .as_array()
            .expect("windows array");
        let overlay = windows
            .iter()
            .find(|window| window["label"] == "overlay")
            .expect("overlay window entry");

        assert_eq!(overlay["width"], 300);
        assert_eq!(overlay["height"], 200);
        assert_eq!(overlay["transparent"], true);
        assert_eq!(overlay["alwaysOnTop"], true);
        assert_eq!(overlay["decorations"], false);
    }

    #[test]
    fn tauri_config_declares_tray_icon() {
        let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json");
        let config: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(config_path).expect("read tauri config"))
                .expect("parse tauri config");

        assert!(
            config["app"]["trayIcon"]["iconPath"].is_string(),
            "tray icon should be configured for system tray visibility"
        );
    }
}
