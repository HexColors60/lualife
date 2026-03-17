use bevy::prelude::*;

use crate::creeps::Creep;

pub fn healthbar_render_system(
    creeps: Query<&Creep>,
) {
    for creep in creeps.iter() {
        let hp_ratio = creep.hp / creep.max_hp;
        let power_ratio = creep.power_reserve / creep.max_power;

        // Placeholder: would render health/power bars above creep
        tracing::trace!(
            "Creep {} HP: {:.0}% Power: {:.0}%",
            creep.id,
            hp_ratio * 100.0,
            power_ratio * 100.0
        );
    }
}