mod formatting;
mod i18n;
mod language;
mod rtl;
mod translations;

pub use formatting::*;
pub use i18n::*;
pub use language::*;
pub use rtl::*;
pub use translations::*;

use bevy::prelude::*;

/// Plugin for localization features
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalizationSettings>()
            .init_resource::<LocalizationState>()
            .init_resource::<TranslationRegistry>()
            .add_systems(PreUpdate, load_translations_system)
            .add_systems(Update, (update_locale_system, format_numbers_system));
    }
}

/// Global localization settings
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct LocalizationSettings {
    pub current_language: Language,
    pub fallback_language: Language,
    pub auto_detect: bool,
    pub date_format: DateFormat,
    pub number_format: NumberFormat,
    pub first_day_of_week: DayOfWeek,
}

impl Default for LocalizationSettings {
    fn default() -> Self {
        Self {
            current_language: Language::English,
            fallback_language: Language::English,
            auto_detect: true,
            date_format: DateFormat::Medium,
            number_format: NumberFormat::default(),
            first_day_of_week: DayOfWeek::Monday,
        }
    }
}

/// Localization state
#[derive(Debug, Clone, Resource, Default)]
pub struct LocalizationState {
    pub loaded: bool,
    pub available_languages: Vec<Language>,
    pub missing_keys: Vec<String>,
}

/// Date format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DateFormat {
    Short,  // 1/1/2024
    Medium, // Jan 1, 2024
    Long,   // January 1, 2024
    Full,   // Monday, January 1, 2024
    Iso,    // 2024-01-01
}

/// Number format settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct NumberFormat {
    pub decimal_separator: DecimalSeparator,
    pub thousands_separator: ThousandsSeparator,
    pub currency_position: CurrencyPosition,
    pub decimal_places: u8,
}

impl Default for NumberFormat {
    fn default() -> Self {
        Self {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::Comma,
            currency_position: CurrencyPosition::Before,
            decimal_places: 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DecimalSeparator {
    Dot,   // 1.234
    Comma, // 1,234
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ThousandsSeparator {
    Comma,      // 1,000
    Dot,        // 1.000
    Space,      // 1 000
    None,       // 1000
    Apostrophe, // 1'000
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CurrencyPosition {
    Before,      // $100
    After,       // 100$
    BeforeSpace, // $ 100
    AfterSpace,  // 100 $
}

/// Day of week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl DayOfWeek {
    pub fn iso_number(&self) -> u8 {
        match self {
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
            DayOfWeek::Sunday => 7,
        }
    }
}
