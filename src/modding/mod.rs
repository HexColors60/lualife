use bevy::prelude::*;
use ron::de::from_reader;
use ron::ser::to_string_pretty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Mod metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub game_version: String,
    pub dependencies: Vec<String>,
}

impl Default for ModInfo {
    fn default() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "Unknown Mod".to_string(),
            version: "1.0.0".to_string(),
            author: "Unknown".to_string(),
            description: "".to_string(),
            game_version: "0.1.0".to_string(),
            dependencies: vec![],
        }
    }
}

/// Custom building definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomBuilding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: HashMap<String, u32>,
    pub build_time: u32,
    pub max_hp: u32,
    pub size: (u32, u32),
    pub abilities: Vec<BuildingAbility>,
    pub requirements: Vec<String>,
}

/// Building ability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingAbility {
    pub ability_type: AbilityType,
    pub value: f32,
    pub range: u32,
}

/// Types of building abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityType {
    Storage { resource: String, capacity: u32 },
    Production { resource: String, rate: f32 },
    Defense { damage: f32 },
    Repair { rate: f32 },
    Spawn { creep_type: String },
    Research { tech_bonus: f32 },
}

/// Custom resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomResource {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: ResourceRarity,
    pub base_value: u32,
    pub stack_size: u32,
    pub mineable: bool,
    pub mine_rate: f32,
}

/// Resource rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Custom creep body part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomBodyPart {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: HashMap<String, u32>,
    pub effects: Vec<BodyPartEffect>,
}

/// Body part effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPartEffect {
    pub effect_type: BodyEffectType,
    pub value: f32,
}

/// Types of body part effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyEffectType {
    MoveSpeed,
    WorkEfficiency,
    AttackDamage,
    AttackRange,
    MiningRate,
    BuildSpeed,
    CarryCapacity,
    HitPoints,
    PowerConsumption,
}

/// Custom tech definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTech {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: u32,
    pub cost: u32,
    pub research_time: u32,
    pub prerequisites: Vec<String>,
    pub unlocks: Vec<String>,
}

/// Mod content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModContent {
    pub buildings: Vec<CustomBuilding>,
    pub resources: Vec<CustomResource>,
    pub body_parts: Vec<CustomBodyPart>,
    pub techs: Vec<CustomTech>,
}

/// Loaded mod
#[derive(Debug, Clone)]
pub struct LoadedMod {
    pub info: ModInfo,
    pub content: ModContent,
    pub path: PathBuf,
    pub enabled: bool,
}

/// Mod manager
#[derive(Debug, Clone, Resource, Default)]
pub struct ModManager {
    pub loaded_mods: HashMap<String, LoadedMod>,
    pub load_order: Vec<String>,
    pub mods_directory: PathBuf,
}

impl ModManager {
    pub fn new() -> Self {
        Self {
            loaded_mods: HashMap::new(),
            load_order: Vec::new(),
            mods_directory: PathBuf::from("mods"),
        }
    }

    pub fn load_mod(&mut self, path: PathBuf) -> Result<String, String> {
        // Load mod info
        let info_path = path.join("mod.ron");
        let info: ModInfo = if info_path.exists() {
            let file = std::fs::File::open(&info_path)
                .map_err(|e| format!("Failed to open mod info: {}", e))?;
            from_reader(file).map_err(|e| format!("Failed to parse mod info: {}", e))?
        } else {
            ModInfo::default()
        };

        // Load mod content
        let content_path = path.join("content.ron");
        let content: ModContent = if content_path.exists() {
            let file = std::fs::File::open(&content_path)
                .map_err(|e| format!("Failed to open mod content: {}", e))?;
            from_reader(file).map_err(|e| format!("Failed to parse mod content: {}", e))?
        } else {
            ModContent::default()
        };

        let mod_id = info.id.clone();
        let loaded_mod = LoadedMod {
            info,
            content,
            path,
            enabled: true,
        };

        self.loaded_mods.insert(mod_id.clone(), loaded_mod);
        self.load_order.push(mod_id.clone());

        Ok(mod_id)
    }

    pub fn unload_mod(&mut self, mod_id: &str) -> bool {
        if let Some(_) = self.loaded_mods.remove(mod_id) {
            self.load_order.retain(|id| id != mod_id);
            true
        } else {
            false
        }
    }

    pub fn enable_mod(&mut self, mod_id: &str) -> bool {
        if let Some(mod_data) = self.loaded_mods.get_mut(mod_id) {
            mod_data.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_mod(&mut self, mod_id: &str) -> bool {
        if let Some(mod_data) = self.loaded_mods.get_mut(mod_id) {
            mod_data.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn get_all_buildings(&self) -> Vec<&CustomBuilding> {
        self.loaded_mods
            .values()
            .filter(|m| m.enabled)
            .flat_map(|m| &m.content.buildings)
            .collect()
    }

    pub fn get_all_resources(&self) -> Vec<&CustomResource> {
        self.loaded_mods
            .values()
            .filter(|m| m.enabled)
            .flat_map(|m| &m.content.resources)
            .collect()
    }

    pub fn get_all_body_parts(&self) -> Vec<&CustomBodyPart> {
        self.loaded_mods
            .values()
            .filter(|m| m.enabled)
            .flat_map(|m| &m.content.body_parts)
            .collect()
    }

    pub fn get_all_techs(&self) -> Vec<&CustomTech> {
        self.loaded_mods
            .values()
            .filter(|m| m.enabled)
            .flat_map(|m| &m.content.techs)
            .collect()
    }

    pub fn save_mod(&self, mod_id: &str) -> Result<(), String> {
        let mod_data = self.loaded_mods.get(mod_id).ok_or("Mod not found")?;

        let info_string = to_string_pretty(&mod_data.info, Default::default())
            .map_err(|e| format!("Failed to serialize mod info: {}", e))?;
        let content_string = to_string_pretty(&mod_data.content, Default::default())
            .map_err(|e| format!("Failed to serialize mod content: {}", e))?;

        std::fs::create_dir_all(&mod_data.path)
            .map_err(|e| format!("Failed to create mod directory: {}", e))?;

        std::fs::write(mod_data.path.join("mod.ron"), info_string)
            .map_err(|e| format!("Failed to write mod info: {}", e))?;
        std::fs::write(mod_data.path.join("content.ron"), content_string)
            .map_err(|e| format!("Failed to write mod content: {}", e))?;

        Ok(())
    }
}

/// Mod event
#[derive(Event, Debug, Clone)]
pub enum ModEvent {
    ModLoaded { mod_id: String, mod_name: String },
    ModUnloaded { mod_id: String },
    ModEnabled { mod_id: String },
    ModDisabled { mod_id: String },
    ModError { mod_id: String, error: String },
}

/// Workshop integration
#[derive(Debug, Clone, Resource, Default)]
pub struct WorkshopManager {
    pub published_mods: HashMap<String, WorkshopMod>,
    pub api_key: Option<String>,
}

/// Workshop mod entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkshopMod {
    pub workshop_id: String,
    pub mod_id: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub downloads: u32,
    pub rating: f32,
    pub updated: u64,
}

impl WorkshopManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish_mod(&mut self, _mod_id: &str) -> Result<String, String> {
        // Placeholder for workshop publishing
        Err("Workshop publishing not implemented".to_string())
    }

    pub fn subscribe_mod(&mut self, _workshop_id: &str) -> Result<(), String> {
        // Placeholder for workshop subscription
        Err("Workshop subscription not implemented".to_string())
    }

    pub fn unsubscribe_mod(&mut self, _workshop_id: &str) -> Result<(), String> {
        // Placeholder for workshop unsubscription
        Err("Workshop unsubscription not implemented".to_string())
    }

    pub fn update_mod(&mut self, _workshop_id: &str) -> Result<(), String> {
        // Placeholder for workshop update
        Err("Workshop update not implemented".to_string())
    }
}

/// System to load mods on startup
pub fn mod_loading_system(
    mut mod_manager: ResMut<ModManager>,
    mut events: EventWriter<ModEvent>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Create mods directory if it doesn't exist
    if !mod_manager.mods_directory.exists() {
        if let Err(e) = std::fs::create_dir_all(&mod_manager.mods_directory) {
            game_log.add(format!("Failed to create mods directory: {}", e));
            return;
        }
    }

    // Scan for mods
    if let Ok(entries) = std::fs::read_dir(&mod_manager.mods_directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                match mod_manager.load_mod(path.clone()) {
                    Ok(mod_id) => {
                        if let Some(loaded_mod) = mod_manager.loaded_mods.get(&mod_id) {
                            events.send(ModEvent::ModLoaded {
                                mod_id: mod_id.clone(),
                                mod_name: loaded_mod.info.name.clone(),
                            });
                            game_log.add(format!("Loaded mod: {}", loaded_mod.info.name));
                        }
                    }
                    Err(e) => {
                        game_log.add(format!("Failed to load mod: {}", e));
                    }
                }
            }
        }
    }
}

/// Plugin for modding system
pub struct ModdingPlugin;

impl Plugin for ModdingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModManager>()
            .init_resource::<WorkshopManager>()
            .add_event::<ModEvent>()
            .add_systems(Startup, mod_loading_system);
    }
}
