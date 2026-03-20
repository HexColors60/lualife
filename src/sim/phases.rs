use bevy::prelude::*;

use crate::core::GameState;
use crate::creeps::{Creep, CreepAction};
use crate::mines::MineNode;
use crate::path::{AStar, PathCache};
use crate::world::WorldMap;

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

/// Movement phase system - handles creep movement with pathfinding
pub fn movement_phase(
    mut creeps: Query<&mut Creep>,
    world_map: Res<WorldMap>,
    mut path_cache: ResMut<PathCache>,
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

                // Try to get cached path or compute new one
                let path = if let Some(cached) = path_cache.get(creep.position, target) {
                    cached.clone()
                } else {
                    if let Some(new_path) = AStar::find_path(&world_map, creep.position, target) {
                        path_cache.insert(creep.position, target, new_path.clone());
                        new_path
                    } else {
                        // No path found, cancel action
                        creep.current_action = None;
                        continue;
                    }
                };

                // Find next waypoint
                if let Some(&next_pos) = path
                    .iter()
                    .find(|&&pos| pos.x != creep.position.x || pos.y != creep.position.y)
                {
                    // Move towards next waypoint
                    let dx = (next_pos.x - creep.position.x).signum();
                    let dy = (next_pos.y - creep.position.y).signum();

                    let new_x = creep.position.x + dx;
                    let new_y = creep.position.y + dy;

                    // Check if target reached
                    if new_x == target.x && new_y == target.y {
                        creep.position.x = target.x;
                        creep.position.y = target.y;
                        creep.current_action = None;
                    } else {
                        // Move to next position
                        let new_pos = crate::world::WorldPos::new(new_x, new_y);
                        if world_map.is_walkable(new_pos) {
                            creep.position.x = new_x;
                            creep.position.y = new_y;
                        }
                    }
                } else {
                    // Already at target
                    creep.current_action = None;
                }
            }
        }
    }
}

/// Mining phase system - handles resource extraction
pub fn mining_phase(
    mut creeps: Query<(&mut Creep, &Transform)>,
    mut mines: Query<&mut MineNode>,
    game_state: Res<GameState>,
    mut transfer_events: EventWriter<crate::render::ResourceTransferEvent>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (mut creep, transform) in creeps.iter_mut() {
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

                            // Send visual feedback event
                            transfer_events.send(crate::render::ResourceTransferEvent {
                                position: transform.translation,
                                resource_type,
                                amount: extracted,
                                is_deposit: true,
                            });
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
    mut creeps: Query<(Entity, &mut Creep, &Transform)>,
    game_state: Res<GameState>,
    mut damage_events: EventWriter<crate::render::DamageEvent>,
) {
    if *game_state != GameState::Running {
        return;
    }

    // Simple combat: creeps with Fight action attack target
    let mut damage_to_apply: Vec<(Entity, f32, Vec3)> = Vec::new();

    // Collect all creeps for targeting
    let creep_data: Vec<(Entity, crate::factions::FactionId)> =
        creeps.iter().map(|(e, c, _)| (e, c.faction_id)).collect();

    for (entity, mut creep, transform) in creeps.iter_mut() {
        if let Some(ref action) = creep.current_action {
            if let CreepAction::Attack { target_id: _ } = action.action {
                // Find target by creep id (not entity)
                for (target_entity, target_faction) in &creep_data {
                    // Check if different faction
                    if *target_faction != creep.faction_id {
                        // Simple: attack the first enemy found
                        let damage = creep.body.attack_power();
                        damage_to_apply.push((*target_entity, damage, transform.translation));
                        break;
                    }
                }
            }
        }
    }

    // Apply damage and send events
    for (entity, mut creep, transform) in creeps.iter_mut() {
        for (target_entity, damage, _pos) in &damage_to_apply {
            if entity == *target_entity {
                creep.take_damage(*damage);
                // Send damage event for visual feedback
                damage_events.send(crate::render::DamageEvent {
                    target: entity,
                    attacker_faction: creep.faction_id,
                    damage: *damage as u32,
                    position: transform.translation,
                });
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

/// Economy phase - handles resource regeneration with scarcity effects
pub fn economy_phase(
    mut mines: Query<&mut MineNode>,
    game_state: Res<GameState>,
    levels: Res<crate::resources::GlobalResourceLevels>,
    config: Res<crate::resources::ScarcityConfig>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut mine in mines.iter_mut() {
        // Regenerate mine resources with scarcity multiplier
        if mine.current_amount < mine.max_amount {
            let base_regen = crate::consts::DEFAULT_MINE_REGEN_RATE;
            let multiplier = crate::resources::get_regen_multiplier(
                mine.resource_type(),
                &levels,
                &config,
            );
            let regen = (mine.max_amount as f32 * base_regen * multiplier) as u32;
            mine.current_amount = (mine.current_amount + regen).min(mine.max_amount);
            
            // Unexhaust if regenerated
            if mine.current_amount > 0 && mine.exhausted {
                mine.exhausted = false;
            }
        }
    }
}

/// Build phase system - handles construction
pub fn build_phase(
    mut creeps: Query<&mut Creep>,
    mut construction_sites: Query<(Entity, &mut crate::buildings::ConstructionSite, &Transform)>,
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut creep in creeps.iter_mut() {
        if let Some(ref action) = creep.current_action {
            if let CreepAction::Build { building_id } = action.action {
                // Find the construction site
                for (entity, mut site, transform) in construction_sites.iter_mut() {
                    if entity.index() as u32 == building_id {
                        // Check if creep is close enough
                        let site_pos = crate::world::WorldPos::new(
                            (transform.translation.x + 128.0) as i32,
                            (transform.translation.y + 128.0) as i32,
                        );
                        let dx = (creep.position.x - site_pos.x) as f32;
                        let dy = (creep.position.y - site_pos.y) as f32;
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance <= 3.0 {
                            // Add construction progress
                            let build_power = creep.body.build_efficiency();
                            site.add_progress(build_power * 10.0);

                            // Check if construction is complete
                            if site.is_complete() {
                                // Convert construction site to building
                                let building = crate::buildings::Building::new(
                                    0, // Will be assigned by building system
                                    site.building_type,
                                    site.faction_id,
                                    site_pos,
                                );

                                commands.entity(entity).despawn();
                                commands.spawn(building);

                                game_log.add(format!(
                                    "Building {:?} completed for faction {:?}",
                                    site.building_type, site.faction_id
                                ));
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}
