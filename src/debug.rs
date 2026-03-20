//! Debug module providing GOD mode, debug overlays, and development tools.
//!
//! This module is only for development and debugging purposes.

use bevy::prelude::*;

/// Plugin that registers debug resources and systems.
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GodMode>()
            .init_resource::<DebugOverlays>()
            .init_resource::<SelectionState>()
            .init_resource::<PerfMetrics>();
    }
}

/// Resource controlling GOD mode capabilities.
///
/// When enabled, the user has full visibility and control over the simulation.
#[derive(Resource, Debug, Clone)]
pub struct GodMode {
    /// Whether GOD mode is currently active.
    pub enabled: bool,
    /// Simulation speed multiplier (1.0 = normal, 2.0 = double speed, etc.)
    pub speed: f32,
    /// Whether the simulation is paused.
    pub paused: bool,
}

impl Default for GodMode {
    fn default() -> Self {
        Self {
            enabled: true, // GOD mode is on by default
            speed: 1.0,
            paused: false,
        }
    }
}

/// Resource controlling which debug overlays are visible.
#[derive(Resource, Debug, Clone)]
pub struct DebugOverlays {
    /// Show terrain grid overlay.
    pub show_grid: bool,
    /// Show creep movement paths.
    pub show_paths: bool,
    /// Show danger zones (tower ranges, hostile areas).
    pub show_danger_zones: bool,
    /// Show room border lines.
    pub show_room_borders: bool,
    /// Show room ownership colors.
    pub show_ownership: bool,
    /// Show mine richness indicators.
    pub show_mine_richness: bool,
}

impl Default for DebugOverlays {
    fn default() -> Self {
        Self {
            show_grid: false,
            show_paths: false,
            show_danger_zones: false,
            show_room_borders: false,
            show_ownership: false,
            show_mine_richness: false,
        }
    }
}

/// Resource tracking the currently selected entity.
#[derive(Resource, Debug, Clone, Default)]
pub struct SelectionState {
    /// The currently selected entity, if any.
    pub selected_entity: Option<Entity>,
}

impl SelectionState {
    /// Select an entity.
    pub fn select(&mut self, entity: Entity) {
        self.selected_entity = Some(entity);
    }

    /// Clear the current selection.
    pub fn deselect(&mut self) {
        self.selected_entity = None;
    }

    /// Check if an entity is currently selected.
    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected_entity == Some(entity)
    }
}

/// Entity count metrics for performance display.
#[derive(Debug, Clone, Default)]
pub struct EntityCounts {
    pub creeps: usize,
    pub buildings: usize,
    pub mines: usize,
    pub resources: usize,
}

/// Resource tracking performance metrics for debug display.
#[derive(Resource, Debug, Clone)]
pub struct PerfMetrics {
    /// Tick timing history for averaging (in milliseconds).
    pub tick_times: Vec<f32>,
    /// Maximum number of tick times to keep for averaging.
    pub max_tick_samples: usize,
    /// Current entity counts.
    pub entity_counts: EntityCounts,
    /// Current FPS estimate.
    pub fps: f32,
    /// Lua execution time per tick (milliseconds).
    pub lua_time_ms: f32,
    /// Pathfinding time per tick (milliseconds).
    pub pathfinding_time_ms: f32,
}

impl PerfMetrics {
    /// Calculate the average tick time over recorded samples.
    pub fn avg_tick_time(&self) -> f32 {
        if self.tick_times.is_empty() {
            return 0.0;
        }
        self.tick_times.iter().sum::<f32>() / self.tick_times.len() as f32
    }

    /// Record a new tick time sample.
    pub fn record_tick_time(&mut self, time_ms: f32) {
        self.tick_times.push(time_ms);
        if self.tick_times.len() > self.max_tick_samples {
            self.tick_times.remove(0);
        }
    }
}

impl Default for PerfMetrics {
    fn default() -> Self {
        Self {
            tick_times: Vec::with_capacity(60),
            max_tick_samples: 60,
            entity_counts: EntityCounts::default(),
            fps: 0.0,
            lua_time_ms: 0.0,
            pathfinding_time_ms: 0.0,
        }
    }
}