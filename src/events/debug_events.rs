use bevy::prelude::*;

use crate::factions::FactionId;

#[derive(Event, Debug, Clone)]
pub enum DebugEvent {
    GodModeToggled(bool),
    PauseToggled,
    SpeedChanged(f32),
    StepRequested(u32),
    ReloadScriptsRequested { faction_id: Option<FactionId> },
    SpawnCreepRequested { faction_id: FactionId, x: i32, y: i32 },
    SpawnBuildingRequested { faction_id: FactionId, building_type: String, x: i32, y: i32 },
    TeleportRequested { entity: Entity, x: i32, y: i32 },
    InspectEntity(Entity),
}