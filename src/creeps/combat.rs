use bevy::prelude::*;

use super::{Creep, CreepAction};
use crate::factions::DiplomacyState;

pub fn creep_combat_system(mut creeps: Query<&mut Creep>, diplomacy: Res<DiplomacyState>) {
    // Simple combat: creeps attack enemies in range
    // Collect attacker info first
    let attackers: Vec<_> = creeps
        .iter()
        .filter_map(|c| {
            if let Some(CreepAction::Attack { target_id }) =
                c.current_action.as_ref().map(|a| &a.action)
            {
                Some((c.id, c.position, c.faction_id, *target_id))
            } else {
                None
            }
        })
        .collect();

    // Now apply damage
    for (_attacker_id, attacker_pos, attacker_faction, target_id) in attackers {
        for mut defender in creeps.iter_mut() {
            if defender.id == target_id {
                // Check if hostile
                if diplomacy.is_hostile(attacker_faction, defender.faction_id) {
                    // Check range (adjacent)
                    let distance = attacker_pos.distance(&defender.position);
                    if distance <= 1.5 {
                        // Attack - get attacker's attack power
                        let _damage = 10.0; // Default damage
                        defender.take_damage(_damage);
                    }
                }
                break;
            }
        }
    }
}
