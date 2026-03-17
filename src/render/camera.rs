use bevy::prelude::*;

use crate::config::UiConfig;
use crate::consts::{WORLD_TILES_X, WORLD_TILES_Y};

#[derive(Debug, Clone, Component)]
pub struct MainCamera;

#[derive(Resource, Debug, Clone)]
pub struct CameraState {
    pub zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub pan_speed: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            pan_speed: 500.0,
        }
    }
}

pub fn camera_control_system(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<UiConfig>,
) {
    let speed = config.camera_pan_speed * time.delta_seconds();

    for mut transform in camera.iter_mut() {
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += speed;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= speed;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }

        // Clamp camera to world bounds
        transform.translation.x = transform.translation.x.clamp(0.0, WORLD_TILES_X as f32);
        transform.translation.y = transform.translation.y.clamp(0.0, WORLD_TILES_Y as f32);
    }
}

pub fn camera_zoom_system(
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    config: Res<UiConfig>,
) {
    for event in scroll_events.read() {
        for mut projection in camera.iter_mut() {
            projection.scale = (projection.scale + event.y * config.camera_zoom_speed)
                .clamp(0.1, 5.0);
        }
    }
}