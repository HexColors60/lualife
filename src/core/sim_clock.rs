use bevy::prelude::*;

use crate::config::GameConfig;

#[derive(Resource, Debug, Clone)]
pub struct SimClock {
    /// Accumulated time since last tick
    pub accumulator: f32,
    /// Ticks per second
    pub tick_rate: u64,
    /// Speed multiplier
    pub speed: f32,
}

impl Default for SimClock {
    fn default() -> Self {
        Self {
            accumulator: 0.0,
            tick_rate: 20,
            speed: 1.0,
        }
    }
}

impl SimClock {
    pub fn tick_duration(&self) -> f32 {
        if self.tick_rate == 0 {
            return f32::MAX;
        }
        (1.0 / self.tick_rate as f32) / self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.1, 10.0);
    }

    pub fn set_tick_rate(&mut self, rate: u64) {
        self.tick_rate = rate.max(1);
    }
}

impl From<&GameConfig> for SimClock {
    fn from(config: &GameConfig) -> Self {
        Self {
            accumulator: 0.0,
            tick_rate: config.tick_rate,
            speed: config.sim_speed,
        }
    }
}
