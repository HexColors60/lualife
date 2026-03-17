use bevy::prelude::*;

use crate::creeps::Creep;
use crate::factions::FactionRegistry;

pub fn creep_render_system(
    creeps: Query<&Creep>,
    factions: Res<FactionRegistry>,
) {
    for creep in creeps.iter() {
        // Get faction color
        let color = factions
            .get(creep.faction_id)
            .map(|f| f.color)
            .unwrap_or((255, 255, 255));

        // Placeholder: would render creep sprite/icon
        tracing::trace!(
            "Creep {} at ({}, {}) with color {:?}",
            creep.id,
            creep.position.x,
            creep.position.y,
            color
        );
    }
}