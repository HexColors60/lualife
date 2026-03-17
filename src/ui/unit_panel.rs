use bevy::prelude::*;

use crate::creeps::Creep;
use crate::debug::SelectionState;

pub fn unit_panel_system(
    selection: Res<SelectionState>,
    creeps: Query<&Creep>,
) {
    if let Some(entity) = selection.selected_entity {
        if let Ok(creep) = creeps.get(entity) {
            // Display creep info
            tracing::debug!(
                "Selected creep: id={}, faction={:?}, hp={}/{}, power={}/{}",
                creep.id,
                creep.faction_id,
                creep.hp,
                creep.max_hp,
                creep.power_reserve,
                creep.max_power
            );
        }
    }
}