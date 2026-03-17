use bevy::prelude::*;
use std::collections::HashMap;

use super::{Language, LocalizationSettings};

/// Translation key type
pub type TranslationKey = String;

/// Translation value type
pub type TranslationValue = String;

/// Translation registry resource
#[derive(Debug, Clone, Resource, Default)]
pub struct TranslationRegistry {
    translations: HashMap<Language, HashMap<TranslationKey, TranslationValue>>,
    namespaces: HashMap<String, Vec<TranslationKey>>,
}

impl TranslationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register translations for a language
    pub fn register(
        &mut self,
        language: Language,
        translations: HashMap<TranslationKey, TranslationValue>,
    ) {
        self.translations.insert(language, translations);
    }

    /// Register translations for a namespace
    pub fn register_namespace(
        &mut self,
        language: Language,
        namespace: &str,
        translations: HashMap<TranslationKey, TranslationValue>,
    ) {
        let prefixed: HashMap<TranslationKey, TranslationValue> = translations
            .into_iter()
            .map(|(k, v)| (format!("{}.{}", namespace, k), v))
            .collect();

        let keys: Vec<TranslationKey> = prefixed.keys().cloned().collect();
        self.namespaces
            .entry(namespace.to_string())
            .or_default()
            .extend(keys);

        self.translations
            .entry(language)
            .or_default()
            .extend(prefixed);
    }

    /// Get a translation
    pub fn get(&self, language: Language, key: &str) -> Option<&TranslationValue> {
        self.translations.get(&language).and_then(|t| t.get(key))
    }

    /// Get a translation with fallback
    pub fn get_with_fallback<'a>(
        &'a self,
        settings: &LocalizationSettings,
        key: &'a str,
    ) -> &'a str {
        self.get(settings.current_language, key)
            .or_else(|| self.get(settings.fallback_language, key))
            .map(|s| s.as_str())
            .unwrap_or(key)
    }

    /// Check if a translation exists
    pub fn has(&self, language: Language, key: &str) -> bool {
        self.translations
            .get(&language)
            .map(|t| t.contains_key(key))
            .unwrap_or(false)
    }

    /// Get all keys for a language
    pub fn keys(&self, language: Language) -> Vec<&TranslationKey> {
        self.translations
            .get(&language)
            .map(|t| t.keys().collect())
            .unwrap_or_default()
    }

    /// Get all translations for a language
    pub fn all_translations(
        &self,
        language: Language,
    ) -> Option<&HashMap<TranslationKey, TranslationValue>> {
        self.translations.get(&language)
    }

    /// Clear all translations
    pub fn clear(&mut self) {
        self.translations.clear();
        self.namespaces.clear();
    }

    /// Get loaded languages
    pub fn loaded_languages(&self) -> Vec<Language> {
        self.translations.keys().copied().collect()
    }
}

/// Trait for localizable strings
pub trait Localizable {
    fn localize(&self, registry: &TranslationRegistry, settings: &LocalizationSettings) -> String;
}

impl Localizable for &str {
    fn localize(&self, registry: &TranslationRegistry, settings: &LocalizationSettings) -> String {
        registry.get_with_fallback(settings, self).to_string()
    }
}

impl Localizable for String {
    fn localize(&self, registry: &TranslationRegistry, settings: &LocalizationSettings) -> String {
        registry.get_with_fallback(settings, self).to_string()
    }
}

/// Localized string component
#[derive(Debug, Clone, Component)]
pub struct LocalizedText {
    pub key: String,
    pub params: HashMap<String, String>,
}

impl LocalizedText {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            params: HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn resolve(
        &self,
        registry: &TranslationRegistry,
        settings: &LocalizationSettings,
    ) -> String {
        let mut text = registry.get_with_fallback(settings, &self.key).to_string();

        // Replace parameters
        for (key, value) in &self.params {
            text = text.replace(&format!("{{{}}}", key), value);
        }

        text
    }
}

/// System to load translations
pub fn load_translations_system(
    mut registry: ResMut<TranslationRegistry>,
    mut state: ResMut<super::LocalizationState>,
) {
    if state.loaded {
        return;
    }

    // Load built-in translations
    load_builtin_translations(&mut registry);

    // Load translation files from disk
    if let Err(e) = load_translation_files(&mut registry) {
        tracing::warn!("Failed to load translation files: {}", e);
    }

    state.loaded = true;
    state.available_languages = registry.loaded_languages();
    tracing::info!("Loaded {} languages", state.available_languages.len());
}

/// Load built-in translations
fn load_builtin_translations(registry: &mut TranslationRegistry) {
    // English translations (default)
    let mut en = HashMap::new();
    en.insert("game.title".to_string(), "bevy_screeps_lua".to_string());
    en.insert("game.tick".to_string(), "Tick: {tick}".to_string());
    en.insert("game.paused".to_string(), "Paused".to_string());
    en.insert("game.speed".to_string(), "Speed: {speed}x".to_string());

    // UI
    en.insert("ui.minimap".to_string(), "Minimap".to_string());
    en.insert("ui.log".to_string(), "Log".to_string());
    en.insert("ui.resources".to_string(), "Resources".to_string());
    en.insert("ui.factions".to_string(), "Factions".to_string());
    en.insert("ui.tech_tree".to_string(), "Tech Tree".to_string());
    en.insert("ui.market".to_string(), "Market".to_string());
    en.insert("ui.diplomacy".to_string(), "Diplomacy".to_string());
    en.insert("ui.settings".to_string(), "Settings".to_string());
    en.insert("ui.save".to_string(), "Save".to_string());
    en.insert("ui.load".to_string(), "Load".to_string());
    en.insert("ui.quit".to_string(), "Quit".to_string());
    en.insert("ui.close".to_string(), "Close".to_string());
    en.insert("ui.cancel".to_string(), "Cancel".to_string());
    en.insert("ui.confirm".to_string(), "Confirm".to_string());
    en.insert("ui.yes".to_string(), "Yes".to_string());
    en.insert("ui.no".to_string(), "No".to_string());
    en.insert("ui.ok".to_string(), "OK".to_string());

    // Buildings
    en.insert("building.spawn".to_string(), "Spawn".to_string());
    en.insert("building.tower".to_string(), "Tower".to_string());
    en.insert("building.storage".to_string(), "Storage".to_string());
    en.insert("building.refinery".to_string(), "Refinery".to_string());
    en.insert(
        "building.research_lab".to_string(),
        "Research Lab".to_string(),
    );
    en.insert("building.market".to_string(), "Market".to_string());
    en.insert("building.road".to_string(), "Road".to_string());
    en.insert("building.wall".to_string(), "Wall".to_string());

    // Resources
    en.insert("resource.power".to_string(), "Power".to_string());
    en.insert("resource.iron".to_string(), "Iron".to_string());
    en.insert("resource.copper".to_string(), "Copper".to_string());
    en.insert("resource.silicon".to_string(), "Silicon".to_string());
    en.insert("resource.crystal".to_string(), "Crystal".to_string());
    en.insert("resource.carbon".to_string(), "Carbon".to_string());
    en.insert("resource.stone".to_string(), "Stone".to_string());
    en.insert("resource.sulfur".to_string(), "Sulfur".to_string());
    en.insert("resource.water".to_string(), "Water".to_string());
    en.insert("resource.biomass".to_string(), "Biomass".to_string());

    // Units
    en.insert("unit.creep".to_string(), "Creep".to_string());
    en.insert("unit.worker".to_string(), "Worker".to_string());
    en.insert("unit.fighter".to_string(), "Fighter".to_string());
    en.insert("unit.harvester".to_string(), "Harvester".to_string());
    en.insert("unit.builder".to_string(), "Builder".to_string());
    en.insert("unit.transport".to_string(), "Transport".to_string());

    // Actions
    en.insert("action.move".to_string(), "Move".to_string());
    en.insert("action.attack".to_string(), "Attack".to_string());
    en.insert("action.mine".to_string(), "Mine".to_string());
    en.insert("action.build".to_string(), "Build".to_string());
    en.insert("action.transfer".to_string(), "Transfer".to_string());
    en.insert("action.repair".to_string(), "Repair".to_string());

    // Messages
    en.insert("msg.game_saved".to_string(), "Game saved".to_string());
    en.insert("msg.game_loaded".to_string(), "Game loaded".to_string());
    en.insert(
        "msg.building_placed".to_string(),
        "Building placed".to_string(),
    );
    en.insert("msg.unit_spawned".to_string(), "Unit spawned".to_string());
    en.insert(
        "msg.research_complete".to_string(),
        "Research complete: {tech}".to_string(),
    );
    en.insert(
        "msg.enemy_detected".to_string(),
        "Enemy detected!".to_string(),
    );
    en.insert("msg.under_attack".to_string(), "Under attack!".to_string());
    en.insert(
        "msg.resources_low".to_string(),
        "Resources low: {resource}".to_string(),
    );

    // Accessibility
    en.insert(
        "accessibility.settings".to_string(),
        "Accessibility Settings".to_string(),
    );
    en.insert(
        "accessibility.screen_reader".to_string(),
        "Screen Reader".to_string(),
    );
    en.insert(
        "accessibility.colorblind_mode".to_string(),
        "Colorblind Mode".to_string(),
    );
    en.insert(
        "accessibility.high_contrast".to_string(),
        "High Contrast".to_string(),
    );
    en.insert("accessibility.ui_scale".to_string(), "UI Scale".to_string());

    registry.register(Language::English, en);

    // Chinese Simplified translations
    let mut zh_cn = HashMap::new();
    zh_cn.insert("game.title".to_string(), "bevy_screeps_lua".to_string());
    zh_cn.insert("game.tick".to_string(), "回合: {tick}".to_string());
    zh_cn.insert("game.paused".to_string(), "已暂停".to_string());
    zh_cn.insert("game.speed".to_string(), "速度: {speed}x".to_string());

    zh_cn.insert("ui.minimap".to_string(), "小地图".to_string());
    zh_cn.insert("ui.log".to_string(), "日志".to_string());
    zh_cn.insert("ui.resources".to_string(), "资源".to_string());
    zh_cn.insert("ui.factions".to_string(), "派系".to_string());
    zh_cn.insert("ui.tech_tree".to_string(), "科技树".to_string());
    zh_cn.insert("ui.market".to_string(), "市场".to_string());
    zh_cn.insert("ui.diplomacy".to_string(), "外交".to_string());
    zh_cn.insert("ui.settings".to_string(), "设置".to_string());
    zh_cn.insert("ui.save".to_string(), "保存".to_string());
    zh_cn.insert("ui.load".to_string(), "加载".to_string());
    zh_cn.insert("ui.quit".to_string(), "退出".to_string());
    zh_cn.insert("ui.close".to_string(), "关闭".to_string());
    zh_cn.insert("ui.cancel".to_string(), "取消".to_string());
    zh_cn.insert("ui.confirm".to_string(), "确认".to_string());
    zh_cn.insert("ui.yes".to_string(), "是".to_string());
    zh_cn.insert("ui.no".to_string(), "否".to_string());
    zh_cn.insert("ui.ok".to_string(), "确定".to_string());

    zh_cn.insert("building.spawn".to_string(), "孵化器".to_string());
    zh_cn.insert("building.tower".to_string(), "炮塔".to_string());
    zh_cn.insert("building.storage".to_string(), "仓库".to_string());
    zh_cn.insert("building.refinery".to_string(), "精炼厂".to_string());
    zh_cn.insert("building.research_lab".to_string(), "研究所".to_string());
    zh_cn.insert("building.market".to_string(), "市场".to_string());
    zh_cn.insert("building.road".to_string(), "道路".to_string());
    zh_cn.insert("building.wall".to_string(), "墙壁".to_string());

    zh_cn.insert("resource.power".to_string(), "能量".to_string());
    zh_cn.insert("resource.iron".to_string(), "铁".to_string());
    zh_cn.insert("resource.copper".to_string(), "铜".to_string());
    zh_cn.insert("resource.silicon".to_string(), "硅".to_string());
    zh_cn.insert("resource.crystal".to_string(), "水晶".to_string());
    zh_cn.insert("resource.carbon".to_string(), "碳".to_string());
    zh_cn.insert("resource.stone".to_string(), "石头".to_string());
    zh_cn.insert("resource.sulfur".to_string(), "硫磺".to_string());
    zh_cn.insert("resource.water".to_string(), "水".to_string());
    zh_cn.insert("resource.biomass".to_string(), "生物质".to_string());

    zh_cn.insert("unit.creep".to_string(), "爬虫".to_string());
    zh_cn.insert("unit.worker".to_string(), "工人".to_string());
    zh_cn.insert("unit.fighter".to_string(), "战士".to_string());
    zh_cn.insert("unit.harvester".to_string(), "采集者".to_string());
    zh_cn.insert("unit.builder".to_string(), "建造者".to_string());
    zh_cn.insert("unit.transport".to_string(), "运输者".to_string());

    zh_cn.insert("action.move".to_string(), "移动".to_string());
    zh_cn.insert("action.attack".to_string(), "攻击".to_string());
    zh_cn.insert("action.mine".to_string(), "采集".to_string());
    zh_cn.insert("action.build".to_string(), "建造".to_string());
    zh_cn.insert("action.transfer".to_string(), "转移".to_string());
    zh_cn.insert("action.repair".to_string(), "修复".to_string());

    zh_cn.insert("msg.game_saved".to_string(), "游戏已保存".to_string());
    zh_cn.insert("msg.game_loaded".to_string(), "游戏已加载".to_string());
    zh_cn.insert("msg.building_placed".to_string(), "建筑已放置".to_string());
    zh_cn.insert("msg.unit_spawned".to_string(), "单位已生成".to_string());
    zh_cn.insert(
        "msg.research_complete".to_string(),
        "研究完成: {tech}".to_string(),
    );
    zh_cn.insert("msg.enemy_detected".to_string(), "发现敌人!".to_string());
    zh_cn.insert("msg.under_attack".to_string(), "正在被攻击!".to_string());
    zh_cn.insert(
        "msg.resources_low".to_string(),
        "资源不足: {resource}".to_string(),
    );

    zh_cn.insert(
        "accessibility.settings".to_string(),
        "无障碍设置".to_string(),
    );
    zh_cn.insert(
        "accessibility.screen_reader".to_string(),
        "屏幕阅读器".to_string(),
    );
    zh_cn.insert(
        "accessibility.colorblind_mode".to_string(),
        "色盲模式".to_string(),
    );
    zh_cn.insert(
        "accessibility.high_contrast".to_string(),
        "高对比度".to_string(),
    );
    zh_cn.insert("accessibility.ui_scale".to_string(), "界面缩放".to_string());

    registry.register(Language::ChineseSimplified, zh_cn);
}

/// Load translation files from disk
fn load_translation_files(registry: &mut TranslationRegistry) -> Result<(), String> {
    let translations_dir = std::path::Path::new("assets/translations");

    if !translations_dir.exists() {
        return Ok(());
    }

    let entries = std::fs::read_dir(translations_dir)
        .map_err(|e| format!("Failed to read translations directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "ron").unwrap_or(false) {
            if let Some(stem) = path.file_stem() {
                if let Some(lang) = Language::from_iso_code(&stem.to_string_lossy()) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(translations) = ron::from_str::<HashMap<String, String>>(&content)
                        {
                            registry.register(lang, translations);
                            tracing::info!("Loaded translations for {:?}", lang);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// System to update locale
pub fn update_locale_system(
    settings: Res<LocalizationSettings>,
    mut registry: ResMut<TranslationRegistry>,
) {
    if settings.is_changed() {
        tracing::info!("Locale changed to {:?}", settings.current_language);
    }
}
