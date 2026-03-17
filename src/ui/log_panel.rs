use bevy::prelude::*;

use crate::events::EventLog;

pub fn log_panel_system(
    event_log: Res<EventLog>,
) {
    // Display recent log entries
    if event_log.entries.len() > 0 {
        let recent: Vec<_> = event_log.entries.iter().rev().take(5).collect();
        for entry in recent {
            tracing::debug!("[{:?}] {}", entry.level, entry.message);
        }
    }
}