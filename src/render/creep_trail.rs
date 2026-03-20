//! Creep trail visualization for tracking movement patterns.

use bevy::prelude::*;

use crate::creeps::Creep;

/// Component for a single trail segment
#[derive(Component)]
pub struct TrailSegment {
    /// Time this segment was created
    pub created: f32,
    /// Lifetime in seconds
    pub lifetime: f32,
    /// Faction for color
    pub faction_id: crate::factions::FactionId,
}

/// Resource for trail settings
#[derive(Resource, Debug, Clone)]
pub struct TrailSettings {
    /// Whether trails are visible
    pub enabled: bool,
    /// Maximum trail segments per creep
    pub max_segments: usize,
    /// Trail segment lifetime in seconds
    pub segment_lifetime: f32,
    /// How often to record positions (in ticks)
    pub record_interval: u64,
}

impl Default for TrailSettings {
    fn default() -> Self {
        Self {
            enabled: false, // Off by default
            max_segments: 20,
            segment_lifetime: 5.0,
            record_interval: 10,
        }
    }
}

/// Trail history for a creep
#[derive(Component, Default)]
pub struct TrailHistory {
    /// Recorded positions
    pub positions: Vec<(i32, i32, u64)>, // (x, y, tick)
    /// Last recorded tick
    pub last_record_tick: u64,
}

/// Toggle trail visualization
pub fn toggle_trails(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<TrailSettings>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        settings.enabled = !settings.enabled;
    }
}

/// Record creep positions for trails
pub fn record_trail_positions(
    mut commands: Commands,
    settings: Res<TrailSettings>,
    mut creeps: Query<(Entity, &Creep, &mut TrailHistory, &Transform), Without<TrailSegment>>,
    tick: Res<crate::core::TickNumber>,
) {
    if !settings.enabled {
        return;
    }

    let current_tick = tick.0;

    for (entity, creep, mut history, transform) in creeps.iter_mut() {
        // Check if we should record this tick
        if current_tick - history.last_record_tick < settings.record_interval {
            continue;
        }

        // Record position
        let x = creep.position.x;
        let y = creep.position.y;
        history.positions.push((x, y, current_tick));
        history.last_record_tick = current_tick;

        // Trim old positions
        while history.positions.len() > settings.max_segments {
            history.positions.remove(0);
        }

        // Spawn a visual trail segment
        let alpha = 0.3; // Semi-transparent
        let color = get_faction_trail_color(creep.faction_id, alpha);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.0)),
                ..default()
            },
            TrailSegment {
                created: 0.0,
                lifetime: settings.segment_lifetime,
                faction_id: creep.faction_id,
            },
        ));
    }
}

/// Update trail segments (fade and despawn)
pub fn update_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<TrailSettings>,
    mut query: Query<(Entity, &mut TrailSegment, &mut Sprite)>,
) {
    if !settings.enabled {
        // Despawn all trail segments when disabled
        for (entity, _, _) in query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    for (entity, mut segment, mut sprite) in query.iter_mut() {
        segment.created += time.delta_seconds();

        // Fade out
        let age_ratio = segment.created / segment.lifetime;
        if age_ratio < 1.0 {
            let alpha = 0.3 * (1.0 - age_ratio);
            sprite.color = get_faction_trail_color(segment.faction_id, alpha);
        }

        // Despawn when expired
        if segment.created >= segment.lifetime {
            commands.entity(entity).despawn();
        }
    }
}

/// Get trail color for a faction
fn get_faction_trail_color(faction_id: crate::factions::FactionId, alpha: f32) -> Color {
    // Generate a unique color for each faction
    let hue = (faction_id.0 as f32 * 11.25) % 360.0;
    let saturation: f32 = 0.6;
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

    Color::srgba(r + m, g + m, b + m, alpha)
}

/// Plugin for creep trail visualization
pub struct CreepTrailPlugin;

impl Plugin for CreepTrailPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TrailSettings>()
            .add_systems(Update, (toggle_trails, record_trail_positions, update_trail_segments));
    }
}