mod colorblind;
mod high_contrast;
mod keybindings;
mod screen_reader;
mod ui_scaling;

pub use colorblind::*;
pub use high_contrast::*;
pub use keybindings::*;
pub use screen_reader::*;
pub use ui_scaling::*;

use bevy::prelude::*;

/// Plugin for accessibility features
pub struct AccessibilityPlugin;

impl Plugin for AccessibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccessibilitySettings>()
            .init_resource::<KeybindingsConfig>()
            .init_resource::<ScreenReaderState>()
            .init_resource::<UIScalingSettings>()
            .init_resource::<HighContrastSettings>()
            .init_resource::<ColorblindSettings>()
            .add_systems(PreUpdate, (apply_colorblind_filter, apply_high_contrast))
            .add_systems(
                Update,
                (
                    handle_keybinding_input,
                    update_ui_scaling,
                    screen_reader_announce,
                ),
            );
    }
}

/// Global accessibility settings
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct AccessibilitySettings {
    pub enabled: bool,
    pub screen_reader_enabled: bool,
    pub colorblind_mode: ColorblindMode,
    pub high_contrast_enabled: bool,
    pub ui_scale: f32,
    pub reduced_motion: bool,
    pub text_to_speech: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            screen_reader_enabled: false,
            colorblind_mode: ColorblindMode::None,
            high_contrast_enabled: false,
            ui_scale: 1.0,
            reduced_motion: false,
            text_to_speech: false,
        }
    }
}
