use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// UI scaling settings resource
#[derive(Debug, Clone, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct UIScalingSettings {
    pub scale: f32,
    pub font_scale: f32,
    pub icon_scale: f32,
    pub min_scale: f32,
    pub max_scale: f32,
    pub auto_scale: bool,
    pub target_resolution: (f32, f32),
}

impl Default for UIScalingSettings {
    fn default() -> Self {
        Self {
            scale: 1.0,
            font_scale: 1.0,
            icon_scale: 1.0,
            min_scale: 0.5,
            max_scale: 2.0,
            auto_scale: false,
            target_resolution: (1920.0, 1080.0),
        }
    }
}

impl UIScalingSettings {
    /// Create new scaling settings with a specific scale
    pub fn with_scale(scale: f32) -> Self {
        Self {
            scale: scale.clamp(0.5, 2.0),
            font_scale: scale.clamp(0.5, 2.0),
            icon_scale: scale.clamp(0.5, 2.0),
            ..Default::default()
        }
    }

    /// Set the overall scale
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.clamp(self.min_scale, self.max_scale);
    }

    /// Set the font scale independently
    pub fn set_font_scale(&mut self, scale: f32) {
        self.font_scale = scale.clamp(self.min_scale, self.max_scale);
    }

    /// Set the icon scale independently
    pub fn set_icon_scale(&mut self, scale: f32) {
        self.icon_scale = scale.clamp(self.min_scale, self.max_scale);
    }

    /// Calculate auto scale based on window size
    pub fn calculate_auto_scale(&self, window_width: f32, window_height: f32) -> f32 {
        let width_scale = window_width / self.target_resolution.0;
        let height_scale = window_height / self.target_resolution.1;
        let scale = width_scale.min(height_scale);
        scale.clamp(self.min_scale, self.max_scale)
    }

    /// Get scaled font size
    pub fn scale_font(&self, base_size: f32) -> f32 {
        base_size * self.font_scale
    }

    /// Get scaled icon size
    pub fn scale_icon(&self, base_size: f32) -> f32 {
        base_size * self.icon_scale
    }

    /// Get scaled UI element size
    pub fn scale_ui(&self, base_size: f32) -> f32 {
        base_size * self.scale
    }

    /// Get scaled padding
    pub fn scale_padding(&self, base_padding: f32) -> f32 {
        base_padding * self.scale
    }

    /// Get scaled margin
    pub fn scale_margin(&self, base_margin: f32) -> f32 {
        base_margin * self.scale
    }
}

/// Component for UI elements that should scale
#[derive(Debug, Clone, Component)]
pub struct ScalableUI {
    pub base_font_size: Option<f32>,
    pub base_width: Option<f32>,
    pub base_height: Option<f32>,
    pub base_padding: Option<f32>,
}

impl Default for ScalableUI {
    fn default() -> Self {
        Self {
            base_font_size: None,
            base_width: None,
            base_height: None,
            base_padding: None,
        }
    }
}

impl ScalableUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.base_font_size = Some(size);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.base_width = Some(width);
        self.base_height = Some(height);
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.base_padding = Some(padding);
        self
    }
}

/// System to update UI scaling
pub fn update_ui_scaling(
    settings: Res<UIScalingSettings>,
    mut query: Query<(&ScalableUI, &mut Style, &mut Text)>,
    windows: Query<&Window>,
) {
    if !settings.is_changed() && !settings.auto_scale {
        return;
    }

    // Calculate auto scale if enabled
    let scale = if settings.auto_scale {
        if let Ok(window) = windows.get_single() {
            settings.calculate_auto_scale(window.width(), window.height())
        } else {
            settings.scale
        }
    } else {
        settings.scale
    };

    for (scalable, mut style, mut text) in query.iter_mut() {
        // Scale font size
        if let Some(base_font_size) = scalable.base_font_size {
            for section in text.sections.iter_mut() {
                section.style.font_size = base_font_size * settings.font_scale;
            }
        }

        // Scale width
        if let Some(base_width) = scalable.base_width {
            style.width = Val::Px(base_width * scale);
        }

        // Scale height
        if let Some(base_height) = scalable.base_height {
            style.height = Val::Px(base_height * scale);
        }

        // Scale padding
        if let Some(base_padding) = scalable.base_padding {
            style.padding = UiRect::all(Val::Px(base_padding * scale));
        }
    }
}

/// Preset UI scales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIScalePreset {
    Small,
    Normal,
    Large,
    ExtraLarge,
}

impl UIScalePreset {
    pub fn scale(&self) -> f32 {
        match self {
            UIScalePreset::Small => 0.75,
            UIScalePreset::Normal => 1.0,
            UIScalePreset::Large => 1.25,
            UIScalePreset::ExtraLarge => 1.5,
        }
    }

    pub fn font_scale(&self) -> f32 {
        self.scale()
    }

    pub fn icon_scale(&self) -> f32 {
        self.scale()
    }
}

impl From<UIScalePreset> for UIScalingSettings {
    fn from(preset: UIScalePreset) -> Self {
        Self {
            scale: preset.scale(),
            font_scale: preset.font_scale(),
            icon_scale: preset.icon_scale(),
            ..Default::default()
        }
    }
}

/// Text size options for accessibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextSizeOption {
    ExtraSmall,
    Small,
    Normal,
    Large,
    ExtraLarge,
    Huge,
}

impl TextSizeOption {
    pub fn scale(&self) -> f32 {
        match self {
            TextSizeOption::ExtraSmall => 0.7,
            TextSizeOption::Small => 0.85,
            TextSizeOption::Normal => 1.0,
            TextSizeOption::Large => 1.15,
            TextSizeOption::ExtraLarge => 1.3,
            TextSizeOption::Huge => 1.5,
        }
    }

    pub fn base_font_size(&self) -> f32 {
        14.0 * self.scale()
    }
}

/// UI element size options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementSizeOption {
    Compact,
    Normal,
    Comfortable,
    Spacious,
}

impl ElementSizeOption {
    pub fn scale(&self) -> f32 {
        match self {
            ElementSizeOption::Compact => 0.85,
            ElementSizeOption::Normal => 1.0,
            ElementSizeOption::Comfortable => 1.15,
            ElementSizeOption::Spacious => 1.3,
        }
    }

    pub fn padding(&self) -> f32 {
        8.0 * self.scale()
    }

    pub fn margin(&self) -> f32 {
        4.0 * self.scale()
    }

    pub fn button_height(&self) -> f32 {
        32.0 * self.scale()
    }

    pub fn icon_size(&self) -> f32 {
        24.0 * self.scale()
    }
}