use bevy::prelude::*;

use super::TextDirection;

/// RTL (Right-to-Left) text support
#[derive(Debug, Clone, Resource, Default)]
pub struct RtlSupport {
    pub enabled: bool,
}

impl RtlSupport {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

/// Process text for RTL display
pub fn process_rtl_text(text: &str, direction: TextDirection) -> String {
    match direction {
        TextDirection::LeftToRight => text.to_string(),
        TextDirection::RightToLeft => {
            // Add RTL mark and reverse visual order for display
            // Note: This is a simplified implementation
            // A full implementation would use a proper bidi algorithm
            let mut result = String::new();

            // Add RTL mark
            result.push('\u{202E}'); // RIGHT-TO-LEFT OVERRIDE
            result.push_str(text);
            result.push('\u{202C}'); // POP DIRECTIONAL FORMATTING

            result
        }
    }
}

/// Detect text direction from content
pub fn detect_text_direction(text: &str) -> TextDirection {
    // Check for RTL characters
    for ch in text.chars() {
        // Arabic range
        if ('\u{0600}'..='\u{06FF}').contains(&ch) {
            return TextDirection::RightToLeft;
        }
        // Hebrew range
        if ('\u{0590}'..='\u{05FF}').contains(&ch) {
            return TextDirection::RightToLeft;
        }
        // Arabic Presentation Forms
        if ('\u{FB50}'..='\u{FDFF}').contains(&ch) {
            return TextDirection::RightToLeft;
        }
        if ('\u{FE70}'..='\u{FEFF}').contains(&ch) {
            return TextDirection::RightToLeft;
        }
    }

    TextDirection::LeftToRight
}

/// RTL-aware text layout
#[derive(Debug, Clone, Component)]
pub struct RtlTextLayout {
    pub direction: TextDirection,
    pub align: TextAlignment,
}

impl Default for RtlTextLayout {
    fn default() -> Self {
        Self {
            direction: TextDirection::LeftToRight,
            align: TextAlignment::Left,
        }
    }
}

/// Text alignment for RTL support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Start, // Align to start of text direction
    End,   // Align to end of text direction
}

impl TextAlignment {
    /// Get the actual alignment based on text direction
    pub fn resolve(&self, direction: TextDirection) -> Self {
        match (self, direction) {
            (TextAlignment::Start, TextDirection::LeftToRight) => TextAlignment::Left,
            (TextAlignment::Start, TextDirection::RightToLeft) => TextAlignment::Right,
            (TextAlignment::End, TextDirection::LeftToRight) => TextAlignment::Right,
            (TextAlignment::End, TextDirection::RightToLeft) => TextAlignment::Left,
            _ => *self,
        }
    }
}

/// Mirror UI layout for RTL
#[derive(Debug, Clone, Component)]
pub struct RtlLayoutMirror {
    pub mirror_horizontal: bool,
    pub mirror_vertical: bool,
}

impl Default for RtlLayoutMirror {
    fn default() -> Self {
        Self {
            mirror_horizontal: true,
            mirror_vertical: false,
        }
    }
}

/// System to apply RTL layout
pub fn apply_rtl_layout_system(
    settings: Res<super::LocalizationSettings>,
    mut query: Query<(&mut RtlLayoutMirror, &mut Style)>,
) {
    let is_rtl = settings.current_language.is_rtl();

    for (mut mirror, mut style) in query.iter_mut() {
        mirror.mirror_horizontal = is_rtl;

        // Mirror flex direction for RTL
        if is_rtl {
            if let FlexDirection::Row = style.flex_direction {
                style.flex_direction = FlexDirection::RowReverse;
            }
        }
    }
}

/// Unicode bidirectional characters
pub mod bidi_chars {
    /// LEFT-TO-RIGHT MARK
    pub const LRM: char = '\u{200E}';
    /// RIGHT-TO-LEFT MARK
    pub const RLM: char = '\u{200F}';
    /// LEFT-TO-RIGHT EMBEDDING
    pub const LRE: char = '\u{202A}';
    /// RIGHT-TO-LEFT EMBEDDING
    pub const RLE: char = '\u{202B}';
    /// POP DIRECTIONAL FORMATTING
    pub const PDF: char = '\u{202C}';
    /// LEFT-TO-RIGHT OVERRIDE
    pub const LRO: char = '\u{202D}';
    /// RIGHT-TO-LEFT OVERRIDE
    pub const RLO: char = '\u{202E}';
    /// LEFT-TO-RIGHT ISOLATE
    pub const LRI: char = '\u{2066}';
    /// RIGHT-TO-LEFT ISOLATE
    pub const RLI: char = '\u{2067}';
    /// FIRST STRONG ISOLATE
    pub const FSI: char = '\u{2068}';
    /// POP DIRECTIONAL ISOLATE
    pub const PDI: char = '\u{2069}';
}

/// Wrap text with direction markers
pub fn wrap_with_direction(text: &str, direction: TextDirection) -> String {
    match direction {
        TextDirection::LeftToRight => {
            format!("{}{}{}", bidi_chars::LRE, text, bidi_chars::PDF)
        }
        TextDirection::RightToLeft => {
            format!("{}{}{}", bidi_chars::RLE, text, bidi_chars::PDF)
        }
    }
}

/// Isolate text for proper bidi handling
pub fn isolate_text(text: &str, direction: TextDirection) -> String {
    match direction {
        TextDirection::LeftToRight => {
            format!("{}{}{}", bidi_chars::LRI, text, bidi_chars::PDI)
        }
        TextDirection::RightToLeft => {
            format!("{}{}{}", bidi_chars::RLI, text, bidi_chars::PDI)
        }
    }
}
