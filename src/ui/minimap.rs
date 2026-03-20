use bevy::prelude::*;

use crate::consts::{ROOM_TILE_SIZE, WORLD_TILES_X};
use crate::render::MainCamera;
use crate::territory::TerritoryManager;
use crate::factions::FactionId;

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

/// Marker for minimap territory display
#[derive(Component)]
pub struct MinimapTerritory;

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

    // Territory visualization (canvas of colored pixels)
    // Using a sprite to render territory colors
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.2, 0.3, 0.8),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        MinimapTerritory,
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

/// Faction colors for minimap display
fn get_faction_color(faction_id: FactionId) -> Color {
    // Generate a unique color for each faction based on ID
    let hue = (faction_id.0 as f32 * 11.25) % 360.0; // 360/32 = 11.25 degrees per faction
    let saturation: f32 = 0.7;
    let lightness: f32 = 0.5;
    
    // HSL to RGB conversion
    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = lightness - c / 2.0;
    
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    Color::srgb(r + m, g + m, b + m)
}

/// Update minimap territory colors based on faction control
pub fn update_minimap_territory(
    territory: Res<TerritoryManager>,
    mut territory_sprite: Query<&mut Sprite, With<MinimapTerritory>>,
) {
    // This would ideally use a texture to render territory colors
    // For simplicity, we'll just log the territory counts
    if territory.is_changed() {
        let mut total_controlled = 0;
        for faction_id in 0..32u16 {
            let faction = FactionId(faction_id);
            let count = territory.get_territory_count(faction);
            if count > 0 {
                total_controlled += count;
            }
        }
        // Territory is being tracked
        let _ = total_controlled;
    }
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