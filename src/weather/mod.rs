//! Weather system for dynamic environmental conditions.
//!
//! Provides weather types (Clear, Rain, Snow, Storm, Fog, Heatwave) with
//! gameplay effects on movement, visibility, combat, and resources.

use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::core::TickNumber;
use crate::events_world::Season;

/// Weather types with distinct gameplay effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WeatherType {
    #[default]
    Clear,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
    Heatwave,
}

impl WeatherType {
    /// Get display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            WeatherType::Clear => "Clear",
            WeatherType::Cloudy => "Cloudy",
            WeatherType::Rain => "Rain",
            WeatherType::Storm => "Storm",
            WeatherType::Snow => "Snow",
            WeatherType::Fog => "Fog",
            WeatherType::Heatwave => "Heatwave",
        }
    }

    /// Movement speed multiplier (1.0 = normal)
    pub fn movement_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 1.0,
            WeatherType::Rain => 0.85,
            WeatherType::Storm => 0.6,
            WeatherType::Snow => 0.7,
            WeatherType::Fog => 0.9,
            WeatherType::Heatwave => 0.95,
        }
    }

    /// Visibility range multiplier (1.0 = normal)
    pub fn visibility_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 0.95,
            WeatherType::Rain => 0.8,
            WeatherType::Storm => 0.5,
            WeatherType::Snow => 0.6,
            WeatherType::Fog => 0.4,
            WeatherType::Heatwave => 0.9,
        }
    }

    /// Combat accuracy multiplier (1.0 = normal)
    pub fn combat_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 1.0,
            WeatherType::Rain => 0.9,
            WeatherType::Storm => 0.7,
            WeatherType::Snow => 0.85,
            WeatherType::Fog => 0.75,
            WeatherType::Heatwave => 0.95,
        }
    }

    /// Mining efficiency multiplier (1.0 = normal)
    pub fn mining_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 1.0,
            WeatherType::Rain => 0.9,
            WeatherType::Storm => 0.5,
            WeatherType::Snow => 0.75,
            WeatherType::Fog => 0.9,
            WeatherType::Heatwave => 0.85,
        }
    }

    /// Power consumption multiplier (1.0 = normal)
    pub fn power_consumption_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 1.0,
            WeatherType::Rain => 1.1,
            WeatherType::Storm => 1.2,
            WeatherType::Snow => 1.3,
            WeatherType::Fog => 1.05,
            WeatherType::Heatwave => 1.15,
        }
    }

    /// Building efficiency multiplier (1.0 = normal)
    pub fn building_efficiency_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 1.0,
            WeatherType::Rain => 0.85,
            WeatherType::Storm => 0.4,
            WeatherType::Snow => 0.7,
            WeatherType::Fog => 0.95,
            WeatherType::Heatwave => 0.9,
        }
    }

    /// Tower range multiplier (1.0 = normal)
    pub fn tower_range_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 0.95,
            WeatherType::Rain => 0.85,
            WeatherType::Storm => 0.6,
            WeatherType::Snow => 0.8,
            WeatherType::Fog => 0.5,
            WeatherType::Heatwave => 0.9,
        }
    }

    /// Get weather probability weights for a given season
    pub fn season_weights(season: Season) -> [(WeatherType, f32); 7] {
        match season {
            Season::Spring => [
                (WeatherType::Clear, 35.0),
                (WeatherType::Cloudy, 25.0),
                (WeatherType::Rain, 25.0),
                (WeatherType::Storm, 5.0),
                (WeatherType::Snow, 0.0),
                (WeatherType::Fog, 8.0),
                (WeatherType::Heatwave, 2.0),
            ],
            Season::Summer => [
                (WeatherType::Clear, 45.0),
                (WeatherType::Cloudy, 20.0),
                (WeatherType::Rain, 10.0),
                (WeatherType::Storm, 10.0),
                (WeatherType::Snow, 0.0),
                (WeatherType::Fog, 5.0),
                (WeatherType::Heatwave, 10.0),
            ],
            Season::Autumn => [
                (WeatherType::Clear, 25.0),
                (WeatherType::Cloudy, 30.0),
                (WeatherType::Rain, 25.0),
                (WeatherType::Storm, 8.0),
                (WeatherType::Snow, 2.0),
                (WeatherType::Fog, 10.0),
                (WeatherType::Heatwave, 0.0),
            ],
            Season::Winter => [
                (WeatherType::Clear, 20.0),
                (WeatherType::Cloudy, 25.0),
                (WeatherType::Rain, 5.0),
                (WeatherType::Storm, 15.0),
                (WeatherType::Snow, 30.0),
                (WeatherType::Fog, 5.0),
                (WeatherType::Heatwave, 0.0),
            ],
        }
    }
}

/// Weather forecast entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub weather: WeatherType,
    pub start_tick: u64,
    pub duration_ticks: u64,
}

/// Global weather state
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct WeatherState {
    pub current_weather: WeatherType,
    pub weather_start_tick: u64,
    pub weather_duration: u64,
    pub forecast: VecDeque<WeatherForecast>,
    pub transition_progress: f32,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            weather_start_tick: 0,
            weather_duration: 500,
            forecast: VecDeque::new(),
            transition_progress: 0.0,
        }
    }
}

impl WeatherState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if weather should change
    pub fn should_change_weather(&self, current_tick: u64) -> bool {
        current_tick >= self.weather_start_tick + self.weather_duration
    }

    /// Get remaining ticks for current weather
    pub fn remaining_ticks(&self, current_tick: u64) -> u64 {
        self.weather_duration.saturating_sub(current_tick.saturating_sub(self.weather_start_tick))
    }

    /// Get progress through current weather (0.0 to 1.0)
    pub fn progress(&self, current_tick: u64) -> f32 {
        let elapsed = current_tick.saturating_sub(self.weather_start_tick);
        if self.weather_duration == 0 {
            1.0
        } else {
            (elapsed as f32 / self.weather_duration as f32).min(1.0)
        }
    }

    /// Set new weather
    pub fn set_weather(&mut self, weather: WeatherType, tick: u64, duration: u64) {
        self.current_weather = weather;
        self.weather_start_tick = tick;
        self.weather_duration = duration;
        self.transition_progress = 0.0;
    }

    /// Generate weather forecast
    pub fn generate_forecast(&mut self, season: Season, current_tick: u64, count: usize) {
        let mut rng = rand::thread_rng();
        let mut tick = current_tick + self.weather_duration;

        for _ in 0..count {
            let weather = Self::random_weather_for_season(season, &mut rng);
            let duration = rng.gen_range(300..800);

            self.forecast.push_back(WeatherForecast {
                weather,
                start_tick: tick,
                duration_ticks: duration,
            });

            tick += duration;
        }
    }

    /// Get random weather based on season weights
    pub fn random_weather_for_season(season: Season, rng: &mut impl Rng) -> WeatherType {
        let weights = WeatherType::season_weights(season);
        let total: f32 = weights.iter().map(|(_, w)| w).sum();

        if total == 0.0 {
            return WeatherType::Clear;
        }

        let mut roll = rng.gen::<f32>() * total;

        for (weather, weight) in weights {
            if roll < weight {
                return weather;
            }
            roll -= weight;
        }

        WeatherType::Clear
    }
}

/// Weather change event
#[derive(Event, Debug, Clone)]
pub struct WeatherChangedEvent {
    pub old_weather: WeatherType,
    pub new_weather: WeatherType,
}

/// System to update weather based on time and season
pub fn weather_transition_system(
    mut weather_state: ResMut<WeatherState>,
    mut events: EventWriter<WeatherChangedEvent>,
    tick: Res<TickNumber>,
    season: Option<Res<crate::events_world::WorldEventManager>>,
) {
    let current_tick = tick.0;

    // Check if we should change weather
    if weather_state.should_change_weather(current_tick) {
        let old_weather = weather_state.current_weather;

        // Try to get next weather from forecast
        let (new_weather, duration) = if let Some(forecast) = weather_state.forecast.pop_front() {
            (forecast.weather, forecast.duration_ticks)
        } else {
            // Generate new random weather
            let current_season = season
                .as_ref()
                .map(|s| s.current_season)
                .unwrap_or(Season::Spring);

            let mut rng = rand::thread_rng();
            let weather = WeatherState::random_weather_for_season(current_season, &mut rng);
            let duration = rng.gen_range(300..800);
            (weather, duration)
        };

        // Update weather
        weather_state.set_weather(new_weather, current_tick, duration);

        // Regenerate forecast if low
        if weather_state.forecast.len() < 3 {
            let current_season = season
                .as_ref()
                .map(|s| s.current_season)
                .unwrap_or(Season::Spring);
            weather_state.generate_forecast(current_season, current_tick, 5);
        }

        // Emit event
        events.send(WeatherChangedEvent {
            old_weather,
            new_weather,
        });
    }
}

/// System to apply weather effects to game mechanics
pub fn apply_weather_effects_system(
    weather_state: Res<WeatherState>,
    mut game_log: Option<ResMut<crate::ui::GameLog>>,
) {
    // This system can be expanded to apply effects to specific entities
    // For now, the weather modifiers are read directly by other systems

    // Log significant weather events
    if weather_state.is_changed() {
        if let Some(log) = &mut game_log {
            match weather_state.current_weather {
                WeatherType::Storm => {
                    log.add("⚠ Storm brewing! Movement and visibility reduced.".to_string());
                }
                WeatherType::Heatwave => {
                    log.add("🌡 Heatwave! Power consumption increased.".to_string());
                }
                WeatherType::Snow => {
                    log.add("❄ Heavy snow! Movement and mining reduced.".to_string());
                }
                _ => {}
            }
        }
    }
}

/// Plugin for weather system
pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeatherState>()
            .add_event::<WeatherChangedEvent>()
            .add_systems(
                Update,
                (weather_transition_system, apply_weather_effects_system).chain(),
            );
    }
}