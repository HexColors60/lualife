use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::consts::{ROOM_GRID_X, ROOM_GRID_Y, ROOM_TILE_SIZE, WORLD_TILES_X, WORLD_TILES_Y};
use crate::render::MainCamera;
use crate::world::WorldMap;

#[derive(Resource, Debug, Clone)]
pub struct MinimapState {
    pub hovered_room: Option<(u32, u32)>,
    pub visible: bool,
}

impl Default for MinimapState {
    fn default() -> Self {
        Self {
            hovered_room: None,
            visible: true,
        }
    }
}

/// Marker for minimap container
#[derive(Component)]
pub struct MinimapPanel;

/// Marker for minimap image
#[derive(Component)]
pub struct MinimapImage;

/// Marker for camera indicator on minimap
#[derive(Component)]
pub struct MinimapCameraIndicator;

/// Setup minimap UI
pub fn setup_minimap(mut commands: Commands, minimap_state: Res<MinimapState>) {
    if !minimap_state.visible {
        return;
    }

    // Minimap panel (bottom-right, above tick counter)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(40.0),
                right: Val::Px(10.0),
                width: Val::Px(128.0),
                height: Val::Px(128.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            border_color: BorderColor(Color::srgb(0.4, 0.4, 0.5)),
            ..default()
        },
        MinimapPanel,
    ));

    // Camera indicator (small rectangle showing camera view)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 0.0, 0.5),
                custom_size: Some(Vec2::new(8.0, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        MinimapCameraIndicator,
    ));
}

/// Update minimap camera indicator position
pub fn update_minimap_indicator(
    camera: Query<&Transform, With<MainCamera>>,
    mut indicator: Query<&mut Transform, (With<MinimapCameraIndicator>, Without<MainCamera>)>,
) {
    let Ok(cam_transform) = camera.get_single() else {
        return;
    };

    // Convert world position to minimap position
    // World is 256x256, minimap is 128x128 pixels
    let scale = 128.0 / WORLD_TILES_X as f32;

    // World position is centered at origin (offset by -128)
    let world_x = cam_transform.translation.x + 128.0;
    let world_y = cam_transform.translation.y + 128.0;

    // Minimap position (bottom-right of screen)
    let minimap_x = world_x * scale - 64.0; // Offset to center in minimap area
    let minimap_y = world_y * scale - 64.0;

    for mut transform in indicator.iter_mut() {
        transform.translation.x = minimap_x;
        transform.translation.y = minimap_y;
    }
}

/// Handle minimap clicks to move camera
pub fn minimap_click_system(
    mut events: EventReader<crate::events::UiEvent>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    for event in events.read() {
        if let crate::events::UiEvent::MinimapClicked { room_x, room_y } = event {
            // Move camera to center of room
            let world_x = (room_x * ROOM_TILE_SIZE + ROOM_TILE_SIZE / 2) as f32 - 128.0;
            let world_y = (room_y * ROOM_TILE_SIZE + ROOM_TILE_SIZE / 2) as f32 - 128.0;

            for mut transform in camera.iter_mut() {
                transform.translation.x = world_x;
                transform.translation.y = world_y;
            }
        }
    }
}

/// Toggle minimap visibility
pub fn toggle_minimap(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut minimap_state: ResMut<MinimapState>,
    mut panel: Query<&mut Visibility, With<MinimapPanel>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        minimap_state.visible = !minimap_state.visible;

        for mut visibility in panel.iter_mut() {
            *visibility = if minimap_state.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
