use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct RepairBay {
    pub repair_rate: f32,
    pub range: f32,
}

impl Default for RepairBay {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairBay {
    pub fn new() -> Self {
        Self {
            repair_rate: 10.0,
            range: 3.0,
        }
    }
}