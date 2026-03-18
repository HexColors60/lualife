use bevy::prelude::*;

use crate::creeps::Creep;
use crate::render::creep_render::CreepSprite;

/// Marker for health bar background
#[derive(Component)]
pub struct HealthBarBackground;

/// Marker for health bar foreground (HP)
#[derive(Component)]
pub struct HealthBarForeground {
    pub creep_entity: Entity,
}

/// Marker for power bar foreground
#[derive(Component)]
pub struct PowerBarForeground {
    pub creep_entity: Entity,
}

/// Spawn health bars for creeps
pub fn spawn_health_bars(mut commands: Commands, creeps: Query<(Entity, &Creep), Added<Creep>>) {
    for (entity, creep) in creeps.iter() {
        let x = creep.position.x as f32 - 128.0;
        let y = creep.position.y as f32 - 128.0 + 2.0; // Above creep

        // Background bar (dark)
        let bg_entity = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.2, 0.2, 0.8),
                        custom_size: Some(Vec2::new(3.0, 0.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 2.0),
                    ..default()
                },
                HealthBarBackground,
            ))
            .id();

        // HP bar (green)
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2),
                    custom_size: Some(Vec2::new(3.0, 0.25)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y + 0.15, 2.1),
                ..default()
            },
            HealthBarForeground {
                creep_entity: entity,
            },
        ));

        // Power bar (blue)
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.5, 0.9),
                    custom_size: Some(Vec2::new(3.0, 0.25)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y - 0.15, 2.1),
                ..default()
            },
            PowerBarForeground {
                creep_entity: entity,
            },
        ));
    }
}

/// Update health bar positions and sizes
pub fn update_health_bars(
    creeps: Query<&Creep, Changed<Creep>>,
    mut hp_bars: Query<(&HealthBarForeground, &mut Sprite, &mut Transform), Without<PowerBarForeground>>,
    mut power_bars: Query<(&PowerBarForeground, &mut Sprite, &mut Transform), Without<HealthBarForeground>>,
) {
    // Update HP bars
    for (hp_bar, mut sprite, mut transform) in hp_bars.iter_mut() {
        if let Ok(creep) = creeps.get(hp_bar.creep_entity) {
            let hp_ratio = (creep.hp / creep.max_hp).clamp(0.0, 1.0);

            // Update width based on HP ratio
            sprite.custom_size = Some(Vec2::new(3.0 * hp_ratio, 0.25));

            // Update color (green -> yellow -> red)
            let color = if hp_ratio > 0.5 {
                Color::srgb(0.2, 0.8, 0.2)
            } else if hp_ratio > 0.25 {
                Color::srgb(0.8, 0.8, 0.2)
            } else {
                Color::srgb(0.9, 0.2, 0.2)
            };
            sprite.color = color;

            // Update position
            let x = creep.position.x as f32 - 128.0;
            let y = creep.position.y as f32 - 128.0 + 2.15;
            transform.translation.x = x - 1.5 * (1.0 - hp_ratio);
            transform.translation.y = y;
        }
    }

    // Update power bars
    for (power_bar, mut sprite, mut transform) in power_bars.iter_mut() {
        if let Ok(creep) = creeps.get(power_bar.creep_entity) {
            let power_ratio = (creep.power_reserve / creep.max_power).clamp(0.0, 1.0);

            // Update width based on power ratio
            sprite.custom_size = Some(Vec2::new(3.0 * power_ratio, 0.25));

            // Update position
            let x = creep.position.x as f32 - 128.0;
            let y = creep.position.y as f32 - 128.0 + 1.85;
            transform.translation.x = x - 1.5 * (1.0 - power_ratio);
            transform.translation.y = y;
        }
    }
}
