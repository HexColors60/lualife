mod load_game;
mod save_game;
mod snapshot;
mod versioning;

pub use load_game::*;
pub use save_game::*;
pub use snapshot::*;
pub use versioning::*;

use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_save_input, handle_load_input));
    }
}

/// Handle save keyboard input (F5 for quick save)
fn handle_save_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    tick: Res<crate::core::TickNumber>,
    creeps: Query<&crate::creeps::Creep>,
    buildings: Query<&crate::buildings::Building>,
    mines: Query<&crate::mines::MineNode>,
    factions: Res<crate::factions::FactionRegistry>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        // Create snapshot
        let mut snapshot = GameSnapshot::new(&tick);

        // Save factions
        for faction in factions.all() {
            snapshot.factions.push(FactionSnapshot {
                id: faction.id,
                name: faction.name.clone(),
                resources: std::collections::HashMap::new(),
            });
        }

        // Save creeps count
        let creep_count = creeps.iter().count();
        let building_count = buildings.iter().count();
        let mine_count = mines.iter().count();

        // Try to save
        match SaveGame::quick_save(&snapshot) {
            Ok(()) => {
                game_log.add(format!(
                    "Game saved! (Tick {}, {} creeps, {} buildings, {} mines)",
                    tick.0, creep_count, building_count, mine_count
                ));
            }
            Err(e) => {
                game_log.add(format!("Failed to save: {}", e));
            }
        }
    }
}

/// Handle load keyboard input (F9 for quick load)
fn handle_load_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        // List available saves
        match LoadGame::list_saves() {
            Ok(saves) => {
                if saves.is_empty() {
                    game_log.add("No save files found".to_string());
                } else {
                    game_log.add(format!(
                        "Available saves: {:?}",
                        saves.iter().take(5).collect::<Vec<_>>()
                    ));
                    // Try to load the most recent
                    if let Some(latest) = saves.first() {
                        match LoadGame::load_from_file(latest) {
                            Ok(snapshot) => {
                                game_log.add(format!("Loaded save from tick {}", snapshot.tick));
                            }
                            Err(e) => {
                                game_log.add(format!("Failed to load: {}", e));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                game_log.add(format!("Failed to list saves: {}", e));
            }
        }
    }
}
