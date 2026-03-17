use bevy::prelude::*;
use std::collections::HashMap;

/// Help encyclopedia resource
#[derive(Resource, Debug, Clone, Default)]
pub struct HelpEncyclopedia {
    pub entries: HashMap<String, HelpEntry>,
    pub categories: Vec<HelpCategory>,
    pub search_query: String,
    pub current_entry: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HelpEntry {
    pub id: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub related: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HelpCategory {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

impl HelpEncyclopedia {
    pub fn new() -> Self {
        let mut encyclopedia = Self {
            entries: HashMap::new(),
            categories: Vec::new(),
            search_query: String::new(),
            current_entry: None,
        };

        encyclopedia.load_default_content();
        encyclopedia
    }

    fn load_default_content(&mut self) {
        // Define categories
        self.categories = vec![
            HelpCategory {
                id: "getting_started".to_string(),
                name: "Getting Started".to_string(),
                icon: Some("📖".to_string()),
            },
            HelpCategory {
                id: "units".to_string(),
                name: "Units".to_string(),
                icon: Some("👤".to_string()),
            },
            HelpCategory {
                id: "buildings".to_string(),
                name: "Buildings".to_string(),
                icon: Some("🏠".to_string()),
            },
            HelpCategory {
                id: "resources".to_string(),
                name: "Resources".to_string(),
                icon: Some("💎".to_string()),
            },
            HelpCategory {
                id: "combat".to_string(),
                name: "Combat".to_string(),
                icon: Some("⚔️".to_string()),
            },
            HelpCategory {
                id: "economy".to_string(),
                name: "Economy".to_string(),
                icon: Some("💰".to_string()),
            },
            HelpCategory {
                id: "technology".to_string(),
                name: "Technology".to_string(),
                icon: Some("🔬".to_string()),
            },
            HelpCategory {
                id: "diplomacy".to_string(),
                name: "Diplomacy".to_string(),
                icon: Some("🤝".to_string()),
            },
            HelpCategory {
                id: "controls".to_string(),
                name: "Controls".to_string(),
                icon: Some("🎮".to_string()),
            },
        ];

        // Getting Started entries
        self.add_entry(HelpEntry {
            id: "overview".to_string(),
            title: "Game Overview".to_string(),
            category: "getting_started".to_string(),
            content: r#"# Lualife Overview

Lualife is a Screeps-like autonomous simulation game where AI factions control units using Lua scripts.

## Your Role
You are in **GOD mode** - you can observe and inspect everything, but units are controlled by AI scripts.

## The World
- 256x256 tile world map
- Divided into 32x32 rooms (8x8 tiles each)
- Each room has terrain, mines, and resources

## AI Factions
- 32 AI factions compete for resources
- Each faction runs Lua scripts from ai/ai_xx/ folders
- Scripts control unit behavior automatically

## Goal
Watch the AI factions compete, or modify their scripts to change their behavior!"#.to_string(),
            related: vec!["controls".to_string(), "units".to_string()],
            tags: vec!["basics".to_string(), "intro".to_string()],
        });

        self.add_entry(HelpEntry {
            id: "controls".to_string(),
            title: "Controls".to_string(),
            category: "controls".to_string(),
            content: r#"# Controls

## Camera
- **WASD / Arrow Keys**: Pan camera
- **Mouse Scroll**: Zoom in/out
- **Middle Mouse Drag**: Pan camera

## Selection
- **Left Click**: Select unit/building
- **Right Click**: Context menu
- **Escape**: Deselect

## UI
- **M**: Toggle minimap
- **L**: Toggle log panel
- **T**: Tech tree
- **K**: Market
- **Shift+D**: Diplomacy

## Simulation
- **Space**: Pause/Resume
- **+/-**: Speed up/down
- **F5**: Quick save
- **F9**: Quick load

## Debug
- **F1**: Start tutorial
- **F3**: Debug overlay
- **F4**: Performance stats
- **F6**: Path visualization
- **F7**: Tower range display"#
                .to_string(),
            related: vec!["overview".to_string()],
            tags: vec!["controls".to_string(), "keyboard".to_string()],
        });

        // Units entries
        self.add_entry(HelpEntry {
            id: "creeps".to_string(),
            title: "Creeps".to_string(),
            category: "units".to_string(),
            content: r#"# Creeps

Creeps are the basic units in Lualife. They are autonomous agents controlled by AI scripts.

## Body Parts
- **MOVE**: Increases movement speed
- **WORK**: Allows harvesting and building
- **FIGHT**: Combat capability
- **MINE**: Mining efficiency
- **BUILD**: Construction speed
- **EAT**: Healing ability
- **TRANSPORT**: Carry capacity

## Roles
Creeps can be assigned different roles:
- **Worker**: General purpose
- **Harvester**: Resource gathering
- **Builder**: Construction
- **Fighter**: Combat
- **Transport**: Resource delivery

## Stats
- **HP**: Health points
- **Power**: Energy upkeep cost
- **Carry**: Current inventory"#
                .to_string(),
            related: vec!["buildings".to_string(), "resources".to_string()],
            tags: vec!["units".to_string(), "creeps".to_string()],
        });

        // Buildings entries
        self.add_entry(HelpEntry {
            id: "buildings".to_string(),
            title: "Buildings".to_string(),
            category: "buildings".to_string(),
            content: r#"# Buildings

Buildings provide various functions for factions.

## Types
- **Spawn**: Creates new creeps
- **Tower**: Automated defense
- **Storage**: Resource storage
- **Refinery**: Processes raw materials
- **Research Lab**: Technology research
- **Market**: Trading hub
- **Road**: Faster movement
- **Wall**: Defensive structure

## Construction
Buildings are placed by creeps with the BUILD body part. Construction requires resources and time.

## Upkeep
Most buildings require Power to operate. Without sufficient Power, buildings may shut down."#
                .to_string(),
            related: vec!["creeps".to_string(), "resources".to_string()],
            tags: vec!["buildings".to_string(), "construction".to_string()],
        });

        // Resources entries
        self.add_entry(HelpEntry {
            id: "resources".to_string(),
            title: "Resources".to_string(),
            category: "resources".to_string(),
            content: r#"# Resources

There are 10 resource types in Lualife:

## Primary Resources
- **Power**: Universal energy source, required for all operations
- **Iron**: Basic construction material
- **Copper**: Electronics and advanced components
- **Silicon**: Computing and tech components

## Advanced Resources
- **Crystal**: High-tech applications
- **Carbon**: Chemical processes
- **Stone**: Basic construction
- **Sulfur**: Explosives and chemicals
- **Water**: Various processes
- **Biomass**: Organic processes

## Gathering
Resources are gathered from mines by creeps with the MINE body part. Each mine type produces a specific resource."#.to_string(),
            related: vec!["creeps".to_string(), "economy".to_string()],
            tags: vec!["resources".to_string(), "mining".to_string()],
        });

        // Combat entries
        self.add_entry(HelpEntry {
            id: "combat".to_string(),
            title: "Combat".to_string(),
            category: "combat".to_string(),
            content: r#"# Combat

Combat occurs when hostile units or buildings interact.

## Combat Mechanics
- Units with FIGHT parts can attack
- Towers automatically attack enemies in range
- Damage is calculated based on attack power vs defense

## Defense
- Walls block movement and absorb damage
- Towers provide area defense
- Allied units can defend together

## Tactics
- Overwhelm with numbers
- Use terrain advantages
- Target weak units first
- Protect your economy"#
                .to_string(),
            related: vec!["creeps".to_string(), "buildings".to_string()],
            tags: vec!["combat".to_string(), "warfare".to_string()],
        });

        // Economy entries
        self.add_entry(HelpEntry {
            id: "economy".to_string(),
            title: "Economy".to_string(),
            category: "economy".to_string(),
            content: r#"# Economy

A strong economy is essential for faction success.

## Resource Management
- Balance resource gathering with consumption
- Stockpile resources for expansion
- Trade excess resources at the Market

## Upkeep
- All creeps consume Power
- Buildings may require Power to operate
- Insufficient Power leads to shutdown

## Growth
- Expand mining operations
- Build more storage capacity
- Research economic technologies
- Trade strategically"#
                .to_string(),
            related: vec!["resources".to_string(), "market".to_string()],
            tags: vec!["economy".to_string(), "management".to_string()],
        });

        // Technology entries
        self.add_entry(HelpEntry {
            id: "technology".to_string(),
            title: "Technology".to_string(),
            category: "technology".to_string(),
            content: r#"# Technology

Research new technologies to unlock abilities and improvements.

## Research
- Build Research Labs to conduct research
- Each technology requires resources and time
- Technologies unlock new capabilities

## Tech Tree
- Technologies have prerequisites
- Some technologies are mutually exclusive
- Plan your research path carefully

## Categories
- **Economy**: Resource efficiency
- **Military**: Combat improvements
- **Infrastructure**: Building upgrades
- **Special**: Unique abilities"#
                .to_string(),
            related: vec!["buildings".to_string(), "economy".to_string()],
            tags: vec!["technology".to_string(), "research".to_string()],
        });

        // Diplomacy entries
        self.add_entry(HelpEntry {
            id: "diplomacy".to_string(),
            title: "Diplomacy".to_string(),
            category: "diplomacy".to_string(),
            content: r#"# Diplomacy

Manage relations with other factions.

## Relations
- **Allied**: Share vision, no combat
- **Neutral**: Default state
- **Hostile**: Active combat

## Actions
- Propose alliances
- Declare war
- Trade agreements
- Non-aggression pacts

## Reputation
Your actions affect your reputation with all factions. High reputation enables better trade deals and alliances."#.to_string(),
            related: vec!["economy".to_string(), "combat".to_string()],
            tags: vec!["diplomacy".to_string(), "relations".to_string()],
        });
    }

    fn add_entry(&mut self, entry: HelpEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    pub fn get_entry(&self, id: &str) -> Option<&HelpEntry> {
        self.entries.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<&HelpEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|entry| {
                entry.title.to_lowercase().contains(&query_lower)
                    || entry.content.to_lowercase().contains(&query_lower)
                    || entry
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn get_by_category(&self, category: &str) -> Vec<&HelpEntry> {
        self.entries
            .values()
            .filter(|entry| entry.category == category)
            .collect()
    }
}

#[derive(Component)]
pub struct HelpDisplay;

pub fn help_encyclopedia_system(_encyclopedia: Res<HelpEncyclopedia>) {
    // System to handle help encyclopedia interactions
}
