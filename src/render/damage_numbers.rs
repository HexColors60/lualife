//! Floating damage numbers for combat visualization.

use bevy::prelude::*;

use crate::factions::FactionId;

/// Component for floating damage text
#[derive(Component)]
pub struct DamageNumber {
    /// Damage amount displayed
    pub damage: u32,
    /// Time since spawned
    pub age: f32,
    /// Maximum lifetime in seconds
    pub lifetime: f32,
    /// Vertical velocity
    pub velocity: f32,
}

/// Spawn a damage number at a world position
pub fn spawn_damage_number(
    commands: &mut Commands,
    position: Vec3,
    damage: u32,
) {
    let random_offset = Vec3::new(
        (rand::random::<f32>() - 0.5) * 20.0,
        10.0,
        0.0,
    );

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("-{}", damage),
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(1.0, 0.3, 0.3),
                    ..default()
                },
            ),
            transform: Transform::from_translation(position + random_offset)
                .with_scale(Vec3::splat(1.0)),
            ..default()
        },
        DamageNumber {
            damage,
            age: 0.0,
            lifetime: 1.0,
            velocity: 30.0,
        },
    ));
}

/// System to update damage numbers (float up and fade)
pub fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform, &mut Text)>,
) {
    for (entity, mut damage, mut transform, mut text) in query.iter_mut() {
        damage.age += time.delta_seconds();

        // Float up
        transform.translation.y += damage.velocity * time.delta_seconds();

        let fade_start = damage.lifetime - 0.3;
        if damage.age > fade_start {
            let fade_progress = (damage.age - fade_start) / 0.3;
            let alpha = 1.0 - fade_progress.min(1.0);
            text.sections[0].style.color = Color::srgba(1.0, 0.3, 0.3, alpha);
        }

        // Despawn when lifetime exceeded
        if damage.age >= damage.lifetime {
            commands.entity(entity).despawn();
        }
    }
}

/// Event for damage dealt (to trigger visual feedback)
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub attacker_faction: FactionId,
    pub damage: u32,
    pub position: Vec3,
}

/// System to spawn damage numbers from damage events
pub fn damage_event_system(
    mut events: EventReader<DamageEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        spawn_damage_number(&mut commands, event.position, event.damage);
    }
}

/// Plugin for damage number visualization
pub struct DamageNumbersPlugin;

impl Plugin for DamageNumbersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(Update, (damage_event_system, update_damage_numbers));
    }
}