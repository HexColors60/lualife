mod beginner_guide;
mod help_encyclopedia;
mod hint_system;
mod tooltip_system;
mod tutorial_system;

pub use beginner_guide::*;
pub use help_encyclopedia::*;
pub use hint_system::*;
pub use tooltip_system::*;
pub use tutorial_system::*;

use bevy::prelude::*;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TutorialState>()
            .init_resource::<TooltipState>()
            .init_resource::<HintState>()
            .init_resource::<BeginnerGuide>()
            .init_resource::<HelpEncyclopedia>()
            .add_event::<TutorialEvent>()
            .add_event::<HintEvent>()
            .add_systems(Startup, setup_tutorial_ui)
            .add_systems(Update, (
                tutorial_system,
                tooltip_system,
                hint_system,
                beginner_guide_system,
                handle_tutorial_input,
            ));
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct TutorialSettings {
    pub enabled: bool,
    pub show_hints: bool,
    pub show_tooltips: bool,
    pub auto_advance: bool,
}