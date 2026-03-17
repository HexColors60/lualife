use bevy::prelude::*;

/// High contrast settings resource
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct HighContrastSettings {
    pub enabled: bool,
    pub mode: HighContrastMode,
    pub text_contrast: f32,
    pub background_contrast: f32,
    pub border_width: f32,
    pub use_patterns: bool,
    pub enhance_focus: bool,
}

impl Default for HighContrastSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: HighContrastMode::Normal,
            text_contrast: 1.0,
            background_contrast: 1.0,
            border_width: 1.0,
            use_patterns: false,
            enhance_focus: true,
        }
    }
}

/// High contrast mode variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HighContrastMode {
    /// Normal high contrast (black on white or white on black)
    Normal,
    /// Inverted colors
    Inverted,
    /// Black on white
    Light,
    /// White on black
    Dark,
    /// Yellow on black
    YellowOnBlack,
    /// Custom color scheme
    Custom,
}

impl HighContrastSettings {
    /// Get the primary text color for the current mode
    pub fn text_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.0, 0.0, 0.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(1.0, 1.0, 1.0),
            HighContrastMode::YellowOnBlack => Color::srgb(1.0, 1.0, 0.0),
            HighContrastMode::Custom => Color::srgb(1.0, 1.0, 1.0),
        }
    }

    /// Get the primary background color for the current mode
    pub fn background_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(1.0, 1.0, 1.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(0.0, 0.0, 0.0),
            HighContrastMode::YellowOnBlack => Color::srgb(0.0, 0.0, 0.0),
            HighContrastMode::Custom => Color::srgb(0.1, 0.1, 0.1),
        }
    }

    /// Get the accent color for the current mode
    pub fn accent_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.0, 0.0, 0.8),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(0.5, 0.8, 1.0),
            HighContrastMode::YellowOnBlack => Color::srgb(1.0, 1.0, 0.5),
            HighContrastMode::Custom => Color::srgb(0.3, 0.6, 1.0),
        }
    }

    /// Get the selection color for the current mode
    pub fn selection_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgba(0.0, 0.0, 0.8, 0.3),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgba(1.0, 1.0, 0.0, 0.3),
            HighContrastMode::YellowOnBlack => Color::srgba(1.0, 1.0, 0.0, 0.5),
            HighContrastMode::Custom => Color::srgba(0.3, 0.6, 1.0, 0.3),
        }
    }

    /// Get the border color for the current mode
    pub fn border_color(&self) -> Color {
        self.text_color()
    }

    /// Get the disabled text color
    pub fn disabled_text_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.5, 0.5, 0.5),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(0.5, 0.5, 0.5),
            HighContrastMode::YellowOnBlack => Color::srgb(0.6, 0.6, 0.0),
            HighContrastMode::Custom => Color::srgb(0.5, 0.5, 0.5),
        }
    }

    /// Get the error color
    pub fn error_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.8, 0.0, 0.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(1.0, 0.3, 0.3),
            HighContrastMode::YellowOnBlack => Color::srgb(1.0, 0.5, 0.0),
            HighContrastMode::Custom => Color::srgb(1.0, 0.3, 0.3),
        }
    }

    /// Get the success color
    pub fn success_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.0, 0.6, 0.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(0.3, 1.0, 0.3),
            HighContrastMode::YellowOnBlack => Color::srgb(0.5, 1.0, 0.0),
            HighContrastMode::Custom => Color::srgb(0.3, 1.0, 0.3),
        }
    }

    /// Get the warning color
    pub fn warning_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.8, 0.6, 0.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(1.0, 0.8, 0.0),
            HighContrastMode::YellowOnBlack => Color::srgb(1.0, 0.8, 0.0),
            HighContrastMode::Custom => Color::srgb(1.0, 0.8, 0.0),
        }
    }

    /// Get the focus ring color
    pub fn focus_ring_color(&self) -> Color {
        match self.mode {
            HighContrastMode::Normal | HighContrastMode::Light => Color::srgb(0.0, 0.0, 1.0),
            HighContrastMode::Inverted | HighContrastMode::Dark => Color::srgb(0.0, 1.0, 1.0),
            HighContrastMode::YellowOnBlack => Color::srgb(1.0, 1.0, 1.0),
            HighContrastMode::Custom => Color::srgb(0.5, 0.8, 1.0),
        }
    }

    /// Get the focus ring width
    pub fn focus_ring_width(&self) -> f32 {
        if self.enhance_focus {
            3.0 * self.border_width
        } else {
            2.0 * self.border_width
        }
    }
}

/// System to apply high contrast mode
pub fn apply_high_contrast(
    settings: Res<HighContrastSettings>,
    mut query: Query<(&mut BackgroundColor, &HighContrastElement)>,
    mut text_query: Query<(&mut Text, &HighContrastText)>,
) {
    if !settings.is_changed() {
        return;
    }

    if !settings.enabled {
        return;
    }

    // Apply to background elements
    for (mut bg, element) in query.iter_mut() {
        *bg = BackgroundColor(match element.element_type {
            HighContrastElementType::Background => settings.background_color(),
            HighContrastElementType::Panel => settings.background_color(),
            HighContrastElementType::Button => settings.accent_color(),
            HighContrastElementType::Selected => settings.selection_color(),
            HighContrastElementType::Disabled => Color::srgba(0.3, 0.3, 0.3, 1.0),
        });
    }

    // Apply to text elements
    for (mut text, text_element) in text_query.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.color = match text_element.text_type {
                HighContrastTextType::Normal => settings.text_color(),
                HighContrastTextType::Heading => settings.text_color(),
                HighContrastTextType::Accent => settings.accent_color(),
                HighContrastTextType::Disabled => settings.disabled_text_color(),
                HighContrastTextType::Error => settings.error_color(),
                HighContrastTextType::Success => settings.success_color(),
                HighContrastTextType::Warning => settings.warning_color(),
            };
        }
    }
}

/// Component for high contrast UI elements
#[derive(Debug, Clone, Component)]
pub struct HighContrastElement {
    pub element_type: HighContrastElementType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighContrastElementType {
    Background,
    Panel,
    Button,
    Selected,
    Disabled,
}

/// Component for high contrast text
#[derive(Debug, Clone, Component)]
pub struct HighContrastText {
    pub text_type: HighContrastTextType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighContrastTextType {
    Normal,
    Heading,
    Accent,
    Disabled,
    Error,
    Success,
    Warning,
}

/// Pattern types for distinguishing elements without color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternType {
    None,
    Solid,
    HorizontalLines,
    VerticalLines,
    DiagonalLines,
    CrossHatch,
    Dots,
    Checkerboard,
    Stripes,
}

impl PatternType {
    /// Get a description of the pattern for screen readers
    pub fn description(&self) -> &'static str {
        match self {
            PatternType::None => "no pattern",
            PatternType::Solid => "solid",
            PatternType::HorizontalLines => "horizontal lines",
            PatternType::VerticalLines => "vertical lines",
            PatternType::DiagonalLines => "diagonal lines",
            PatternType::CrossHatch => "cross hatch",
            PatternType::Dots => "dotted",
            PatternType::Checkerboard => "checkerboard",
            PatternType::Stripes => "striped",
        }
    }
}

/// Component for patterned UI elements
#[derive(Debug, Clone, Component)]
pub struct PatternedElement {
    pub pattern: PatternType,
    pub pattern_scale: f32,
}

impl Default for PatternedElement {
    fn default() -> Self {
        Self {
            pattern: PatternType::None,
            pattern_scale: 1.0,
        }
    }
}

/// Focus ring component for enhanced focus visibility
#[derive(Debug, Clone, Component)]
pub struct FocusRing {
    pub visible: bool,
    pub width: f32,
    pub color: Color,
    pub offset: f32,
}

impl Default for FocusRing {
    fn default() -> Self {
        Self {
            visible: false,
            width: 2.0,
            color: Color::srgb(0.0, 0.5, 1.0),
            offset: 2.0,
        }
    }
}

/// System to update focus rings
pub fn update_focus_rings(
    settings: Res<HighContrastSettings>,
    mut query: Query<(&Focusable, &mut FocusRing)>,
) {
    for (focusable, mut ring) in query.iter_mut() {
        ring.visible = focusable.focused;
        if settings.enabled && settings.enhance_focus {
            ring.width = settings.focus_ring_width();
            ring.color = settings.focus_ring_color();
        }
    }
}

use crate::accessibility::Focusable;
