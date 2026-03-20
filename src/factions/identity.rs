//! Faction identity system - unique names, leaders, and colors for each faction.

use std::collections::HashMap;
use bevy::prelude::*;

use crate::factions::FactionId;
use crate::ai::{AiPersonality, AiDifficulty};

/// Faction identity with unique name and leader
#[derive(Debug, Clone)]
pub struct FactionIdentity {
    pub faction_id: FactionId,
    pub name: String,
    pub leader_name: String,
    pub leader_title: String,
    pub personality: AiPersonality,
    pub difficulty: AiDifficulty,
    pub motto: String,
    pub color_hex: String,
}

impl FactionIdentity {
    /// Create a unique identity for a faction based on its ID
    pub fn generate(faction_id: FactionId) -> Self {
        let id = faction_id.0 as usize;
        
        let color_hex = generate_faction_color_hex(faction_id);
        
        if let Some((name, leader_name, leader_title, personality, motto)) = FACTION_DATA.get(id) {
            Self {
                faction_id,
                name: name.to_string(),
                leader_name: leader_name.to_string(),
                leader_title: leader_title.to_string(),
                personality: *personality,
                difficulty: determine_difficulty(id),
                motto: motto.to_string(),
                color_hex,
            }
        } else {
            Self {
                faction_id,
                name: format!("Faction {}", id),
                leader_name: "Unknown Leader".to_string(),
                leader_title: "The Nameless".to_string(),
                personality: AiPersonality::Balanced,
                difficulty: determine_difficulty(id),
                motto: "We endure.".to_string(),
                color_hex,
            }
        }
    }
}



/// Determine AI difficulty based on faction ID (varies for variety)
fn determine_difficulty(id: usize) -> AiDifficulty {
    match id % 8 {
        0 => AiDifficulty::Hard,
        1 | 2 | 3 => AiDifficulty::Normal,
        4 | 5 | 6 => AiDifficulty::Easy,
        7 => AiDifficulty::Nightmare,
        _ => AiDifficulty::Normal,
    }
}

/// Generate a hex color string for a faction
fn generate_faction_color_hex(faction_id: FactionId) -> String {
    let hue = (faction_id.0 as f32 / 32.0) * 360.0;
    let (r, g, b) = hsl_to_rgb(hue, 0.7, 0.5);
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// HSL to RGB conversion
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

/// Faction data: (name, leader_name, leader_title, personality, motto)
const FACTION_DATA: [(&str, &str, &str, AiPersonality, &str); 32] = [
    // Faction 00 - Aggressive warmonger
    ("Iron Legion", "General Kael", "the Iron Fist", AiPersonality::Aggressive, "Steel resolves all conflicts."),
    // Faction 01 - Economic powerhouse
    ("Golden Consortium", "Director Vex", "the Golden", AiPersonality::Economic, "Wealth is power."),
    // Faction 02 - Scientific researchers
    ("Azure Academy", "Scholar Mira", "the Enlightened", AiPersonality::Scientific, "Knowledge conquers all."),
    // Faction 03 - Diplomatic alliance
    ("Silver Covenant", "Ambassador Thane", "the Peacemaker", AiPersonality::Diplomatic, "Unity through words."),
    // Faction 04 - Defensive fortress
    ("Obsidian Guard", "Commander Zara", "the Unyielding", AiPersonality::Defensive, "We do not retreat."),
    // Faction 05 - Aggressive raiders
    ("Crimson Raiders", "Warlord Gorth", "the Bloodthirsty", AiPersonality::Aggressive, "Take what you can."),
    // Faction 06 - Economic traders
    ("Emerald Traders", "Merchant Prince Olin", "the Wealthy", AiPersonality::Economic, "Everything has a price."),
    // Faction 07 - Scientific experimenters
    ("Violet Institute", "Dr. Synthia", "the Innovator", AiPersonality::Scientific, "Progress through experiment."),
    // Faction 08 - Diplomatic negotiators
    ("Pearl Assembly", "Chancellor Eve", "the Mediator", AiPersonality::Diplomatic, "Peace through negotiation."),
    // Faction 09 - Defensive protectors
    ("Granite Sentinels", "Warden Thorpe", "the Guardian", AiPersonality::Defensive, "None shall pass."),
    // Faction 10 - Aggressive conquerors
    ("Flame Legion", "Blaze Commander Rex", "the Burning", AiPersonality::Aggressive, "Purge the weak."),
    // Faction 11 - Economic industrialists
    ("Bronze Foundry", "Industrialist Kane", "the Builder", AiPersonality::Economic, "Production is survival."),
    // Faction 12 - Scientific technologists
    ("Cyan Collective", "Techno-Architect Neo", "the Visionary", AiPersonality::Scientific, "Technology shapes destiny."),
    // Faction 13 - Diplomatic union
    ("Ivory Union", "High Councilor Amis", "the Wise", AiPersonality::Diplomatic, "Together we rise."),
    // Faction 14 - Defensive stronghold
    ("Titan Fortress", "Siege Master Holt", "the Immovable", AiPersonality::Defensive, "Endurance is victory."),
    // Faction 15 - Isolationist (Balanced leaning defensive)
    ("Onyx Enclave", "Keeper Shale", "the Silent", AiPersonality::Defensive, "We endure in shadow."),
    // Faction 16 - Aggressive berserkers
    ("Storm Reavers", "Storm Caller Drax", "the Furious", AiPersonality::Aggressive, "Let chaos reign."),
    // Faction 17 - Economic merchants
    ("Sapphire Syndicate", "Trade Master Lira", "the Prosperous", AiPersonality::Economic, "Growth through trade."),
    // Faction 18 - Scientific researchers
    ("Amber Observatory", "Star Gazer Phin", "the Seeker", AiPersonality::Scientific, "The stars guide us."),
    // Faction 19 - Diplomatic federation
    ("Jade Federation", "Prime Minister Chen", "the Unifier", AiPersonality::Diplomatic, "Strength in numbers."),
    // Faction 20 - Defensive bastion
    ("Marble Bastion", "Shield Captain Vera", "the Defender", AiPersonality::Defensive, "Stand as one."),
    // Faction 21 - Aggressive horde
    ("Savage Horde", "Chieftain Krug", "the Destroyer", AiPersonality::Aggressive, "No mercy, no fear."),
    // Faction 22 - Economic entrepreneurs
    ("Topaz Ventures", "Venture Capitalist Max", "the Investor", AiPersonality::Economic, "Invest in the future."),
    // Faction 23 - Scientific pioneers
    ("Quartz Explorers", "Explorer Captain Nix", "the Pioneer", AiPersonality::Scientific, "Beyond the horizon."),
    // Faction 24 - Diplomatic coalition
    ("Lapis Coalition", "Coalition Head Rya", "the Bridge Builder", AiPersonality::Diplomatic, "Connect and prosper."),
    // Faction 25 - Defensive wardens
    ("Crystal Wardens", "Crystal Keeper Ion", "the Protector", AiPersonality::Defensive, "Shield the innocent."),
    // Faction 26 - Balanced strategists
    ("Platinum Strategists", "Grand Strategist Kai", "the Calculated", AiPersonality::Balanced, "Every move matters."),
    // Faction 27 - Aggressive crusaders
    ("Scarlet Crusade", "Crusader Lord Vorn", "the Zealot", AiPersonality::Aggressive, "Victory through faith."),
    // Faction 28 - Economic miners
    ("Copper Miners Guild", "Guild Master Flint", "the Deep Digger", AiPersonality::Economic, "From earth, wealth."),
    // Faction 29 - Scientific biologists
    ("Verdant Labs", "Chief Botanist Ivy", "the Cultivator", AiPersonality::Scientific, "Life finds a way."),
    // Faction 30 - Diplomatic peacemakers
    ("White Dove Society", "Peacekeeper Luna", "the Hope", AiPersonality::Diplomatic, "War solves nothing."),
    // Faction 31 - Balanced survivors
    ("Survivor Collective", "Survivor Chief Echo", "the Resilient", AiPersonality::Balanced, "We adapt. We survive."),
];

/// Resource holding all faction identities
#[derive(Resource, Debug, Clone, Default)]
pub struct FactionIdentities {
    pub identities: HashMap<FactionId, FactionIdentity>,
}

impl FactionIdentities {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize identities for all factions
    pub fn initialize(&mut self, faction_count: usize) {
        for i in 0..faction_count {
            let faction_id = FactionId(i as u16);
            let identity = FactionIdentity::generate(faction_id);
            self.identities.insert(faction_id, identity);
        }
    }

    /// Get identity for a faction
    pub fn get(&self, faction_id: FactionId) -> Option<&FactionIdentity> {
        self.identities.get(&faction_id)
    }

    /// Get leader name for a faction
    pub fn leader_name(&self, faction_id: FactionId) -> &str {
        self.get(faction_id)
            .map(|i| i.leader_name.as_str())
            .unwrap_or("Unknown")
    }

    /// Get faction name
    pub fn faction_name(&self, faction_id: FactionId) -> &str {
        self.get(faction_id)
            .map(|i| i.name.as_str())
            .unwrap_or("Unknown Faction")
    }
}
