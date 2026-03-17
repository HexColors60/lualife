use bevy::prelude::*;

use crate::creeps::{Creep, CreepAction};
use crate::core::{GameState, TickNumber};
use crate::world::WorldMap;
use crate::mines::MineNode;
use crate::resources::ResourceType;

/// Simulation phase ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimPhase {
    Input,
    ScriptSchedule,
    LuaExecution,
    CommandValidation,
    Movement,
    Mining,
    Transport,
    Building,
    Combat,
    Upkeep,
    Economy,
    DeathCleanup,
    EventFlush,
    UiRefresh,
}

impl SimPhase {
    pub fn order() -> Vec<Self> {
        vec![
            SimPhase::Input,
            SimPhase::ScriptSchedule,
            SimPhase::LuaExecution,
            SimPhase::CommandValidation,
            SimPhase::Movement,
            SimPhase::Mining,
            SimPhase::Transport,
            SimPhase::Building,
            SimPhase::Combat,
            SimPhase::Upkeep,
            SimPhase::Economy,
            SimPhase::DeathCleanup,
            SimPhase::EventFlush,
            SimPhase::UiRefresh,
        ]
    }
}

/// Movement phase system - handles creep movement
pub fn movement_phase(
    mut creeps: Query<&mut Creep>,
    world_map: Res<WorldMap>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut creep in creeps.iter_mut() {
        // Check if creep has a move action
        if let Some(ref action) = creep.current_action {
            if let CreepAction::MoveTo { target } = action.action {
                let speed = creep.body.speed();
                if speed <= 0.0 {
                    continue;
                }

                // Simple movement towards target
                let dx = (target.x as f32 - creep.position.x as f32).signum();
                let dy = (target.y as f32 - creep.position.y as f32).signum();

                let new_x = (creep.position.x as f32 + dx * speed) as i32;
                let new_y = (creep.position.y as f32 + dy * speed) as i32;

                // Check if target reached
                if (new_x - target.x).abs() <= 1 && (new_y - target.y).abs() <= 1 {
                    creep.position.x = target.x;
                    creep.position.y = target.y;
                    creep.current_action = None;
                } else {
                    // Check walkability
                    let new_pos = crate::world::WorldPos::new(new_x, new_y);
                    if world_map.is_walkable(new_pos) {
                        creep.position.x = new_x;
                        creep.position.y = new_y;
                    }
                }
            }
        }
    }
}

/// Mining phase system - handles resource extraction
pub fn mining_phase(
    mut creeps: Query<&mut Creep>,
    mut mines: Query<&mut MineNode>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut creep in creeps.iter_mut() {
        if let Some(ref action) = creep.current_action {
            if let CreepAction::Mine { mine_id } = action.action {
                // Find the mine and extract resources
                for mut mine in mines.iter_mut() {
                    if mine.id == mine_id && mine.current_amount > 0 {
                        let efficiency = creep.body.mining_efficiency();
                        let extracted = (efficiency * 5.0).min(mine.current_amount as f32) as u32;

                        if extracted > 0 {
                            mine.current_amount -= extracted;
                            // Add to creep inventory
                            let resource_type = mine.resource_type();
                            creep.inventory.add(resource_type, extracted);
                        }
                        break;
                    }
                }
            }
        }
    }
}

/// Combat phase system - handles creep vs creep combat
pub fn combat_phase(
    mut creeps: Query<(Entity, &mut Creep)>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    // Simple combat: creeps with Fight action attack target
    let mut damage_to_apply: Vec<(Entity, f32)> = Vec::new();

    // Collect all creeps for targeting
    let creep_data: Vec<(Entity, crate::factions::FactionId)> = creeps
        .iter()
        .map(|(e, c)| (e, c.faction_id))
        .collect();

    for (entity, mut creep) in creeps.iter_mut() {
        if let Some(ref action) = creep.current_action {
            if let CreepAction::Attack { target_id } = action.action {
                // Find target by creep id (not entity)
                for (target_entity, target_faction) in &creep_data {
                    // Check if different faction
                    if *target_faction != creep.faction_id {
                        // Simple: attack the first enemy found
                        let damage = creep.body.attack_power();
                        damage_to_apply.push((*target_entity, damage));
                        break;
                    }
                }
            }
        }
    }

    // Apply damage
    for (entity, mut creep) in creeps.iter_mut() {
        for (target_entity, damage) in &damage_to_apply {
            if entity == *target_entity {
                creep.take_damage(*damage);
            }
        }
    }
}

/// Upkeep phase system - handles power consumption
pub fn upkeep_phase(
    mut creeps: Query<&mut Creep>,
    game_state: Res<GameState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut creep in creeps.iter_mut() {
        // Consume power each tick
        let consumption = crate::consts::CREEP_POWER_CONSUMPTION;
        creep.consume_power(consumption);

        // Check for starvation
        if creep.power_reserve <= 0.0 {
            creep.take_damage(1.0);
            if creep.hp <= 0.0 {
                game_log.add(format!("Creep {} starved to death!", creep.id));
            }
        }
    }
}

/// Death cleanup phase - removes dead creeps
pub fn death_cleanup_phase(
    mut commands: Commands,
    creeps: Query<(Entity, &Creep)>,
    game_state: Res<GameState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (entity, creep) in creeps.iter() {
        if !creep.is_alive() {
            game_log.add(format!("Creep {} died", creep.id));
            commands.entity(entity).despawn();
        }
    }
}

/// Economy phase - handles resource regeneration
pub fn economy_phase(
    mut mines: Query<&mut MineNode>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut mine in mines.iter_mut() {
        // Regenerate mine resources
        if mine.current_amount < mine.max_amount {
            let regen = (mine.max_amount as f32 * crate::consts::DEFAULT_MINE_REGEN_RATE) as u32;
            mine.current_amount = (mine.current_amount + regen).min(mine.max_amount);
        }
    }
}