use bevy::prelude::*;

use super::{CurrencyPosition, DateFormat, DecimalSeparator, NumberFormat, ThousandsSeparator};

/// Format a number according to locale settings
pub fn format_number(value: f64, format: &NumberFormat) -> String {
    let decimal_str = match format.decimal_separator {
        DecimalSeparator::Dot => ".",
        DecimalSeparator::Comma => ",",
    };

    let thousands_str = match format.thousands_separator {
        ThousandsSeparator::Comma => ",",
        ThousandsSeparator::Dot => ".",
        ThousandsSeparator::Space => " ",
        ThousandsSeparator::None => "",
        ThousandsSeparator::Apostrophe => "'",
    };

    let abs_value = value.abs();
    let sign = if value < 0.0 { "-" } else { "" };

    // Split into integer and decimal parts
    let rounded = (abs_value * 10f64.powi(format.decimal_places as i32)).round();
    let integer_part = (rounded / 10f64.powi(format.decimal_places as i32)) as u64;
    let decimal_part = (rounded % 10f64.powi(format.decimal_places as i32)) as u32;

    // Format integer part with thousands separator
    let int_str = integer_part.to_string();
    let mut formatted_int = String::new();
    for (i, c) in int_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 && !thousands_str.is_empty() {
            formatted_int.push_str(&thousands_str.chars().rev().collect::<String>());
        }
        formatted_int.push(c);
    }
    let formatted_int: String = formatted_int.chars().rev().collect();

    // Combine parts
    if format.decimal_places > 0 {
        let decimal_formatted = format!(
            "{:0>width$}",
            decimal_part,
            width = format.decimal_places as usize
        );
        format!(
            "{}{}{}{}",
            sign, formatted_int, decimal_str, decimal_formatted
        )
    } else {
        format!("{}{}", sign, formatted_int)
    }
}

/// Format an integer according to locale settings
pub fn format_integer(value: i64, format: &NumberFormat) -> String {
    format_number(
        value as f64,
        &NumberFormat {
            decimal_places: 0,
            ..*format
        },
    )
}

/// Format currency according to locale settings
pub fn format_currency(value: f64, currency_symbol: &str, format: &NumberFormat) -> String {
    let number = format_number(value, format);

    match format.currency_position {
        CurrencyPosition::Before => format!("{}{}", currency_symbol, number),
        CurrencyPosition::After => format!("{}{}", number, currency_symbol),
        CurrencyPosition::BeforeSpace => format!("{} {}", currency_symbol, number),
        CurrencyPosition::AfterSpace => format!("{} {}", number, currency_symbol),
    }
}

/// Format a percentage according to locale settings
pub fn format_percentage(value: f64, format: &NumberFormat) -> String {
    format!("{}%", format_number(value * 100.0, format))
}

/// Format a date according to locale settings
pub fn format_date(year: i32, month: u32, day: u32, date_format: DateFormat) -> String {
    match date_format {
        DateFormat::Short => format!("{}/{}/{}", month, day, year),
        DateFormat::Medium => {
            let month_name = month_name_short(month);
            format!("{} {}, {}", month_name, day, year)
        }
        DateFormat::Long => {
            let month_name = month_name_full(month);
            format!("{} {}, {}", month_name, day, year)
        }
        DateFormat::Full => {
            let month_name = month_name_full(month);
            let day_name = day_name_full(year, month, day);
            format!("{}, {} {}, {}", day_name, month_name, day, year)
        }
        DateFormat::Iso => format!("{:04}-{:02}-{:02}", year, month, day),
    }
}

/// Format time according to locale settings
pub fn format_time(hour: u32, minute: u32, second: u32, use_24h: bool) -> String {
    if use_24h {
        format!("{:02}:{:02}:{:02}", hour, minute, second)
    } else {
        let period = if hour >= 12 { "PM" } else { "AM" };
        let display_hour = if hour == 0 {
            12
        } else if hour > 12 {
            hour - 12
        } else {
            hour
        };
        format!("{:02}:{:02}:{:02} {}", display_hour, minute, second, period)
    }
}

/// Format a duration in seconds
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        if secs > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}m", minutes)
        }
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        if minutes > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}h", hours)
        }
    } else {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        if hours > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}d", days)
        }
    }
}

/// Get short month name
fn month_name_short(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

/// Get full month name
fn month_name_full(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "???",
    }
}

/// Get full day name
fn day_name_full(year: i32, month: u32, day: u32) -> &'static str {
    // Zeller's congruence to find day of week
    let m = if month < 3 { month + 12 } else { month };
    let y = if month < 3 { year - 1 } else { year };
    let k = y % 100;
    let j = y / 100;
    let h = (day as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;

    match h {
        0 => "Saturday",
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        _ => "???",
    }
}

/// System to format numbers in UI
pub fn format_numbers_system(_settings: Res<super::LocalizationSettings>) {
    // This system would be used to update UI elements with formatted numbers
    // Implementation depends on how numbers are displayed in the UI
}

/// Get number format for a locale
pub fn get_locale_number_format(language: super::Language) -> NumberFormat {
    match language {
        // European formats (comma as decimal, dot as thousands)
        super::Language::German
        | super::Language::French
        | super::Language::Italian
        | super::Language::Spanish
        | super::Language::Portuguese
        | super::Language::Dutch
        | super::Language::Polish
        | super::Language::Czech
        | super::Language::Hungarian
        | super::Language::Romanian
        | super::Language::Greek
        | super::Language::Russian
        | super::Language::Ukrainian
        | super::Language::Turkish => NumberFormat {
            decimal_separator: DecimalSeparator::Comma,
            thousands_separator: ThousandsSeparator::Dot,
            currency_position: CurrencyPosition::After,
            decimal_places: 2,
        },

        // Swiss format (dot as decimal, apostrophe as thousands)
        _ if matches!(language, super::Language::German) => NumberFormat {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::Apostrophe,
            currency_position: CurrencyPosition::BeforeSpace,
            decimal_places: 2,
        },

        // English format (dot as decimal, comma as thousands)
        super::Language::English => NumberFormat {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::Comma,
            currency_position: CurrencyPosition::Before,
            decimal_places: 2,
        },

        // Chinese/Japanese format (dot as decimal, no thousands separator)
        super::Language::ChineseSimplified
        | super::Language::ChineseTraditional
        | super::Language::Japanese
        | super::Language::Korean => NumberFormat {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::None,
            currency_position: CurrencyPosition::Before,
            decimal_places: 2,
        },

        // Indian format (dot as decimal, comma for lakhs/crores)
        super::Language::Hindi | super::Language::Bengali => NumberFormat {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::Comma,
            currency_position: CurrencyPosition::Before,
            decimal_places: 2,
        },

        // Arabic format (varies by region)
        super::Language::Arabic => NumberFormat {
            decimal_separator: DecimalSeparator::Dot,
            thousands_separator: ThousandsSeparator::Comma,
            currency_position: CurrencyPosition::AfterSpace,
            decimal_places: 2,
        },

        // Default to English format
        _ => NumberFormat::default(),
    }
}
