use bevy::prelude::*;

/// Auto-save settings resource
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct AutoSaveSettings {
    pub enabled: bool,
    pub interval_seconds: f32,
    pub max_saves: usize,
    pub save_on_exit: bool,
    pub last_save_time: f32,
}

impl Default for AutoSaveSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 300.0, // 5 minutes
            max_saves: 10,
            save_on_exit: true,
            last_save_time: 0.0,
        }
    }
}

/// Auto-save event
#[derive(Debug, Clone, Event)]
pub enum AutoSaveEvent {
    TriggerSave,
    SaveCompleted { slot: u32 },
    SaveFailed { error: String },
    SettingsChanged(AutoSaveSettings),
}

/// Auto-save slot information
#[derive(Debug, Clone, Resource, Default)]
pub struct AutoSaveSlots {
    pub slots: Vec<AutoSaveSlot>,
}

#[derive(Debug, Clone)]
pub struct AutoSaveSlot {
    pub index: u32,
    pub timestamp: f32,
    pub tick: u64,
    pub filename: String,
}

/// System to handle auto-save
pub fn auto_save_system(
    time: Res<Time>,
    settings: Res<AutoSaveSettings>,
    mut events: EventWriter<AutoSaveEvent>,
    mut slots: ResMut<AutoSaveSlots>,
    tick: Option<Res<crate::core::TickNumber>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !settings.enabled {
        return;
    }

    // Check for manual auto-save trigger (F5)
    if keyboard.just_pressed(KeyCode::F5) {
        events.send(AutoSaveEvent::TriggerSave);
    }

    // Check for time-based auto-save
    let current_time = time.elapsed_seconds();
    if current_time - settings.last_save_time >= settings.interval_seconds {
        events.send(AutoSaveEvent::TriggerSave);
    }
}

/// Save game to auto-save slot
pub fn save_to_autosave_slot(
    slot_index: u32,
    tick: u64,
) -> Result<String, String> {
    let filename = format!("autosave_{}.sav", slot_index);
    // In a real implementation, this would call the save system
    tracing::info!("Auto-saved to {}", filename);
    Ok(filename)
}

/// Clean up old auto-saves
pub fn cleanup_old_autosaves(
    slots: &mut AutoSaveSlots,
    max_saves: usize,
) {
    while slots.slots.len() > max_saves {
        let oldest = slots.slots.remove(0);
        // Delete the file
        if let Err(e) = std::fs::remove_file(&oldest.filename) {
            tracing::warn!("Failed to delete old auto-save: {}", e);
        }
    }
}

/// Get auto-save interval options
pub fn get_interval_options() -> Vec<(String, f32)> {
    vec![
        ("1 minute".to_string(), 60.0),
        ("5 minutes".to_string(), 300.0),
        ("10 minutes".to_string(), 600.0),
        ("15 minutes".to_string(), 900.0),
        ("30 minutes".to_string(), 1800.0),
        ("Disabled".to_string(), f32::MAX),
    ]
}