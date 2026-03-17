use bevy::prelude::*;

use crate::creeps::Creep;
use crate::debug::SelectionState;
use crate::factions::FactionRegistry;

/// Marker for the unit panel text
#[derive(Component)]
pub struct UnitPanelText;

/// System to update the unit panel with selected creep info
pub fn update_unit_panel(
    selection: Res<SelectionState>,
    creeps: Query<&Creep>,
    factions: Res<FactionRegistry>,
    mut text_query: Query<&mut Text, With<UnitPanelText>>,
) {
    if !selection.is_changed() {
        return;
    }

    for mut text in text_query.iter_mut() {
        if let Some(entity) = selection.selected_entity {
            if let Ok(creep) = creeps.get(entity) {
                let faction_name = factions
                    .get(creep.faction_id)
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                let role_name = format!("{:?}", creep.role);
                let action_name = creep.current_action
                    .as_ref()
                    .map(|a| format!("{:?}", a.action))
                    .unwrap_or_else(|| "Idle".to_string());

                text.sections[0].value = format!(
                    "=== Selected Creep ===\n\
                     ID: {}\n\
                     Faction: {}\n\
                     Role: {}\n\
                     HP: {:.0}/{:.0}\n\
                     Power: {:.0}/{:.0}\n\
                     Action: {}\n\
                     Pos: ({}, {})",
                    creep.id,
                    faction_name,
                    role_name,
                    creep.hp,
                    creep.max_hp,
                    creep.power_reserve,
                    creep.max_power,
                    action_name,
                    creep.position.x,
                    creep.position.y
                );
            } else {
                text.sections[0].value = "No creep selected".to_string();
            }
        } else {
            text.sections[0].value = "Click a creep to select".to_string();
        }
    }
}