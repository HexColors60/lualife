mod beginner_guide;
mod help_encyclopedia;
mod hint_system;
mod tooltip_system;
mod tutorial_system;

pub use beginner_guide::{BeginnerGuide, BeginnerGuideDisplay, Objective};
pub use help_encyclopedia::{HelpCategory, HelpDisplay, HelpEncyclopedia, HelpEntry};
pub use hint_system::{Hint, HintDisplay, HintEvent, HintState, HintTrigger};
pub use tooltip_system::{TooltipDisplay, TooltipInfo, TooltipState, TooltipTarget};
pub use tutorial_system::{
    TutorialAction, TutorialEvent, TutorialState, TutorialStep, TutorialText,
};

use beginner_guide::beginner_guide_system;
use bevy::prelude::*;
use help_encyclopedia::help_encyclopedia_system;
use hint_system::hint_system;
use tooltip_system::tooltip_system;
use tutorial_system::{handle_tutorial_input, setup_tutorial_ui, tutorial_system};

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
            .add_systems(
                Update,
                (
                    tutorial_system,
                    tooltip_system,
                    hint_system,
                    beginner_guide_system,
                    help_encyclopedia_system,
                    handle_tutorial_input,
                ),
            );
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct TutorialSettings {
    pub enabled: bool,
    pub show_hints: bool,
    pub show_tooltips: bool,
    pub auto_advance: bool,
}
