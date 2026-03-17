mod faction_spawn_gen;
mod generator;
mod mine_distribution;
mod room_layout_gen;
mod seeds;
mod terrain_gen;

pub use faction_spawn_gen::*;
pub use generator::*;
pub use mine_distribution::*;
pub use room_layout_gen::*;
pub use seeds::*;
pub use terrain_gen::*;

use bevy::prelude::*;

use crate::config::WorldgenConfig;
use crate::core::GameRng;
use crate::world::WorldBuilder;
use crate::world::WorldMap;

pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_world_system);
    }
}

fn generate_world_system(
    mut commands: Commands,
    config: Res<WorldgenConfig>,
    mut rng: ResMut<GameRng>,
) {
    tracing::info!("Generating world with seed {}...", rng.seed());

    let config_clone = config.clone();
    let generated = WorldGenerator::generate(&config_clone, &mut rng);
    let world_map = WorldBuilder::build_from_generated(generated);

    tracing::info!(
        "World generated: {} rooms, {} mines",
        world_map.room_count(),
        world_map
            .all_rooms()
            .map(|r| r.mine_ids.len())
            .sum::<usize>()
    );

    commands.insert_resource(world_map);
}
