//! Screen shake system for impactful visual feedback.

use bevy::prelude::*;

use crate::render::MainCamera;

/// Screen shake state
#[derive(Resource, Debug, Clone, Default)]
pub struct ScreenShake {
    /// Current shake offset
    pub offset: Vec3,
    /// Shake intensity (decays over time)
    pub intensity: f32,
    /// Shake duration remaining
    pub duration: f32,
    /// Max duration for decay calculation
    pub max_duration: f32,
    /// Random seed for shake pattern
    pub seed: f32,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Trigger a screen shake effect
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = self.intensity.max(intensity);
        self.duration = self.duration.max(duration);
        self.max_duration = self.max_duration.max(duration);
        self.seed = rand::random();
    }

    /// Check if shake is active
    pub fn is_shaking(&self) -> bool {
        self.duration > 0.0 && self.intensity > 0.0
    }
}

/// Screen shake event
#[derive(Event, Debug, Clone)]
pub struct ScreenShakeEvent {
    /// Shake intensity (pixels)
    pub intensity: f32,
    /// Duration in seconds
    pub duration: f32,
    /// Event type for preset values
    pub event_type: ShakeEventType,
}

/// Types of events that cause screen shake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShakeEventType {
    /// Light shake - resource pickup, small events
    Light,
    /// Medium shake - creep death, combat hit
    Medium,
    /// Heavy shake - building destruction, large explosion
    Heavy,
    /// Impact shake - tower shot, major attack
    Impact,
}

impl ScreenShakeEvent {
    /// Create a shake event with preset values based on event type
    pub fn from_type(event_type: ShakeEventType) -> Self {
        let (intensity, duration) = match event_type {
            ShakeEventType::Light => (2.0, 0.1),
            ShakeEventType::Medium => (5.0, 0.2),
            ShakeEventType::Heavy => (10.0, 0.4),
            ShakeEventType::Impact => (8.0, 0.3),
        };
        Self { intensity, duration, event_type }
    }
}

/// System to handle screen shake events
pub fn screen_shake_event_system(
    mut events: EventReader<ScreenShakeEvent>,
    mut shake: ResMut<ScreenShake>,
) {
    for event in events.read() {
        shake.trigger(event.intensity, event.duration);
    }
}

/// System to update screen shake effect
pub fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    if shake.duration <= 0.0 {
        // Reset offset when not shaking
        if shake.offset != Vec3::ZERO {
            shake.offset = Vec3::ZERO;
        }
        return;
    }

    // Update duration
    shake.duration -= time.delta_seconds();

    // Calculate decay factor
    let decay = if shake.max_duration > 0.0 {
        (shake.duration / shake.max_duration).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Generate shake offset using noise-like pattern
    let t = time.elapsed_seconds() * 20.0 + shake.seed;
    let shake_x = (t.sin() * 13.5 + (t * 1.7).cos() * 7.3) * shake.intensity * decay;
    let shake_y = (t.cos() * 11.3 + (t * 2.3).sin() * 9.7) * shake.intensity * decay;

    shake.offset = Vec3::new(shake_x, shake_y, 0.0);

    // Apply to camera
    for mut transform in camera.iter_mut() {
        transform.translation.x += shake.offset.x;
        transform.translation.y += shake.offset.y;
    }
}

/// Plugin for screen shake system
pub struct ScreenShakePlugin;

impl Plugin for ScreenShakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>()
            .add_event::<ScreenShakeEvent>()
            .add_systems(Update, (screen_shake_event_system, update_screen_shake));
    }
}