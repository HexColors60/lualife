use bevy::prelude::*;

use super::Creep;
use crate::consts::CREEP_POWER_CONSUMPTION;

pub fn creep_upkeep_system(
    mut creeps: Query<&mut Creep>,
) {
    for mut creep in creeps.iter_mut() {
        // Consume power each tick
        let consumption = CREEP_POWER_CONSUMPTION * creep.body.eat_parts() as f32;
        creep.consume_power(consumption);

        // If out of power, take damage
        if creep.power_reserve <= 0.0 {
            creep.take_damage(1.0);
        }
    }
}