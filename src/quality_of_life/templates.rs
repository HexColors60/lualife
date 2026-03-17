use bevy::prelude::*;
use std::collections::HashMap;

/// Template manager resource
#[derive(Debug, Clone, Resource, Default)]
pub struct TemplateManager {
    pub templates: HashMap<String, ConstructionTemplate>,
    pub active_template: Option<String>,
}

/// Construction template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionTemplate {
    pub name: String,
    pub description: String,
    pub buildings: Vec<TemplateBuilding>,
    pub created_at: u64,
    pub category: TemplateCategory,
}

/// Building in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateBuilding {
    pub building_type: u8,
    pub offset_x: i32,
    pub offset_y: i32,
    pub rotation: u8,
}

/// Template category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    Base,
    Defense,
    Economy,
    Military,
    Custom,
}

/// Template event
#[derive(Debug, Clone, Event)]
pub enum TemplateEvent {
    Create(String, ConstructionTemplate),
    Delete(String),
    Apply(String, i32, i32),
    Save(String),
    Load(String),
    Rename(String, String),
}

impl TemplateManager {
    pub fn new() -> Self {
        let mut manager = Self {
            templates: HashMap::new(),
            active_template: None,
        };
        manager.load_default_templates();
        manager
    }

    fn load_default_templates(&mut self) {
        // Basic base template
        self.templates.insert(
            "basic_base".to_string(),
            ConstructionTemplate {
                name: "Basic Base".to_string(),
                description: "A simple starter base layout".to_string(),
                buildings: vec![
                    TemplateBuilding { building_type: 1, offset_x: 0, offset_y: 0, rotation: 0 }, // Spawn
                    TemplateBuilding { building_type: 3, offset_x: 2, offset_y: 0, rotation: 0 }, // Storage
                    TemplateBuilding { building_type: 4, offset_x: -2, offset_y: 0, rotation: 0 }, // Refinery
                    TemplateBuilding { building_type: 2, offset_x: 0, offset_y: 2, rotation: 0 }, // Tower
                    TemplateBuilding { building_type: 2, offset_x: 0, offset_y: -2, rotation: 0 }, // Tower
                ],
                created_at: 0,
                category: TemplateCategory::Base,
            },
        );

        // Defense line template
        self.templates.insert(
            "defense_line".to_string(),
            ConstructionTemplate {
                name: "Defense Line".to_string(),
                description: "A line of walls and towers".to_string(),
                buildings: vec![
                    TemplateBuilding { building_type: 8, offset_x: -3, offset_y: 0, rotation: 0 }, // Wall
                    TemplateBuilding { building_type: 8, offset_x: -2, offset_y: 0, rotation: 0 },
                    TemplateBuilding { building_type: 8, offset_x: -1, offset_y: 0, rotation: 0 },
                    TemplateBuilding { building_type: 2, offset_x: 0, offset_y: 0, rotation: 0 }, // Tower
                    TemplateBuilding { building_type: 8, offset_x: 1, offset_y: 0, rotation: 0 }, // Wall
                    TemplateBuilding { building_type: 8, offset_x: 2, offset_y: 0, rotation: 0 },
                    TemplateBuilding { building_type: 8, offset_x: 3, offset_y: 0, rotation: 0 },
                ],
                created_at: 0,
                category: TemplateCategory::Defense,
            },
        );

        // Mining outpost template
        self.templates.insert(
            "mining_outpost".to_string(),
            ConstructionTemplate {
                name: "Mining Outpost".to_string(),
                description: "A small mining operation".to_string(),
                buildings: vec![
                    TemplateBuilding { building_type: 3, offset_x: 0, offset_y: 0, rotation: 0 }, // Storage
                    TemplateBuilding { building_type: 4, offset_x: 2, offset_y: 0, rotation: 0 }, // Refinery
                    TemplateBuilding { building_type: 2, offset_x: 0, offset_y: 2, rotation: 0 }, // Tower
                ],
                created_at: 0,
                category: TemplateCategory::Economy,
            },
        );
    }

    /// Create a new template
    pub fn create(&mut self, name: String, template: ConstructionTemplate) {
        self.templates.insert(name.clone(), template);
    }

    /// Delete a template
    pub fn delete(&mut self, name: &str) -> bool {
        self.templates.remove(name).is_some()
    }

    /// Get a template by name
    pub fn get(&self, name: &str) -> Option<&ConstructionTemplate> {
        self.templates.get(name)
    }

    /// Get all template names
    pub fn get_names(&self) -> Vec<&String> {
        self.templates.keys().collect()
    }

    /// Get templates by category
    pub fn get_by_category(&self, category: TemplateCategory) -> Vec<&ConstructionTemplate> {
        self.templates.values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Set active template
    pub fn set_active(&mut self, name: Option<String>) {
        self.active_template = name;
    }

    /// Check if a template exists
    pub fn exists(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }

    /// Save templates to file
    pub fn save_to_file(&self) -> Result<(), String> {
        let content = ron::ser::to_string_pretty(&self.templates, ron::ser::PrettyConfig::default())
            .map_err(|e| e.to_string())?;
        std::fs::write("templates.ron", content)
            .map_err(|e| e.to_string())
    }

    /// Load templates from file
    pub fn load_from_file(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string("templates.ron")
            .map_err(|e| e.to_string())?;
        let templates: HashMap<String, ConstructionTemplate> = ron::from_str(&content)
            .map_err(|e| e.to_string())?;
        self.templates = templates;
        Ok(())
    }
}

/// System to handle templates
pub fn template_system(
    mut manager: ResMut<TemplateManager>,
    mut events: EventReader<TemplateEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        match event {
            TemplateEvent::Create(name, template) => {
                manager.create(name.clone(), template.clone());
                tracing::info!("Created template: {}", name);
            }
            TemplateEvent::Delete(name) => {
                if manager.delete(name) {
                    tracing::info!("Deleted template: {}", name);
                }
            }
            TemplateEvent::Apply(name, x, y) => {
                if let Some(template) = manager.get(name) {
                    tracing::info!("Applying template {} at ({}, {})", name, x, y);
                    // In real implementation, would place buildings
                }
            }
            TemplateEvent::Save(name) => {
                tracing::info!("Saving template: {}", name);
            }
            TemplateEvent::Load(name) => {
                tracing::info!("Loading template: {}", name);
            }
            TemplateEvent::Rename(old_name, new_name) => {
                if let Some(template) = manager.templates.remove(old_name) {
                    manager.templates.insert(new_name.clone(), template);
                    tracing::info!("Renamed template {} to {}", old_name, new_name);
                }
            }
        }
    }

    // Keyboard shortcuts for template rotation
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Rotate active template
    }
}

use serde::{Deserialize, Serialize};