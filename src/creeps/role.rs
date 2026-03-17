use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CreepRole {
    #[default]
    Idle,
    Harvester,
    Builder,
    Fighter,
    Hauler,
    Scout,
    Upgrader,
    Repairer,
}

impl CreepRole {
    pub fn name(&self) -> &'static str {
        match self {
            CreepRole::Idle => "idle",
            CreepRole::Harvester => "harvester",
            CreepRole::Builder => "builder",
            CreepRole::Fighter => "fighter",
            CreepRole::Hauler => "hauler",
            CreepRole::Scout => "scout",
            CreepRole::Upgrader => "upgrader",
            CreepRole::Repairer => "repairer",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "idle" => Some(CreepRole::Idle),
            "harvester" => Some(CreepRole::Harvester),
            "builder" => Some(CreepRole::Builder),
            "fighter" => Some(CreepRole::Fighter),
            "hauler" => Some(CreepRole::Hauler),
            "scout" => Some(CreepRole::Scout),
            "upgrader" => Some(CreepRole::Upgrader),
            "repairer" => Some(CreepRole::Repairer),
            _ => None,
        }
    }
}
