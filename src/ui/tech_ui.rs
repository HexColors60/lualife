use bevy::prelude::*;

use crate::factions::FactionId;
use crate::research::{ResearchProgress, TechRegistry, TechTier};
use crate::ui::GameLog;

/// Tech tree UI state
#[derive(Resource, Debug, Clone, Default)]
pub struct TechTreeUI {
    pub visible: bool,
    pub selected_faction: Option<FactionId>,
    pub selected_tech: Option<String>,
}

/// System to toggle tech tree UI
pub fn toggle_tech_tree_ui(keyboard: Res<ButtonInput<KeyCode>>, mut tech_ui: ResMut<TechTreeUI>) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        tech_ui.visible = !tech_ui.visible;
    }
}

/// System to render tech tree UI
pub fn tech_tree_ui_system(
    tech_ui: Res<TechTreeUI>,
    tech_registry: Res<TechRegistry>,
    research_progress: Query<&ResearchProgress>,
    mut game_log: ResMut<GameLog>,
) {
    if !tech_ui.visible {
        return;
    }

    // Display tech tree info in game log
    if tech_ui.is_changed() && tech_ui.visible {
        game_log.add("=== Tech Tree ===".to_string());

        for tier in [
            TechTier::Tier1,
            TechTier::Tier2,
            TechTier::Tier3,
            TechTier::Tier4,
        ] {
            game_log.add(format!("--- {:?} ---", tier));

            for tech in tech_registry.get_techs_by_tier(tier) {
                let status = if tech.unlocked_by_default {
                    " [DEFAULT]"
                } else {
                    ""
                };
                game_log.add(format!("  {}{}", tech.name, status));
            }
        }

        game_log.add("Press T to close".to_string());
    }
}

/// Plugin for tech tree UI
pub struct TechTreeUIPlugin;

impl Plugin for TechTreeUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TechTreeUI>()
            .add_systems(Update, (toggle_tech_tree_ui, tech_tree_ui_system));
    }
}
