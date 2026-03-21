//! Visual weather effects (rain, snow, fog, storm)
//!
//! Provides particle-based weather visualization that responds to WeatherState.

use bevy::prelude::*;
use rand::Rng;

use crate::weather::{WeatherState, WeatherType};

/// Marker component for weather particle entities
#[derive(Component)]
pub struct WeatherParticle;

/// Rain drop component
#[derive(Component)]
pub struct RainDrop {
    pub velocity: Vec3,
}

/// Snow flake component
#[derive(Component)]
pub struct SnowFlake {
    pub wobble: f32,
    pub wobble_speed: f32,
}

/// Fog overlay component
#[derive(Component)]
pub struct FogOverlay;

/// Lightning flash component
#[derive(Component)]
pub struct LightningFlash {
    pub timer: Timer,
    pub intensity: f32,
}

/// Weather effects state
#[derive(Resource, Default)]
pub struct WeatherEffectsState {
    pub particle_spawn_timer: Timer,
    pub lightning_timer: Timer,
    pub fog_entity: Option<Entity>,
}

/// Spawn weather particles based on current weather
pub fn spawn_weather_particles(
    mut commands: Commands,
    mut state: ResMut<WeatherEffectsState>,
    weather: Res<WeatherState>,
    time: Res<Time>,
    camera: Query<&Transform, With<Camera>>,
) {
    // Update spawn timer
    state.particle_spawn_timer.tick(time.delta());

    if !state.particle_spawn_timer.finished() {
        return;
    }

    // Get camera position for spawning around visible area
    let camera_pos = camera.iter().next().map(|t| t.translation).unwrap_or(Vec3::ZERO);

    let mut rng = rand::thread_rng();
    let spawn_area = 400.0; // Spawn particles in a 400-unit radius around camera

    match weather.current_weather {
        WeatherType::Rain => {
            // Spawn multiple rain drops
            for _ in 0..5 {
                let x = camera_pos.x + rng.gen_range(-spawn_area..spawn_area);
                let y = camera_pos.y + spawn_area + rng.gen_range(0.0..100.0);

                commands.spawn((
                    Sprite {
                        color: Color::srgba(0.6, 0.7, 0.9, 0.6),
                        custom_size: Some(Vec2::new(1.0, 8.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 50.0),
                    RainDrop {
                        velocity: Vec3::new(rng.gen_range(-1.0..0.0), rng.gen_range(-400.0..-300.0), 0.0),
                    },
                    WeatherParticle,
                ));
            }
        }
        WeatherType::Storm => {
            // Spawn more rain drops for storm
            for _ in 0..10 {
                let x = camera_pos.x + rng.gen_range(-spawn_area..spawn_area);
                let y = camera_pos.y + spawn_area + rng.gen_range(0.0..100.0);

                commands.spawn((
                    Sprite {
                        color: Color::srgba(0.5, 0.6, 0.8, 0.7),
                        custom_size: Some(Vec2::new(2.0, 12.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 50.0),
                    RainDrop {
                        velocity: Vec3::new(rng.gen_range(-50.0..-20.0), rng.gen_range(-500.0..-400.0), 0.0),
                    },
                    WeatherParticle,
                ));
            }
        }
        WeatherType::Snow => {
            // Spawn snowflakes
            for _ in 0..3 {
                let x = camera_pos.x + rng.gen_range(-spawn_area..spawn_area);
                let y = camera_pos.y + spawn_area + rng.gen_range(0.0..100.0);

                commands.spawn((
                    Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 50.0),
                    SnowFlake {
                        wobble: rng.gen_range(0.0..std::f32::consts::TAU),
                        wobble_speed: rng.gen_range(2.0..5.0),
                    },
                    WeatherParticle,
                ));
            }
        }
        _ => {}
    }
}

/// Update weather particle positions and despawn when off-screen
pub fn update_weather_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Transform, Option<&mut RainDrop>, Option<&mut SnowFlake>), With<WeatherParticle>>,
    time: Res<Time>,
    camera: Query<&Transform, With<Camera>>,
) {
    let camera_pos = camera.iter().next().map(|t| t.translation).unwrap_or(Vec3::ZERO);
    let despawn_distance = 500.0;

    for (entity, mut transform, rain, snow) in particles.iter_mut() {
        // Update rain drops
        if let Some(mut rain) = rain {
            transform.translation += rain.velocity * time.delta_seconds();
        }

        // Update snow flakes
        if let Some(mut snow) = snow {
            snow.wobble += snow.wobble_speed * time.delta_seconds();
            transform.translation.x += (snow.wobble.sin() * 20.0) * time.delta_seconds();
            transform.translation.y -= 50.0 * time.delta_seconds();
        }

        // Despawn if too far from camera
        let dist = transform.translation.distance(camera_pos);
        if dist > despawn_distance || transform.translation.y < camera_pos.y - despawn_distance {
            commands.entity(entity).despawn();
        }
    }
}

/// Manage fog overlay based on weather
pub fn manage_fog_overlay(
    mut commands: Commands,
    mut state: ResMut<WeatherEffectsState>,
    weather: Res<WeatherState>,
    fog_query: Query<Entity, With<FogOverlay>>,
) {
    let should_have_fog = matches!(weather.current_weather, WeatherType::Fog | WeatherType::Storm);

    if should_have_fog && state.fog_entity.is_none() {
        // Spawn fog overlay
        let entity = commands.spawn((
            Sprite {
                color: Color::srgba(0.5, 0.5, 0.6, match weather.current_weather {
                    WeatherType::Fog => 0.4,
                    WeatherType::Storm => 0.25,
                    _ => 0.0,
                }),
                custom_size: Some(Vec2::new(2000.0, 2000.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 100.0),
            FogOverlay,
        )).id();
        state.fog_entity = Some(entity);
    } else if !should_have_fog {
        // Despawn fog overlay
        if let Some(entity) = state.fog_entity {
            commands.entity(entity).despawn();
            state.fog_entity = None;
        }
    }
}

/// Spawn lightning flashes during storms
pub fn spawn_lightning(
    mut commands: Commands,
    mut state: ResMut<WeatherEffectsState>,
    weather: Res<WeatherState>,
    time: Res<Time>,
    mut shake_events: EventWriter<crate::render::ScreenShakeEvent>,
) {
    // Only spawn lightning during storms
    if weather.current_weather != WeatherType::Storm {
        state.lightning_timer.reset();
        return;
    }

    state.lightning_timer.tick(time.delta());

    if !state.lightning_timer.finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Random chance for lightning
    if rng.gen::<f32>() < 0.3 {
        // Spawn lightning flash
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.8),
                custom_size: Some(Vec2::new(2000.0, 2000.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 150.0),
            LightningFlash {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
                intensity: rng.gen_range(0.5..1.0),
            },
            WeatherParticle,
        ));

        // Screen shake for thunder
        shake_events.send(crate::render::ScreenShakeEvent::from_type(
            crate::render::ShakeEventType::Heavy,
        ));
    }

    // Reset timer with random duration
    state.lightning_timer = Timer::from_seconds(rng.gen_range(2.0..6.0), TimerMode::Once);
}

/// Update lightning flash
pub fn update_lightning(
    mut commands: Commands,
    mut lightning: Query<(Entity, &mut LightningFlash, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut flash, mut sprite) in lightning.iter_mut() {
        flash.timer.tick(time.delta());

        // Fade out
        let progress = flash.timer.fraction();
        sprite.color = Color::srgba(1.0, 1.0, 1.0, flash.intensity * (1.0 - progress));

        if flash.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Clean up weather particles when weather changes
pub fn cleanup_weather_particles(
    mut commands: Commands,
    weather: Res<WeatherState>,
    particles: Query<Entity, With<WeatherParticle>>,
) {
    if weather.is_changed() {
        // Despawn all weather particles when weather changes
        for entity in particles.iter() {
            commands.entity(entity).despawn();
        }
    }
}

/// Plugin for weather visual effects
pub struct WeatherEffectsPlugin;

impl Plugin for WeatherEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WeatherEffectsState {
            particle_spawn_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            lightning_timer: Timer::from_seconds(3.0, TimerMode::Once),
            fog_entity: None,
        })
        .add_systems(
            Update,
            (
                spawn_weather_particles,
                update_weather_particles,
                manage_fog_overlay,
                spawn_lightning,
                update_lightning,
                cleanup_weather_particles,
            ).chain(),
        );
    }
}