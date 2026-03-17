use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug, Clone, Default)]
pub struct TooltipState {
    pub visible: bool,
    pub current_tooltip: Option<String>,
    pub tooltips: HashMap<String, TooltipInfo>,
    pub mouse_position: Vec2,
}

#[derive(Debug, Clone)]
pub struct TooltipInfo {
    pub title: String,
    pub description: String,
    pub shortcut: Option<String>,
}

impl TooltipState {
    pub fn new() -> Self {
        let mut tooltips = HashMap::new();

        tooltips.insert(
            "minimap".to_string(),
            TooltipInfo {
                title: "Minimap".to_string(),
                description: "Shows the entire world. Click to jump to a location.".to_string(),
                shortcut: Some("M".to_string()),
            },
        );

        tooltips.insert("unit_panel".to_string(), TooltipInfo {
            title: "Unit Panel".to_string(),
            description: "Shows information about the selected unit including HP, role, and current task.".to_string(),
            shortcut: None,
        });

        tooltips.insert(
            "ai_status".to_string(),
            TooltipInfo {
                title: "AI Status".to_string(),
                description:
                    "Shows the status of all 32 AI factions and their Lua script execution."
                        .to_string(),
                shortcut: None,
            },
        );

        tooltips.insert(
            "resource_bar".to_string(),
            TooltipInfo {
                title: "Resource Bar".to_string(),
                description: "Displays current resource stockpiles for the selected faction."
                    .to_string(),
                shortcut: None,
            },
        );

        tooltips.insert(
            "log_panel".to_string(),
            TooltipInfo {
                title: "Game Log".to_string(),
                description: "Shows game events, AI logs, and system messages.".to_string(),
                shortcut: None,
            },
        );

        tooltips.insert(
            "tech_ui".to_string(),
            TooltipInfo {
                title: "Technology Tree".to_string(),
                description: "Research new technologies to unlock abilities and buildings."
                    .to_string(),
                shortcut: Some("T".to_string()),
            },
        );

        tooltips.insert(
            "market_ui".to_string(),
            TooltipInfo {
                title: "Market".to_string(),
                description: "Trade resources with other factions.".to_string(),
                shortcut: Some("K".to_string()),
            },
        );

        tooltips.insert(
            "diplomacy_ui".to_string(),
            TooltipInfo {
                title: "Diplomacy".to_string(),
                description: "Manage alliances and relations with other factions.".to_string(),
                shortcut: Some("Shift+D".to_string()),
            },
        );

        Self {
            visible: false,
            current_tooltip: None,
            tooltips,
            mouse_position: Vec2::ZERO,
        }
    }

    pub fn get_tooltip(&self, id: &str) -> Option<&TooltipInfo> {
        self.tooltips.get(id)
    }

    pub fn show(&mut self, id: &str) {
        if self.tooltips.contains_key(id) {
            self.visible = true;
            self.current_tooltip = Some(id.to_string());
        }
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.current_tooltip = None;
    }
}

#[derive(Component)]
pub struct TooltipTarget {
    pub id: String,
}

#[derive(Component)]
pub struct TooltipDisplay;

pub fn tooltip_system(
    mut tooltip_state: ResMut<TooltipState>,
    mut query: Query<&mut Text, With<TooltipDisplay>>,
    targets: Query<(&TooltipTarget, &GlobalTransform)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        tooltip_state.mouse_position = cursor_pos;
    }

    if tooltip_state.visible {
        if let Some(id) = &tooltip_state.current_tooltip {
            if let Some(info) = tooltip_state.get_tooltip(id) {
                for mut text in query.iter_mut() {
                    let shortcut_text = info
                        .shortcut
                        .as_ref()
                        .map(|s| format!("\n[{}]", s))
                        .unwrap_or_default();
                    text.sections[0].value =
                        format!("{}{}\n{}", info.title, shortcut_text, info.description);
                }
            }
        }
    }
}

pub fn show_tooltip(id: &str, tooltip_state: &mut TooltipState) {
    tooltip_state.show(id);
}

pub fn hide_tooltip(tooltip_state: &mut TooltipState) {
    tooltip_state.hide();
}
