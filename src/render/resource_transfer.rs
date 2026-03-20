//! Resource transfer animation for visual feedback.

use bevy::prelude::*;

use crate::resources::ResourceType;

/// Component for a floating resource transfer indicator
#[derive(Component)]
pub struct ResourceTransferIndicator {
    /// Resource type being transferred
    pub resource_type: ResourceType,
    /// Amount transferred
    pub amount: u32,
    /// Whether this is a deposit (to building) or pickup (from ground/source)
    pub is_deposit: bool,
    /// Time since spawned
    pub age: f32,
    /// Lifetime in seconds
    pub lifetime: f32,
    /// Velocity for floating animation
    pub velocity: Vec2,
}

/// Event for resource transfers (to trigger visual feedback)
#[derive(Event, Debug, Clone)]
pub struct ResourceTransferEvent {
    /// Position where transfer occurred
    pub position: Vec3,
    /// Resource type
    pub resource_type: ResourceType,
    /// Amount transferred
    pub amount: u32,
    /// Whether this is a deposit
    pub is_deposit: bool,
}

/// Spawn a resource transfer indicator
pub fn spawn_transfer_indicator(
    commands: &mut Commands,
    position: Vec3,
    resource_type: ResourceType,
    amount: u32,
    is_deposit: bool,
) {
    let color = get_resource_color(resource_type);
    let text = if is_deposit {
        format!("+{}", amount)
    } else {
        format!("{}", amount)
    };

    let random_offset = Vec3::new(
        (rand::random::<f32>() - 0.5) * 20.0,
        15.0,
        0.0,
    );

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: 12.0,
                    color,
                    ..default()
                },
            ),
            transform: Transform::from_translation(position + random_offset)
                .with_scale(Vec3::splat(1.0)),
            ..default()
        },
        ResourceTransferIndicator {
            resource_type,
            amount,
            is_deposit,
            age: 0.0,
            lifetime: 1.2,
            velocity: Vec2::new(
                (rand::random::<f32>() - 0.5) * 20.0,
                25.0,
            ),
        },
    ));
}

/// Get color for a resource type
fn get_resource_color(resource_type: ResourceType) -> Color {
    match resource_type {
        ResourceType::Power => Color::srgb(1.0, 0.9, 0.2),    // Yellow
        ResourceType::Iron => Color::srgb(0.6, 0.6, 0.7),     // Silver-gray
        ResourceType::Copper => Color::srgb(0.85, 0.55, 0.3), // Copper
        ResourceType::Silicon => Color::srgb(0.5, 0.5, 0.6),  // Gray
        ResourceType::Crystal => Color::srgb(0.6, 0.8, 1.0),  // Light blue
        ResourceType::Carbon => Color::srgb(0.3, 0.3, 0.3),   // Dark gray
        ResourceType::Stone => Color::srgb(0.5, 0.5, 0.45),   // Stone gray
        ResourceType::Sulfur => Color::srgb(0.9, 0.9, 0.3),   // Yellow
        ResourceType::Water => Color::srgb(0.3, 0.6, 0.9),    // Blue
        ResourceType::Biomass => Color::srgb(0.3, 0.7, 0.3),  // Green
    }
}

/// System to spawn transfer indicators from events
pub fn resource_transfer_event_system(
    mut events: EventReader<ResourceTransferEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        spawn_transfer_indicator(
            &mut commands,
            event.position,
            event.resource_type,
            event.amount,
            event.is_deposit,
        );
    }
}

/// Update transfer indicators (float up and fade)
pub fn update_transfer_indicators(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ResourceTransferIndicator, &mut Transform, &mut Text)>,
) {
    for (entity, mut indicator, mut transform, mut text) in query.iter_mut() {
        indicator.age += time.delta_seconds();

        // Float with velocity
        transform.translation.x += indicator.velocity.x * time.delta_seconds();
        transform.translation.y += indicator.velocity.y * time.delta_seconds();

        // Slow down
        indicator.velocity *= 0.95;

        // Fade out in last 0.4 seconds
        let fade_start = indicator.lifetime - 0.4;
        if indicator.age > fade_start {
            let fade_progress = (indicator.age - fade_start) / 0.4;
            let alpha = 1.0 - fade_progress.min(1.0);
            let base_color = get_resource_color(indicator.resource_type);
            text.sections[0].style.color = Color::srgba(
                base_color.to_srgba().red,
                base_color.to_srgba().green,
                base_color.to_srgba().blue,
                alpha,
            );
        }

        // Despawn when lifetime exceeded
        if indicator.age >= indicator.lifetime {
            commands.entity(entity).despawn();
        }
    }
}

/// Plugin for resource transfer visualization
pub struct ResourceTransferPlugin;

impl Plugin for ResourceTransferPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResourceTransferEvent>()
            .add_systems(Update, (resource_transfer_event_system, update_transfer_indicators));
    }
}