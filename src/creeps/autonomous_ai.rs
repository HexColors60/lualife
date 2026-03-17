use bevy::prelude::*;

use crate::core::GameState;
use crate::creeps::{Creep, CreepAction};
use crate::mines::MineNode;
use crate::resources::ResourceType;

/// Simple autonomous AI for creeps
/// This provides basic behavior when Lua scripts are not controlling units
pub fn autonomous_creep_ai(
    mut creeps: Query<&mut Creep>,
    mines: Query<&MineNode>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for mut creep in creeps.iter_mut() {
        // Skip if creep already has an action
        if creep.current_action.is_some() {
            continue;
        }

        // Check if creep is starving - prioritize finding power
        if creep.is_starving() {
            // Find nearest power mine
            if let Some(mine) = find_nearest_mine(&mines, creep.position, ResourceType::Power) {
                if creep.position.distance_to(mine.position) <= 2.0 {
                    // Close enough to mine
                    creep.current_action = Some(crate::creeps::components::CurrentAction {
                        action: CreepAction::Mine { mine_id: mine.id },
                        target_id: None,
                        progress: 0.0,
                    });
                } else {
                    // Move towards mine
                    creep.current_action = Some(crate::creeps::components::CurrentAction {
                        action: CreepAction::MoveTo {
                            target: mine.position,
                        },
                        target_id: None,
                        progress: 0.0,
                    });
                }
                continue;
            }
        }

        // Default behavior: find nearest mine to harvest
        if let Some(mine) = find_nearest_mine(&mines, creep.position, ResourceType::Power) {
            let distance = creep.position.distance_to(mine.position);

            if distance <= 2.0 {
                // Close enough to mine
                creep.current_action = Some(crate::creeps::components::CurrentAction {
                    action: CreepAction::Mine { mine_id: mine.id },
                    target_id: None,
                    progress: 0.0,
                });
            } else {
                // Move towards mine
                creep.current_action = Some(crate::creeps::components::CurrentAction {
                    action: CreepAction::MoveTo {
                        target: mine.position,
                    },
                    target_id: None,
                    progress: 0.0,
                });
            }
        }
    }
}

/// Find the nearest mine of a specific resource type
fn find_nearest_mine(
    mines: &Query<&MineNode>,
    position: crate::world::WorldPos,
    resource_type: ResourceType,
) -> Option<MineNode> {
    let mut nearest: Option<MineNode> = None;
    let mut nearest_dist: f32 = f32::MAX;

    for mine in mines.iter() {
        if mine.resource_type() == resource_type && mine.current_amount > 0 {
            let dist = position.distance_to(mine.position) as f32;
            if dist < nearest_dist {
                nearest_dist = dist;
                nearest = Some(mine.clone());
            }
        }
    }

    nearest
}

/// Trait extension for WorldPos distance calculation
trait DistanceTo {
    fn distance_to(&self, other: crate::world::WorldPos) -> f32;
}

impl DistanceTo for crate::world::WorldPos {
    fn distance_to(&self, other: crate::world::WorldPos) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}
