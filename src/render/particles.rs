//! Particle system for visual effects.

use bevy::prelude::*;

/// Particle component
#[derive(Component)]
pub struct Particle {
    /// Velocity of the particle
    pub velocity: Vec3,
    /// Time since spawned
    pub age: f32,
    /// Lifetime in seconds
    pub lifetime: f32,
    /// Start size
    pub start_size: f32,
    /// End size
    pub end_size: f32,
    /// Start color
    pub start_color: Color,
    /// End color
    pub end_color: Color,
    /// Gravity effect
    pub gravity: f32,
    /// Drag coefficient
    pub drag: f32,
}

/// Particle emitter configuration
#[derive(Debug, Clone)]
pub struct ParticleEmitterConfig {
    /// Number of particles to spawn
    pub count: usize,
    /// Spawn position offset range
    pub position_offset: Vec3,
    /// Initial velocity range (min, max)
    pub velocity_range: (Vec3, Vec3),
    /// Lifetime range (min, max)
    pub lifetime_range: (f32, f32),
    /// Size range (start, end)
    pub size_range: (f32, f32),
    /// Color range
    pub color_start: Color,
    pub color_end: Color,
    /// Gravity
    pub gravity: f32,
    /// Drag
    pub drag: f32,
}

impl Default for ParticleEmitterConfig {
    fn default() -> Self {
        Self {
            count: 20,
            position_offset: Vec3::new(10.0, 10.0, 0.0),
            velocity_range: (Vec3::new(-50.0, -50.0, -10.0), Vec3::new(50.0, 50.0, 10.0)),
            lifetime_range: (0.5, 1.5),
            size_range: (2.0, 0.5),
            color_start: Color::srgba(1.0, 0.8, 0.2, 1.0),
            color_end: Color::srgba(1.0, 0.2, 0.1, 0.0),
            gravity: -98.0,
            drag: 0.5,
        }
    }
}

/// Spawn particles at a position
pub fn spawn_particles(commands: &mut Commands, position: Vec3, config: ParticleEmitterConfig) {
    for _ in 0..config.count {
        // Random offset
        let offset = Vec3::new(
            (rand::random::<f32>() - 0.5) * config.position_offset.x * 2.0,
            (rand::random::<f32>() - 0.5) * config.position_offset.y * 2.0,
            (rand::random::<f32>() - 0.5) * config.position_offset.z * 2.0,
        );

        // Random velocity
        let t = rand::random::<f32>();
        let velocity = Vec3::new(
            config.velocity_range.0.x + (config.velocity_range.1.x - config.velocity_range.0.x) * t,
            config.velocity_range.0.y + (config.velocity_range.1.y - config.velocity_range.0.y) * rand::random::<f32>(),
            config.velocity_range.0.z + (config.velocity_range.1.z - config.velocity_range.0.z) * rand::random::<f32>(),
        );

        // Random lifetime
        let lifetime = config.lifetime_range.0 
            + (config.lifetime_range.1 - config.lifetime_range.0) * rand::random::<f32>();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: config.color_start,
                    custom_size: Some(Vec2::splat(config.size_range.0)),
                    ..default()
                },
                transform: Transform::from_translation(position + offset),
                ..default()
            },
            Particle {
                velocity,
                age: 0.0,
                lifetime,
                start_size: config.size_range.0,
                end_size: config.size_range.1,
                start_color: config.color_start,
                end_color: config.color_end,
                gravity: config.gravity,
                drag: config.drag,
            },
        ));
    }
}

/// Preset particle effects
pub fn explosion_particles(position: Vec3) -> ParticleEmitterConfig {
    ParticleEmitterConfig {
        count: 30,
        position_offset: Vec3::new(5.0, 5.0, 0.0),
        velocity_range: (Vec3::new(-100.0, 50.0, -20.0), Vec3::new(100.0, 150.0, 20.0)),
        lifetime_range: (0.3, 0.8),
        size_range: (4.0, 0.5),
        color_start: Color::srgba(1.0, 0.6, 0.1, 1.0),
        color_end: Color::srgba(0.3, 0.1, 0.0, 0.0),
        gravity: -150.0,
        drag: 2.0,
    }
}

pub fn smoke_particles(position: Vec3) -> ParticleEmitterConfig {
    ParticleEmitterConfig {
        count: 15,
        position_offset: Vec3::new(3.0, 3.0, 0.0),
        velocity_range: (Vec3::new(-20.0, 20.0, -5.0), Vec3::new(20.0, 50.0, 5.0)),
        lifetime_range: (1.0, 2.5),
        size_range: (3.0, 8.0),
        color_start: Color::srgba(0.5, 0.5, 0.5, 0.7),
        color_end: Color::srgba(0.3, 0.3, 0.3, 0.0),
        gravity: 10.0,
        drag: 0.3,
    }
}

pub fn spark_particles(position: Vec3) -> ParticleEmitterConfig {
    ParticleEmitterConfig {
        count: 20,
        position_offset: Vec3::new(2.0, 2.0, 0.0),
        velocity_range: (Vec3::new(-80.0, -80.0, -10.0), Vec3::new(80.0, 80.0, 10.0)),
        lifetime_range: (0.2, 0.5),
        size_range: (2.0, 0.2),
        color_start: Color::srgba(1.0, 1.0, 0.5, 1.0),
        color_end: Color::srgba(1.0, 0.5, 0.0, 0.0),
        gravity: -50.0,
        drag: 1.0,
    }
}

pub fn resource_pickup_particles(position: Vec3, color: Color) -> ParticleEmitterConfig {
    ParticleEmitterConfig {
        count: 8,
        position_offset: Vec3::new(3.0, 3.0, 0.0),
        velocity_range: (Vec3::new(-30.0, 30.0, -5.0), Vec3::new(30.0, 60.0, 5.0)),
        lifetime_range: (0.3, 0.6),
        size_range: (3.0, 1.0),
        color_start: color,
        color_end: Color::srgba(color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, 0.0),
        gravity: -20.0,
        drag: 0.8,
    }
}

/// Update particle system
pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut particle, mut transform, mut sprite) in query.iter_mut() {
        // Extract all particle values first
        let (lifetime, gravity, drag, start_size, end_size, start_color, end_color) = {
            let p = &*particle;
            (p.lifetime, p.gravity, p.drag, p.start_size, p.end_size, p.start_color, p.end_color)
        };
        
        particle.age += dt;

        // Calculate progress (0 to 1)
        let progress = (particle.age / lifetime).min(1.0);

        // Update position
        transform.translation += particle.velocity * dt;

        // Apply gravity
        particle.velocity.y += gravity * dt;

        // Apply drag
        particle.velocity *= 1.0 - drag * dt;

        // Interpolate size
        let size = start_size + (end_size - start_size) * progress;
        sprite.custom_size = Some(Vec2::splat(size));

        // Interpolate color
        let start = start_color.to_srgba();
        let end = end_color.to_srgba();
        sprite.color = Color::srgba(
            start.red + (end.red - start.red) * progress,
            start.green + (end.green - start.green) * progress,
            start.blue + (end.blue - start.blue) * progress,
            start.alpha + (end.alpha - start.alpha) * progress,
        );

        // Despawn when lifetime exceeded
        if particle.age >= lifetime {
            commands.entity(entity).despawn();
        }
    }
}

/// Event for particle effects
#[derive(Event, Debug, Clone)]
pub enum ParticleEvent {
    Explosion { position: Vec3 },
    Smoke { position: Vec3 },
    Sparks { position: Vec3 },
    ResourcePickup { position: Vec3, color: Color },
    Death { position: Vec3 },
}

/// System to handle particle events
pub fn particle_event_system(
    mut events: EventReader<ParticleEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            ParticleEvent::Explosion { position } => {
                spawn_particles(&mut commands, *position, explosion_particles(*position));
                spawn_particles(&mut commands, *position, smoke_particles(*position));
            }
            ParticleEvent::Smoke { position } => {
                spawn_particles(&mut commands, *position, smoke_particles(*position));
            }
            ParticleEvent::Sparks { position } => {
                spawn_particles(&mut commands, *position, spark_particles(*position));
            }
            ParticleEvent::ResourcePickup { position, color } => {
                spawn_particles(&mut commands, *position, resource_pickup_particles(*position, *color));
            }
            ParticleEvent::Death { position } => {
                spawn_particles(&mut commands, *position, smoke_particles(*position));
            }
        }
    }
}

/// Plugin for particle system
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ParticleEvent>()
            .add_systems(Update, (particle_event_system, update_particles));
    }
}