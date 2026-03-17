use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Main window width
    pub window_width: f32,

    /// Main window height
    pub window_height: f32,

    /// Map panel width ratio (0.0 - 1.0)
    pub map_panel_ratio: f32,

    /// Show minimap by default
    pub show_minimap: bool,

    /// Show log panel by default
    pub show_log_panel: bool,

    /// Show perf panel by default
    pub show_perf_panel: bool,

    /// Default zoom level
    pub default_zoom: f32,

    /// Camera pan speed
    pub camera_pan_speed: f32,

    /// Camera zoom speed
    pub camera_zoom_speed: f32,

    /// Font size for UI text
    pub font_size: f32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            window_width: 1600.0,
            window_height: 900.0,
            map_panel_ratio: 0.7,
            show_minimap: true,
            show_log_panel: true,
            show_perf_panel: false,
            default_zoom: 1.0,
            camera_pan_speed: 500.0,
            camera_zoom_speed: 0.1,
            font_size: 14.0,
        }
    }
}
