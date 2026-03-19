mod auto_save;
mod undo_redo;
mod templates;
mod unit_groups;
mod minimap_ping;

pub use auto_save::*;
pub use undo_redo::*;
pub use templates::*;
pub use unit_groups::*;
pub use minimap_ping::*;

use bevy::prelude::*;

/// Plugin for quality of life features
pub struct QualityOfLifePlugin;

impl Plugin for QualityOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoSaveSettings>()
            .init_resource::<AutoSaveSlots>()
            .init_resource::<UndoRedoState>()
            .init_resource::<UndoRedoState>()
            .init_resource::<TemplateManager>()
            .init_resource::<UnitGroupManager>()
            .init_resource::<MinimapPingState>()
            .add_event::<AutoSaveEvent>()
            .add_event::<UndoRedoEvent>()
            .add_event::<TemplateEvent>()
            .add_event::<UnitGroupEvent>()
            .add_event::<MinimapPingEvent>()
            .add_systems(Update, (
                auto_save_system,
                undo_redo_system,
                template_system,
                unit_group_system,
                minimap_ping_system,
            ));
    }
}