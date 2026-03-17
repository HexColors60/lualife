use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum Language {
    // Major languages
    English,
    ChineseSimplified,
    ChineseTraditional,
    Spanish,
    French,
    German,
    Japanese,
    Korean,
    Portuguese,
    Russian,
    Italian,
    Dutch,
    Polish,
    Turkish,
    Arabic,
    Hindi,
    // Additional languages
    Bengali,
    Vietnamese,
    Thai,
    Indonesian,
    Swedish,
    Norwegian,
    Danish,
    Finnish,
    Czech,
    Hungarian,
    Romanian,
    Greek,
    Hebrew,
    Persian,
    Ukrainian,
}

impl Language {
    /// Get the ISO 639-1 code for the language
    pub fn iso_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::ChineseSimplified => "zh-CN",
            Language::ChineseTraditional => "zh-TW",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Italian => "it",
            Language::Dutch => "nl",
            Language::Polish => "pl",
            Language::Turkish => "tr",
            Language::Arabic => "ar",
            Language::Hindi => "hi",
            Language::Bengali => "bn",
            Language::Vietnamese => "vi",
            Language::Thai => "th",
            Language::Indonesian => "id",
            Language::Swedish => "sv",
            Language::Norwegian => "no",
            Language::Danish => "da",
            Language::Finnish => "fi",
            Language::Czech => "cs",
            Language::Hungarian => "hu",
            Language::Romanian => "ro",
            Language::Greek => "el",
            Language::Hebrew => "he",
            Language::Persian => "fa",
            Language::Ukrainian => "uk",
        }
    }

    /// Get the native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "简体中文",
            Language::ChineseTraditional => "繁體中文",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Portuguese => "Português",
            Language::Russian => "Русский",
            Language::Italian => "Italiano",
            Language::Dutch => "Nederlands",
            Language::Polish => "Polski",
            Language::Turkish => "Türkçe",
            Language::Arabic => "العربية",
            Language::Hindi => "हिन्दी",
            Language::Bengali => "বাংলা",
            Language::Vietnamese => "Tiếng Việt",
            Language::Thai => "ไทย",
            Language::Indonesian => "Bahasa Indonesia",
            Language::Swedish => "Svenska",
            Language::Norwegian => "Norsk",
            Language::Danish => "Dansk",
            Language::Finnish => "Suomi",
            Language::Czech => "Čeština",
            Language::Hungarian => "Magyar",
            Language::Romanian => "Română",
            Language::Greek => "Ελληνικά",
            Language::Hebrew => "עברית",
            Language::Persian => "فارسی",
            Language::Ukrainian => "Українська",
        }
    }

    /// Get the English name of the language
    pub fn english_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "Chinese (Simplified)",
            Language::ChineseTraditional => "Chinese (Traditional)",
            Language::Spanish => "Spanish",
            Language::French => "French",
            Language::German => "German",
            Language::Japanese => "Japanese",
            Language::Korean => "Korean",
            Language::Portuguese => "Portuguese",
            Language::Russian => "Russian",
            Language::Italian => "Italian",
            Language::Dutch => "Dutch",
            Language::Polish => "Polish",
            Language::Turkish => "Turkish",
            Language::Arabic => "Arabic",
            Language::Hindi => "Hindi",
            Language::Bengali => "Bengali",
            Language::Vietnamese => "Vietnamese",
            Language::Thai => "Thai",
            Language::Indonesian => "Indonesian",
            Language::Swedish => "Swedish",
            Language::Norwegian => "Norwegian",
            Language::Danish => "Danish",
            Language::Finnish => "Finnish",
            Language::Czech => "Czech",
            Language::Hungarian => "Hungarian",
            Language::Romanian => "Romanian",
            Language::Greek => "Greek",
            Language::Hebrew => "Hebrew",
            Language::Persian => "Persian",
            Language::Ukrainian => "Ukrainian",
        }
    }

    /// Check if the language is RTL (Right-to-Left)
    pub fn is_rtl(&self) -> bool {
        matches!(
            self,
            Language::Arabic | Language::Hebrew | Language::Persian
        )
    }

    /// Get the text direction
    pub fn text_direction(&self) -> TextDirection {
        if self.is_rtl() {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        }
    }

    /// Parse from ISO code
    pub fn from_iso_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" => Some(Language::English),
            "zh" | "zh-cn" | "zh-hans" => Some(Language::ChineseSimplified),
            "zh-tw" | "zh-hant" => Some(Language::ChineseTraditional),
            "es" | "es-es" => Some(Language::Spanish),
            "fr" | "fr-fr" => Some(Language::French),
            "de" | "de-de" => Some(Language::German),
            "ja" => Some(Language::Japanese),
            "ko" => Some(Language::Korean),
            "pt" | "pt-br" | "pt-pt" => Some(Language::Portuguese),
            "ru" => Some(Language::Russian),
            "it" => Some(Language::Italian),
            "nl" => Some(Language::Dutch),
            "pl" => Some(Language::Polish),
            "tr" => Some(Language::Turkish),
            "ar" => Some(Language::Arabic),
            "hi" => Some(Language::Hindi),
            "bn" => Some(Language::Bengali),
            "vi" => Some(Language::Vietnamese),
            "th" => Some(Language::Thai),
            "id" => Some(Language::Indonesian),
            "sv" => Some(Language::Swedish),
            "no" => Some(Language::Norwegian),
            "da" => Some(Language::Danish),
            "fi" => Some(Language::Finnish),
            "cs" => Some(Language::Czech),
            "hu" => Some(Language::Hungarian),
            "ro" => Some(Language::Romanian),
            "el" => Some(Language::Greek),
            "he" => Some(Language::Hebrew),
            "fa" => Some(Language::Persian),
            "uk" => Some(Language::Ukrainian),
            _ => None,
        }
    }

    /// Get all available languages
    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::ChineseSimplified,
            Language::ChineseTraditional,
            Language::Spanish,
            Language::French,
            Language::German,
            Language::Japanese,
            Language::Korean,
            Language::Portuguese,
            Language::Russian,
            Language::Italian,
            Language::Dutch,
            Language::Polish,
            Language::Turkish,
            Language::Arabic,
            Language::Hindi,
            Language::Bengali,
            Language::Vietnamese,
            Language::Thai,
            Language::Indonesian,
            Language::Swedish,
            Language::Norwegian,
            Language::Danish,
            Language::Finnish,
            Language::Czech,
            Language::Hungarian,
            Language::Romanian,
            Language::Greek,
            Language::Hebrew,
            Language::Persian,
            Language::Ukrainian,
        ]
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

impl Default for TextDirection {
    fn default() -> Self {
        TextDirection::LeftToRight
    }
}
