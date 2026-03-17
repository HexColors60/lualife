use bevy::prelude::*;

use crate::debug::PerfMetrics;

pub fn perf_panel_system(
    perf: Res<PerfMetrics>,
    time: Res<Time>,
) {
    // Update FPS calculation
    let fps = 1.0 / time.delta_seconds();
    tracing::debug!(
        "FPS: {:.1}, Avg tick: {:.2}ms, Entities: creeps={}, buildings={}",
        fps,
        perf.avg_tick_time(),
        perf.entity_counts.creeps,
        perf.entity_counts.buildings
    );
}