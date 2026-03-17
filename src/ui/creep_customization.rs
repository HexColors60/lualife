use bevy::prelude::*;

use crate::creeps::{CreepBody, PartType};
use crate::ui::GameLog;

/// Creep body customization UI state
#[derive(Resource, Debug, Clone, Default)]
pub struct CreepCustomizationUI {
    pub visible: bool,
    pub selected_spawn: Option<u32>,
    pub current_body: CreepBody,
    pub available_parts: Vec<PartType>,
}

impl CreepCustomizationUI {
    pub fn new() -> Self {
        Self {
            visible: false,
            selected_spawn: None,
            current_body: CreepBody::default_harvester(),
            available_parts: vec![
                PartType::Move,
                PartType::Work,
                PartType::Fight,
                PartType::Mine,
                PartType::Build,
                PartType::Eat,
                PartType::Transport,
            ],
        }
    }

    pub fn add_part(&mut self, part: PartType) {
        self.current_body.add_part(part);
    }

    pub fn remove_part(&mut self, index: usize) {
        if index < self.current_body.parts.len() {
            self.current_body.parts.remove(index);
        }
    }

    pub fn clear_body(&mut self) {
        self.current_body.parts.clear();
    }

    pub fn set_preset(&mut self, preset: CreepPreset) {
        self.current_body = match preset {
            CreepPreset::Worker => CreepBody::default_harvester(),
            CreepPreset::Fighter => CreepBody::default_fighter(),
            CreepPreset::Transporter => CreepBody::default_harvester(),
            CreepPreset::Miner => CreepBody::default_harvester(),
        };
    }

    pub fn body_cost(&self) -> u32 {
        // Each part costs 50 power
        self.current_body.parts.len() as u32 * 50
    }
}

/// Preset creep configurations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreepPreset {
    Worker,
    Fighter,
    Transporter,
    Miner,
}

/// System to toggle creep customization UI
pub fn toggle_creep_customization_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui: ResMut<CreepCustomizationUI>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) && keyboard.pressed(KeyCode::ShiftLeft) {
        ui.visible = !ui.visible;
    }
}

/// System to display creep customization UI
pub fn creep_customization_ui_system(
    ui: Res<CreepCustomizationUI>,
    mut game_log: ResMut<GameLog>,
) {
    if !ui.visible {
        return;
    }

    if ui.is_changed() && ui.visible {
        game_log.add("=== Creep Customization ===".to_string());
        game_log.add(format!("Body parts: {}", ui.current_body.parts.len()));

        for (i, part) in ui.current_body.parts.iter().enumerate() {
            game_log.add(format!("  {}: {:?}", i, part));
        }

        game_log.add(format!("Total cost: {} Power", ui.body_cost()));
        game_log.add("Presets: 1=Worker, 2=Fighter, 3=Transporter, 4=Miner".to_string());
        game_log.add("Press Shift+C to close".to_string());
    }
}

/// System to handle preset selection
pub fn creep_preset_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui: ResMut<CreepCustomizationUI>,
) {
    if !ui.visible {
        return;
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        ui.set_preset(CreepPreset::Worker);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        ui.set_preset(CreepPreset::Fighter);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        ui.set_preset(CreepPreset::Transporter);
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        ui.set_preset(CreepPreset::Miner);
    }
}

/// Plugin for creep customization UI
pub struct CreepCustomizationPlugin;

impl Plugin for CreepCustomizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreepCustomizationUI>()
            .add_systems(Update, (
                toggle_creep_customization_ui,
                creep_customization_ui_system,
                creep_preset_input_system,
            ));
    }
}