use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub enum UiEvent {
    EntitySelected(Entity),
    SelectionCleared,
    PanelToggled { panel: String },
    ZoomChanged(f32),
    CameraMoved { x: f32, y: f32 },
    MinimapClicked { room_x: u32, room_y: u32 },
}