use bevy::prelude::*;
use std::collections::HashMap;

use super::{Language, TranslationKey, TranslationValue};

/// Translation file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationFile {
    pub language: String,
    pub version: String,
    pub translations: HashMap<TranslationKey, TranslationValue>,
}

impl TranslationFile {
    pub fn new(language: Language) -> Self {
        Self {
            language: language.iso_code().to_string(),
            version: "1.0.0".to_string(),
            translations: HashMap::new(),
        }
    }

    pub fn with_translation(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.translations.insert(key.into(), value.into());
        self
    }

    pub fn load(content: &str) -> Result<Self, String> {
        ron::from_str(content).map_err(|e| format!("Failed to parse translation file: {}", e))
    }

    pub fn save(&self) -> Result<String, String> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Failed to serialize translation file: {}", e))
    }
}

/// Translation namespace for organizing translations
#[derive(Debug, Clone, Resource, Default)]
pub struct TranslationNamespaces {
    namespaces: HashMap<String, Vec<TranslationKey>>,
}

impl TranslationNamespaces {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, namespace: &str, keys: Vec<TranslationKey>) {
        self.namespaces.insert(namespace.to_string(), keys);
    }

    pub fn get_keys(&self, namespace: &str) -> Option<&Vec<TranslationKey>> {
        self.namespaces.get(namespace)
    }

    pub fn get_namespaces(&self) -> Vec<&String> {
        self.namespaces.keys().collect()
    }
}

/// Plural forms for translations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralForm {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

impl PluralForm {
    /// Get plural form for a count based on language rules
    pub fn from_count(count: u64, language: Language) -> Self {
        // Simplified plural rules - a full implementation would use CLDR data
        match language {
            // Languages with simple singular/plural
            Language::English
            | Language::German
            | Language::Dutch
            | Language::Swedish
            | Language::Norwegian
            | Language::Danish
            | Language::Finnish
            | Language::Greek
            | Language::Italian
            | Language::Portuguese
            | Language::Spanish => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }

            // Languages with no plural distinction
            Language::ChineseSimplified
            | Language::ChineseTraditional
            | Language::Japanese
            | Language::Korean
            | Language::Vietnamese
            | Language::Thai => PluralForm::Other,

            // Russian, Ukrainian, Polish have complex plural rules
            Language::Russian | Language::Ukrainian | Language::Polish | Language::Czech => {
                let mod10 = count % 10;
                let mod100 = count % 100;

                if mod10 == 1 && mod100 != 11 {
                    PluralForm::One
                } else if (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) {
                    PluralForm::Few
                } else {
                    PluralForm::Many
                }
            }

            // Arabic has all plural forms
            Language::Arabic => {
                let mod100 = count % 100;

                if count == 0 {
                    PluralForm::Zero
                } else if count == 1 {
                    PluralForm::One
                } else if count == 2 {
                    PluralForm::Two
                } else if (3..=10).contains(&mod100) {
                    PluralForm::Few
                } else if mod100 >= 11 && mod100 <= 99 {
                    PluralForm::Many
                } else {
                    PluralForm::Other
                }
            }

            // Default to simple singular/plural
            _ => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
        }
    }
}

/// Get plural key suffix
pub fn plural_key_suffix(form: PluralForm) -> &'static str {
    match form {
        PluralForm::Zero => "_zero",
        PluralForm::One => "_one",
        PluralForm::Two => "_two",
        PluralForm::Few => "_few",
        PluralForm::Many => "_many",
        PluralForm::Other => "_other",
    }
}

/// Get translation with plural support
pub fn get_plural_translation(
    registry: &super::TranslationRegistry,
    settings: &super::LocalizationSettings,
    key: &str,
    count: u64,
) -> String {
    let plural_form = PluralForm::from_count(count, settings.current_language);
    let plural_key = format!("{}{}", key, plural_key_suffix(plural_form));

    // Try plural key first
    if let Some(translation) = registry.get(settings.current_language, &plural_key) {
        return translation.replace("{count}", &count.to_string());
    }

    // Fall back to base key
    if let Some(translation) = registry.get(settings.current_language, key) {
        return translation.replace("{count}", &count.to_string());
    }

    // Fall back to fallback language
    if let Some(translation) = registry.get(settings.fallback_language, &plural_key) {
        return translation.replace("{count}", &count.to_string());
    }

    if let Some(translation) = registry.get(settings.fallback_language, key) {
        return translation.replace("{count}", &count.to_string());
    }

    key.to_string()
}

use serde::{Deserialize, Serialize};
