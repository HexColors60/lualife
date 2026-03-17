use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Colorblind mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub enum ColorblindMode {
    /// No colorblind correction
    None,
    /// Protanopia (red-blind)
    Protanopia,
    /// Deuteranopia (green-blind)
    Deuteranopia,
    /// Tritanopia (blue-blind)
    Tritanopia,
    /// Achromatopsia (total color blindness)
    Achromatopsia,
}

impl Default for ColorblindMode {
    fn default() -> Self {
        Self::None
    }
}

/// Colorblind settings resource
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct ColorblindSettings {
    pub mode: ColorblindMode,
    pub intensity: f32,
}

impl Default for ColorblindSettings {
    fn default() -> Self {
        Self {
            mode: ColorblindMode::None,
            intensity: 1.0,
        }
    }
}

/// Color correction matrices for different colorblind types
/// These are 3x3 matrices that transform RGB colors
pub fn get_colorblind_matrix(mode: ColorblindMode) -> [[f32; 3]; 3] {
    match mode {
        ColorblindMode::None => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        // Protanopia simulation
        ColorblindMode::Protanopia => [
            [0.567, 0.433, 0.0],
            [0.558, 0.442, 0.0],
            [0.0, 0.242, 0.758],
        ],
        // Deuteranopia simulation
        ColorblindMode::Deuteranopia => [[0.625, 0.375, 0.0], [0.7, 0.3, 0.0], [0.0, 0.3, 0.7]],
        // Tritanopia simulation
        ColorblindMode::Tritanopia => [[0.95, 0.05, 0.0], [0.0, 0.433, 0.567], [0.0, 0.475, 0.525]],
        // Achromatopsia (grayscale)
        ColorblindMode::Achromatopsia => [
            [0.299, 0.587, 0.114],
            [0.299, 0.587, 0.114],
            [0.299, 0.587, 0.114],
        ],
    }
}

/// Daltonization correction matrices (help colorblind users distinguish colors)
pub fn get_daltonization_matrix(mode: ColorblindMode) -> [[f32; 3]; 3] {
    match mode {
        ColorblindMode::None => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        ColorblindMode::Protanopia => [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
        ColorblindMode::Deuteranopia => [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        ColorblindMode::Tritanopia => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
        ColorblindMode::Achromatopsia => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
}

/// Apply colorblind filter to a color
pub fn apply_colorblind_filter_to_color(
    color: Color,
    mode: ColorblindMode,
    intensity: f32,
) -> Color {
    if mode == ColorblindMode::None || intensity <= 0.0 {
        return color;
    }

    let srgba = color.to_srgba();
    let r = srgba.red;
    let g = srgba.green;
    let b = srgba.blue;
    let a = srgba.alpha;

    let matrix = get_colorblind_matrix(mode);

    let new_r = r * matrix[0][0] + g * matrix[0][1] + b * matrix[0][2];
    let new_g = r * matrix[1][0] + g * matrix[1][1] + b * matrix[1][2];
    let new_b = r * matrix[2][0] + g * matrix[2][1] + b * matrix[2][2];

    // Blend with original based on intensity
    let final_r = r + (new_r - r) * intensity;
    let final_g = g + (new_g - g) * intensity;
    let final_b = b + (new_b - b) * intensity;

    Color::srgba(final_r, final_g, final_b, a)
}

/// System to apply colorblind filter to game colors
pub fn apply_colorblind_filter(
    settings: Res<ColorblindSettings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }

    // Apply to standard materials
    for (_, material) in materials.iter_mut() {
        material.base_color = apply_colorblind_filter_to_color(
            material.base_color,
            settings.mode,
            settings.intensity,
        );
    }

    // Apply to color materials (2D)
    for (_, material) in color_materials.iter_mut() {
        material.color =
            apply_colorblind_filter_to_color(material.color, settings.mode, settings.intensity);
    }
}

/// Color palette alternatives for colorblind users
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorblindPalette {
    Default,
    ProtanopiaFriendly,
    DeuteranopiaFriendly,
    TritanopiaFriendly,
    HighContrast,
}

impl ColorblindPalette {
    /// Get faction colors adjusted for colorblind mode
    pub fn get_faction_colors(&self, faction_id: u16) -> Color {
        // Use distinct patterns and colors that work for colorblind users
        let hue = (faction_id as f32 * 137.5) % 360.0; // Golden angle distribution

        match self {
            ColorblindPalette::Default => Color::hsl(hue, 0.7, 0.5),
            ColorblindPalette::ProtanopiaFriendly => {
                // Avoid red-green confusion, use blue-orange spectrum
                let adjusted_hue = if hue > 60.0 && hue < 180.0 {
                    // Shift green range to blue
                    (hue + 60.0) % 360.0
                } else {
                    hue
                };
                Color::hsl(adjusted_hue, 0.8, 0.5)
            }
            ColorblindPalette::DeuteranopiaFriendly => {
                // Similar to protanopia, avoid red-green
                let adjusted_hue = if hue > 30.0 && hue < 150.0 {
                    (hue + 90.0) % 360.0
                } else {
                    hue
                };
                Color::hsl(adjusted_hue, 0.8, 0.5)
            }
            ColorblindPalette::TritanopiaFriendly => {
                // Avoid blue-yellow confusion
                let adjusted_hue = if hue > 180.0 && hue < 300.0 {
                    (hue + 60.0) % 360.0
                } else {
                    hue
                };
                Color::hsl(adjusted_hue, 0.8, 0.5)
            }
            ColorblindPalette::HighContrast => {
                // Use high contrast colors with distinct patterns
                let colors = [
                    Color::srgb(0.0, 0.0, 0.0),  // Black
                    Color::srgb(1.0, 1.0, 1.0),  // White
                    Color::srgb(0.0, 0.0, 1.0),  // Blue
                    Color::srgb(1.0, 0.5, 0.0),  // Orange
                    Color::srgb(0.5, 0.0, 0.5),  // Purple
                    Color::srgb(0.0, 0.5, 0.5),  // Teal
                    Color::srgb(1.0, 1.0, 0.0),  // Yellow
                    Color::srgb(0.5, 0.25, 0.0), // Brown
                ];
                colors[faction_id as usize % colors.len()]
            }
        }
    }

    /// Get resource type colors adjusted for colorblind mode
    pub fn get_resource_color(&self, resource_type: u8) -> Color {
        match self {
            ColorblindPalette::Default => {
                let colors = [
                    Color::srgb(1.0, 1.0, 0.0), // Power - Yellow
                    Color::srgb(0.7, 0.4, 0.2), // Iron - Brown
                    Color::srgb(0.8, 0.5, 0.2), // Copper - Copper
                    Color::srgb(0.9, 0.9, 0.9), // Silicon - Light gray
                    Color::srgb(0.5, 0.8, 1.0), // Crystal - Light blue
                    Color::srgb(0.3, 0.3, 0.3), // Carbon - Dark gray
                    Color::srgb(0.6, 0.6, 0.6), // Stone - Gray
                    Color::srgb(1.0, 1.0, 0.3), // Sulfur - Bright yellow
                    Color::srgb(0.3, 0.5, 0.8), // Water - Blue
                    Color::srgb(0.3, 0.7, 0.3), // Biomass - Green
                ];
                colors[resource_type as usize % colors.len()]
            }
            ColorblindPalette::ProtanopiaFriendly | ColorblindPalette::DeuteranopiaFriendly => {
                // Use blue-orange spectrum with patterns
                let colors = [
                    Color::srgb(1.0, 0.6, 0.0), // Power - Orange
                    Color::srgb(0.2, 0.2, 0.8), // Iron - Blue
                    Color::srgb(0.0, 0.6, 0.8), // Copper - Cyan
                    Color::srgb(0.9, 0.9, 0.9), // Silicon - Light gray
                    Color::srgb(0.0, 0.4, 0.8), // Crystal - Medium blue
                    Color::srgb(0.3, 0.3, 0.3), // Carbon - Dark gray
                    Color::srgb(0.6, 0.6, 0.6), // Stone - Gray
                    Color::srgb(1.0, 0.8, 0.0), // Sulfur - Gold
                    Color::srgb(0.2, 0.5, 0.9), // Water - Light blue
                    Color::srgb(0.0, 0.7, 0.7), // Biomass - Teal
                ];
                colors[resource_type as usize % colors.len()]
            }
            ColorblindPalette::TritanopiaFriendly => {
                // Avoid blue-yellow confusion
                let colors = [
                    Color::srgb(1.0, 0.0, 0.5), // Power - Magenta
                    Color::srgb(0.6, 0.3, 0.0), // Iron - Brown
                    Color::srgb(0.8, 0.4, 0.0), // Copper - Orange
                    Color::srgb(0.9, 0.9, 0.9), // Silicon - Light gray
                    Color::srgb(0.7, 0.0, 0.7), // Crystal - Purple
                    Color::srgb(0.3, 0.3, 0.3), // Carbon - Dark gray
                    Color::srgb(0.6, 0.6, 0.6), // Stone - Gray
                    Color::srgb(1.0, 0.5, 0.0), // Sulfur - Orange
                    Color::srgb(0.0, 0.6, 0.6), // Water - Teal
                    Color::srgb(0.0, 0.8, 0.4), // Biomass - Green
                ];
                colors[resource_type as usize % colors.len()]
            }
            ColorblindPalette::HighContrast => {
                let colors = [
                    Color::srgb(1.0, 1.0, 0.0), // Power - Yellow
                    Color::srgb(0.0, 0.0, 1.0), // Iron - Blue
                    Color::srgb(1.0, 0.5, 0.0), // Copper - Orange
                    Color::srgb(1.0, 1.0, 1.0), // Silicon - White
                    Color::srgb(0.5, 0.0, 0.5), // Crystal - Purple
                    Color::srgb(0.3, 0.3, 0.3), // Carbon - Dark gray
                    Color::srgb(0.6, 0.6, 0.6), // Stone - Gray
                    Color::srgb(1.0, 0.8, 0.0), // Sulfur - Gold
                    Color::srgb(0.0, 0.5, 1.0), // Water - Light blue
                    Color::srgb(0.0, 0.8, 0.0), // Biomass - Green
                ];
                colors[resource_type as usize % colors.len()]
            }
        }
    }
}

impl From<ColorblindMode> for ColorblindPalette {
    fn from(mode: ColorblindMode) -> Self {
        match mode {
            ColorblindMode::None => ColorblindPalette::Default,
            ColorblindMode::Protanopia => ColorblindPalette::ProtanopiaFriendly,
            ColorblindMode::Deuteranopia => ColorblindPalette::DeuteranopiaFriendly,
            ColorblindMode::Tritanopia => ColorblindPalette::TritanopiaFriendly,
            ColorblindMode::Achromatopsia => ColorblindPalette::HighContrast,
        }
    }
}
