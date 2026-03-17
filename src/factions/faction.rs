use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Unique identifier for a faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, PartialOrd, Ord)]
pub struct FactionId(pub u16);

impl std::fmt::Display for FactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Faction({})", self.0)
    }
}

impl From<u16> for FactionId {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub color: (u8, u8, u8),
    pub ai_path: String,
    pub active: bool,
}

impl Faction {
    pub fn new(id: FactionId, name: String, ai_path: String) -> Self {
        // Generate a color based on faction id
        let hue = (id.0 as f32 / 32.0) * 360.0;
        let color = hsl_to_rgb(hue, 0.7, 0.5);

        Self {
            id,
            name,
            color,
            ai_path,
            active: true,
        }
    }

    pub fn default_name(id: FactionId) -> String {
        format!("AI_{:02}", id.0)
    }

    pub fn default_path(id: FactionId) -> String {
        format!("ai/ai_{:02}", id.0)
    }
}

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

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

#[derive(Resource, Debug, Clone, Default)]
pub struct FactionRegistry {
    factions: HashMap<FactionId, Faction>,
    next_id: u16,
}

impl FactionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_faction(&mut self, name: Option<String>, ai_path: Option<String>) -> FactionId {
        let id = FactionId(self.next_id);
        self.next_id += 1;

        let faction = Faction::new(
            id,
            name.unwrap_or_else(|| Faction::default_name(id)),
            ai_path.unwrap_or_else(|| Faction::default_path(id)),
        );

        self.factions.insert(id, faction);
        id
    }

    pub fn get(&self, id: FactionId) -> Option<&Faction> {
        self.factions.get(&id)
    }

    pub fn get_mut(&mut self, id: FactionId) -> Option<&mut Faction> {
        self.factions.get_mut(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = &Faction> {
        self.factions.values()
    }

    pub fn count(&self) -> usize {
        self.factions.len()
    }

    pub fn initialize_default_factions(&mut self, count: usize) {
        for i in 0..count {
            let id = FactionId(i as u16);
            let faction = Faction::new(
                id,
                Faction::default_name(id),
                Faction::default_path(id),
            );
            self.factions.insert(id, faction);
        }
        self.next_id = count as u16;
    }
}