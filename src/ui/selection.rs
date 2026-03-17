use bevy::prelude::*;

use crate::debug::SelectionState;
use crate::events::UiEvent;

pub fn selection_system(
    mut selection: ResMut<SelectionState>,
    mut ui_events: EventReader<UiEvent>,
) {
    for event in ui_events.read() {
        match event {
            UiEvent::EntitySelected(entity) => {
                selection.select(*entity);
            }
            UiEvent::SelectionCleared => {
                selection.deselect();
            }
            _ => {}
        }
    }
}