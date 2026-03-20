//! Day/night cycle system for atmospheric lighting.

use bevy::prelude::*;

/// Time of day phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeOfDay {
    #[default]
    Dawn,
    Day,
    Dusk,
    Night,
}

/// Day/night cycle state
#[derive(Resource, Debug, Clone)]
pub struct DayNightCycle {
    /// Current time (0.0 to 24.0)
    pub time: f32,
    /// Time speed multiplier
    pub speed: f32,
    /// Current phase
    pub phase: TimeOfDay,
    /// Ambient light color
    pub ambient_color: Color,
    /// Ambient light intensity
    pub ambient_intensity: f32,
    /// Is cycle paused?
    pub paused: bool,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time: 10.0, // Start at 10 AM
            speed: 1.0, // 1 game hour = 1 real minute at speed 1.0
            phase: TimeOfDay::Day,
            ambient_color: Color::srgb(1.0, 1.0, 0.95),
            ambient_intensity: 1.0,
            paused: false,
        }
    }
}

impl DayNightCycle {
    /// Create a new day/night cycle
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the time string (HH:MM format)
    pub fn time_string(&self) -> String {
        let hours = self.time.floor() as u8;
        let minutes = ((self.time % 1.0) * 60.0) as u8;
        format!("{:02}:{:02}", hours, minutes)
    }

    /// Get the day number (0-indexed)
    pub fn day_number(&self) -> u32 {
        (self.time / 24.0) as u32
    }

    /// Get time progress through current day (0.0 to 1.0)
    pub fn day_progress(&self) -> f32 {
        (self.time % 24.0) / 24.0
    }
}

/// Update the day/night cycle
pub fn update_day_night_cycle(
    time: Res<Time>,
    mut cycle: ResMut<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
) {
    if cycle.paused {
        return;
    }

    // Advance time (1 game hour = 1 real minute at speed 1.0)
    let game_hours_per_second = cycle.speed / 60.0;
    cycle.time += game_hours_per_second * time.delta_seconds();

    // Wrap around at 24 hours
    cycle.time = cycle.time % 24.0;

    // Determine phase and colors
    let hour = cycle.time;
    
    let (phase, ambient_color, ambient_intensity, bg_color) = if hour < 5.0 {
        // Night (midnight to 5 AM)
        (TimeOfDay::Night, 
         Color::srgb(0.3, 0.3, 0.5),
         0.3,
         Color::srgb(0.02, 0.02, 0.05))
    } else if hour < 7.0 {
        // Dawn (5 AM to 7 AM)
        let t = (hour - 5.0) / 2.0;
        (TimeOfDay::Dawn,
         Color::srgb(
             0.3 + 0.7 * t,
             0.3 + 0.5 * t,
             0.5 + 0.45 * t,
         ),
         0.3 + 0.7 * t,
         Color::srgb(
             0.02 + 0.05 * t,
             0.02 + 0.05 * t,
             0.05 + 0.07 * t,
         ))
    } else if hour < 18.0 {
        // Day (7 AM to 6 PM)
        (TimeOfDay::Day,
         Color::srgb(1.0, 0.98, 0.92),
         1.0,
         Color::srgb(0.07, 0.07, 0.12))
    } else if hour < 20.0 {
        // Dusk (6 PM to 8 PM)
        let t = (hour - 18.0) / 2.0;
        (TimeOfDay::Dusk,
         Color::srgb(
             1.0 - 0.5 * t,
             0.98 - 0.5 * t,
             0.92 - 0.3 * t,
         ),
         1.0 - 0.5 * t,
         Color::srgb(
             0.07 + 0.1 * t,
             0.03 + 0.07 * t,
             0.12 + 0.08 * t,
         ))
    } else {
        // Night (8 PM to midnight)
        (TimeOfDay::Night,
         Color::srgb(0.3, 0.3, 0.5),
         0.3,
         Color::srgb(0.02, 0.02, 0.05))
    };

    cycle.phase = phase;
    cycle.ambient_color = ambient_color;
    cycle.ambient_intensity = ambient_intensity;
    
    // Update background clear color
    **clear_color = bg_color;
}

/// Toggle day/night cycle pause
pub fn toggle_day_night_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cycle: ResMut<DayNightCycle>,
) {
    // N key toggles day/night pause
    if keyboard.just_pressed(KeyCode::KeyN) {
        cycle.paused = !cycle.paused;
    }
}

/// Speed up or slow down time
pub fn adjust_day_night_speed(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cycle: ResMut<DayNightCycle>,
) {
    // [ and ] keys adjust speed
    if keyboard.just_pressed(KeyCode::BracketLeft) {
        cycle.speed = (cycle.speed / 2.0).max(0.125);
    }
    if keyboard.just_pressed(KeyCode::BracketRight) {
        cycle.speed = (cycle.speed * 2.0).min(16.0);
    }
}

/// Skip to a specific time of day
pub fn skip_to_time(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cycle: ResMut<DayNightCycle>,
) {
    // Shift + 1-4 to skip to different times
    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        if keyboard.just_pressed(KeyCode::Digit1) {
            cycle.time = 6.0; // Dawn
        } else if keyboard.just_pressed(KeyCode::Digit2) {
            cycle.time = 12.0; // Noon
        } else if keyboard.just_pressed(KeyCode::Digit3) {
            cycle.time = 18.0; // Dusk
        } else if keyboard.just_pressed(KeyCode::Digit4) {
            cycle.time = 0.0; // Midnight
        }
    }
}

/// Plugin for day/night cycle system
pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DayNightCycle>()
            .add_systems(Update, (
                update_day_night_cycle,
                toggle_day_night_pause,
                adjust_day_night_speed,
                skip_to_time,
            ));
    }
}