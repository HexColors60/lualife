//! Room border rendering for visual room boundaries.

use bevy::prelude::*;

use crate::consts::{ROOM_GRID_X, ROOM_GRID_Y, ROOM_TILE_SIZE, WORLD_TILES_X, WORLD_TILES_Y};

/// Marker for room border lines
#[derive(Component)]
pub struct RoomBorder;

/// Setup room border grid
pub fn setup_room_borders(mut commands: Commands) {
    // Calculate world offset (world is centered at origin)
    let world_width = WORLD_TILES_X as f32;
    let world_height = WORLD_TILES_Y as f32;
    let half_width = world_width / 2.0;
    let half_height = world_height / 2.0;

    // Room size in world units
    let room_size = ROOM_TILE_SIZE as f32;

    // Draw vertical lines between rooms
    for x in 0..=ROOM_GRID_X {
        let world_x = (x as f32 * room_size) - half_width;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.3, 0.3, 0.4, 0.3), // Subtle blue-gray, semi-transparent
                    custom_size: Some(Vec2::new(1.0, world_height)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, 0.0, 1.0),
                ..default()
            },
            RoomBorder,
        ));
    }

    // Draw horizontal lines between rooms
    for y in 0..=ROOM_GRID_Y {
        let world_y = (y as f32 * room_size) - half_height;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.3, 0.3, 0.4, 0.3), // Subtle blue-gray, semi-transparent
                    custom_size: Some(Vec2::new(world_width, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, world_y, 1.0),
                ..default()
            },
            RoomBorder,
        ));
    }
}

/// Toggle room border visibility
pub fn toggle_room_borders(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut borders: Query<&mut Visibility, With<RoomBorder>>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        for mut visibility in borders.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible,
            };
        }
    }
}

/// Plugin for room border rendering
pub struct RoomBorderPlugin;

impl Plugin for RoomBorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_room_borders)
            .add_systems(Update, toggle_room_borders);
    }
}