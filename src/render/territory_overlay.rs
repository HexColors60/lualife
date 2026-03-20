//! Territory control visualization - shows faction influence on the map.

use bevy::prelude::*;

use crate::factions::FactionId;
use crate::territory::TerritoryManager;
use crate::world::WorldMap;

/// Marker for territory overlay sprites
#[derive(Component)]
pub struct TerritoryOverlay;

/// Territory overlay configuration
#[derive(Resource, Debug, Clone)]
pub struct TerritoryOverlayConfig {
    /// Whether overlay is visible
    pub visible: bool,
    /// Opacity of territory overlay
    pub opacity: f32,
}

impl Default for TerritoryOverlayConfig {
    fn default() -> Self {
        Self {
            visible: true,
            opacity: 0.3,
        }
    }
}

/// Spawn territory overlay sprites for each room
pub fn spawn_territory_overlay(
    mut commands: Commands,
    territory: Res<TerritoryManager>,
    world_map: Res<WorldMap>,
    config: Res<TerritoryOverlayConfig>,
    existing: Query<Entity, With<TerritoryOverlay>>,
) {
    // Clear existing overlays if visibility changed
    if !config.visible {
        for entity in existing.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    // Only update if territory changed
    if !territory.is_changed() && !existing.is_empty() {
        return;
    }

    // Clear existing
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn colored overlay for each claimed room
    for (coord, territory) in &territory.territories {
        if let Some(owner) = territory.owner {
            let color = get_faction_territory_color(owner);
            let color_with_alpha = Color::srgba(color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, config.opacity);
            
            let x = (coord.x * 32) as f32 + 16.0 - 128.0;
            let y = (coord.y * 32) as f32 + 16.0 - 128.0;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: color_with_alpha,
                        custom_size: Some(Vec2::new(30.0, 30.0)), // Slightly smaller than room
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, -0.5), // Behind other sprites
                    ..default()
                },
                TerritoryOverlay,
            ));
        }
    }
}

/// Get color for a faction's territory
fn get_faction_territory_color(faction_id: FactionId) -> Color {
    let hue = (faction_id.0 as f32 / 32.0) * 360.0;
    hsl_to_rgb(hue, 0.6, 0.5)
}

/// HSL to RGB color conversion
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    Color::srgb(r + m, g + m, b + m)
}

/// Toggle territory overlay visibility
pub fn toggle_territory_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<TerritoryOverlayConfig>,
) {
    // T key toggles territory overlay
    if keyboard.just_pressed(KeyCode::KeyT) {
        config.visible = !config.visible;
    }
}

/// Plugin for territory overlay
pub struct TerritoryOverlayPlugin;

impl Plugin for TerritoryOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerritoryOverlayConfig>()
            .add_systems(Update, (spawn_territory_overlay, toggle_territory_overlay));
    }
}