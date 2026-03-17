use bevy::prelude::*;

use crate::diplomacy::DiplomacyState;
use crate::factions::FactionId;
use crate::ui::GameLog;

/// Diplomacy UI state
#[derive(Resource, Debug, Clone, Default)]
pub struct DiplomacyUI {
    pub visible: bool,
    pub selected_faction: Option<FactionId>,
}

/// System to toggle diplomacy UI
pub fn toggle_diplomacy_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut diplomacy_ui: ResMut<DiplomacyUI>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) && keyboard.pressed(KeyCode::ShiftLeft) {
        diplomacy_ui.visible = !diplomacy_ui.visible;
    }
}

/// System to render diplomacy UI
pub fn diplomacy_ui_system(
    diplomacy_ui: Res<DiplomacyUI>,
    diplomacy_state: Res<DiplomacyState>,
    mut game_log: ResMut<GameLog>,
) {
    if !diplomacy_ui.visible {
        return;
    }

    // Display diplomacy info in game log
    if diplomacy_ui.is_changed() && diplomacy_ui.visible {
        game_log.add("=== Diplomacy ===".to_string());

        // Show alliances
        game_log.add(format!(
            "Active alliances: {}",
            diplomacy_state.alliance_count()
        ));

        for (id, alliance) in diplomacy_state.get_alliances() {
            let members: Vec<String> = alliance
                .members
                .iter()
                .map(|f| format!("F{}", f.0))
                .collect();
            game_log.add(format!(
                "  Alliance {}: {} members: {}",
                id,
                alliance.members.len(),
                members.join(", ")
            ));
        }

        // Show selected faction relations
        if let Some(faction) = diplomacy_ui.selected_faction {
            let allies = diplomacy_state.get_allies(faction);
            let enemies = diplomacy_state.get_enemies(faction);

            game_log.add(format!("Faction {} relations:", faction.0));
            game_log.add(format!("  Allies: {} factions", allies.len()));
            game_log.add(format!("  Enemies: {} factions", enemies.len()));
        }

        game_log.add("Press Shift+D to close".to_string());
    }
}

/// Plugin for diplomacy UI
pub struct DiplomacyUIPlugin;

impl Plugin for DiplomacyUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiplomacyUI>()
            .add_systems(Update, (toggle_diplomacy_ui, diplomacy_ui_system));
    }
}
