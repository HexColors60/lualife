mod damage_numbers;

mod building_render;
mod camera;
mod construction_progress;
mod creep_render;
mod creep_trail;
mod debug_render;
mod healthbars;
mod map_render;
mod mine_render;
mod path_visualization;
mod resource_transfer;
mod room_borders;
mod room_overlay;
mod tower_visualization;

pub use building_render::*;
pub use camera::*;
pub use construction_progress::*;
pub use creep_render::*;
pub use creep_trail::*;
pub use debug_render::*;
pub use damage_numbers::*;
pub use healthbars::*;
pub use map_render::*;
pub use mine_render::*;
pub use path_visualization::*;
pub use resource_transfer::*;
pub use room_borders::*;
pub use room_overlay::*;
pub use tower_visualization::*;


use crate::debug::GodMode;
use crate::world::WorldMap;
use crate::world::get_fog_alpha;
use crate::consts::ROOM_TILE_SIZE;
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapRendered>()
            .add_plugins(DamageNumbersPlugin)
            .add_plugins(RoomBorderPlugin)
            .add_plugins(CreepTrailPlugin)
            .add_plugins(ResourceTransferPlugin)
            .add_plugins(ConstructionProgressPlugin)
            .add_systems(Startup, (setup_render, setup_debug_overlays))
            .add_systems(
                Update,
                (
                    spawn_room_sprites.run_if(resource_exists::<WorldMap>),
                    spawn_creep_sprites.run_if(resource_exists::<crate::factions::FactionRegistry>),
                    spawn_mine_sprites,
                    spawn_building_sprites
                        .run_if(resource_exists::<crate::factions::FactionRegistry>),
                    spawn_health_bars,
                    update_creep_sprites,
                    update_mine_sprites,
                    update_building_sprites,
                    update_health_bars,
                    update_fps_display,
                    update_entity_count,
                    toggle_debug_overlays,
                    toggle_path_visualization,
                    path_visualization_system,
                    toggle_tower_range_visualization,
                    tower_range_visualization_system,
                    update_fog_of_war,
                ),
            );
    }
}


#[derive(Resource, Default)]
pub struct MapRendered(bool);

#[derive(Component)]
pub struct RoomSprite {
    pub room_coord: crate::world::RoomCoord,
}

/// Store original room color for fog of war calculations.
#[derive(Component)]
pub struct RoomOriginalColor(pub Color);

fn setup_render(mut commands: Commands) {
    // Main camera - use default settings (centered at origin)
    commands.spawn((Camera2dBundle::default(), MainCamera));
    tracing::info!("Camera spawned");
}

fn spawn_room_sprites(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut rendered: ResMut<MapRendered>,
) {
    if rendered.0 {
        return;
    }

    if world_map.room_count() == 0 {
        tracing::warn!("World map has no rooms!");
        return;
    }

    let room_size = ROOM_TILE_SIZE as f32;
    let mut first_room = true;

    for room in world_map.all_rooms() {
        // Calculate average terrain color for the room
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;
        let mut count = 0;

        for row in &room.tiles {
            for tile in row {
                let color = tile.terrain.color();
                r += color.0 as u32;
                g += color.1 as u32;
                b += color.2 as u32;
                count += 1;
            }
        }

        let avg_color = if count > 0 {
            Color::srgb(
                (r / count) as f32 / 255.0,
                (g / count) as f32 / 255.0,
                (b / count) as f32 / 255.0,
            )
        } else {
            Color::srgb(0.5, 0.5, 0.5)
        };

        // Position sprites centered around origin
        // World is 32x32 rooms, each 8 tiles = 256x256 total
        // Center at (128, 128), so offset by -128
        let x = (room.coord.x * ROOM_TILE_SIZE) as f32 + room_size / 2.0 - 128.0;
        let y = (room.coord.y * ROOM_TILE_SIZE) as f32 + room_size / 2.0 - 128.0;

        if first_room {
            tracing::info!(
                "First room: coord=({}, {}), pos=({:.1}, {:.1}), color=({:.2}, {:.2}, {:.2})",
                room.coord.x,
                room.coord.y,
                x,
                y,
                (r / count) as f32 / 255.0,
                (g / count) as f32 / 255.0,
                (b / count) as f32 / 255.0
            );
            first_room = false;
        }

        // Spawn room sprite with a small gap between rooms
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: avg_color,
                    custom_size: Some(Vec2::new(room_size - 1.0, room_size - 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            RoomSprite { room_coord: room.coord },
            RoomOriginalColor(avg_color),
        ));
    }

    tracing::info!("Spawned {} room sprites", world_map.room_count());
    rendered.0 = true;
}

/// Update room colors based on fog of war visibility.
fn update_fog_of_war(
    mut room_sprites: Query<(&RoomSprite, &RoomOriginalColor, &mut Sprite), With<RoomSprite>>,
    world_map: Res<WorldMap>,
    god_mode: Res<GodMode>,
) {
    // Only update if world map exists and not in GOD mode with full visibility
    if world_map.room_count() == 0 {
        return;
    }

    // In GOD mode, show all rooms at full brightness
    if god_mode.enabled {
        for (_, original_color, mut sprite) in room_sprites.iter_mut() {
            sprite.color = original_color.0;
        }
        return;
    }

    // Apply fog of war to each room
    for (room_sprite, original_color, mut sprite) in room_sprites.iter_mut() {
        let fog_alpha = get_fog_alpha(&world_map, room_sprite.room_coord, None, false);
        
        // Darken the room based on fog level
        let base_color = original_color.0;
        let darkened = if fog_alpha > 0.99 {
            // Never discovered - very dark
            Color::srgb(0.1, 0.1, 0.15)
        } else if fog_alpha > 0.4 {
            // Discovered but not currently visible - dimmed
            let darken = 0.4;
            Color::srgb(
                base_color.to_srgba().red * darken,
                base_color.to_srgba().green * darken,
                base_color.to_srgba().blue * darken,
            )
        } else {
            // Currently visible - full brightness
            base_color
        };
        
        sprite.color = darkened;
    }
}
