//! Quest UI for displaying active and completed quests.
//!
//! Shows quest progress, objectives, and rewards.

use bevy::prelude::*;

use super::{Quest, QuestRegistry, QuestObjectiveType};

/// Marker for quest panel
#[derive(Component)]
pub struct QuestPanel;

/// Marker for quest list item
#[derive(Component)]
pub struct QuestItem {
    pub quest_id: u32,
}

/// Plugin for quest UI
pub struct QuestUIPlugin;

impl Plugin for QuestUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_quest_panel)
            .add_systems(Update, (update_quest_panel, toggle_quest_panel));
    }
}

fn setup_quest_panel(mut commands: Commands) {
    // Quest panel (hidden by default, toggle with 'P')
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(10.0),
                width: Val::Px(280.0),
                max_height: Val::Px(400.0),
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                flex_direction: FlexDirection::Column,
                display: Display::None, // Hidden by default
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.12, 0.18, 0.95)),
            border_color: BorderColor(Color::srgb(0.3, 0.5, 0.7)),
            ..default()
        },
        QuestPanel,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "📜 Quest Log",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.9, 0.85, 0.6),
                ..default()
            },
        ));

        // Quest list (will be populated dynamically)
        parent.spawn((
            TextBundle::from_section(
                "No active quests",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ),
            QuestListText,
        ));
    });
}

/// Marker for quest list text
#[derive(Component)]
pub struct QuestListText;

/// Resource to track quest panel visibility
#[derive(Resource, Default)]
pub struct QuestPanelState {
    pub visible: bool,
}

fn update_quest_panel(
    registry: Res<QuestRegistry>,
    state: Res<QuestPanelState>,
    mut panels: Query<(&mut Style, &mut Visibility), With<QuestPanel>>,
    mut text: Query<&mut Text, With<QuestListText>>,
) {
    if !registry.is_changed() && !state.is_changed() {
        return;
    }

    for (mut style, mut visibility) in panels.iter_mut() {
        if state.visible {
            style.display = Display::Flex;
            *visibility = Visibility::Visible;
        } else {
            style.display = Display::None;
            *visibility = Visibility::Hidden;
        }
    }

    // Update quest list text
    for mut text in text.iter_mut() {
        let active_quests = registry.get_active_quests();

        if active_quests.is_empty() {
            text.sections[0].value = "No active quests\nPress 'P' to hide".to_string();
            text.sections[0].style.color = Color::srgb(0.5, 0.5, 0.5);
        } else {
            let mut quest_text = String::new();

            for quest in active_quests.iter().take(5) {
                let progress = quest.progress_percent();
                let progress_bar = create_progress_bar(progress);

                quest_text.push_str(&format!(
                    "{}\n{}\n{} [{:.0}%]\n\n",
                    quest.name,
                    get_objective_text(quest.objectives.first()),
                    progress_bar,
                    progress
                ));
            }

            if active_quests.len() > 5 {
                quest_text.push_str(&format!("... and {} more\n", active_quests.len() - 5));
            }

            quest_text.push_str("\nPress 'P' to hide");

            text.sections[0].value = quest_text;
            text.sections[0].style.color = Color::srgb(0.85, 0.85, 0.9);
        }
    }
}

fn toggle_quest_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<QuestPanelState>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        state.visible = !state.visible;
    }
}

/// Create a text-based progress bar
fn create_progress_bar(percent: f32) -> String {
    let filled = (percent / 10.0) as usize;
    let empty = 10 - filled;

    format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}

/// Get objective description text
fn get_objective_text(objective: Option<&super::QuestObjective>) -> String {
    match objective {
        Some(obj) => match &obj.objective_type {
            QuestObjectiveType::KillUnits { enemy_faction } => {
                if let Some(faction) = enemy_faction {
                    format!("Kill {} units (Faction {})", obj.target_amount, faction.0)
                } else {
                    format!("Kill {} enemy units", obj.target_amount)
                }
            }
            QuestObjectiveType::GatherResource { resource_type } => {
                format!("Gather {:?} x{}", resource_type, obj.target_amount)
            }
            QuestObjectiveType::BuildStructure { building_type } => {
                format!("Build {} {}", obj.target_amount, building_type)
            }
            QuestObjectiveType::ExploreRooms { count } => {
                format!("Explore {} rooms", count)
            }
            QuestObjectiveType::DefendPosition { x, y, duration_ticks } => {
                format!("Defend ({}, {}) for {} ticks", x, y, duration_ticks)
            }
            QuestObjectiveType::Survive { ticks } => {
                format!("Survive for {} ticks", ticks)
            }
            QuestObjectiveType::ControlRooms { count } => {
                format!("Control {} rooms", count)
            }
            QuestObjectiveType::ReachPower { amount } => {
                format!("Reach {} power", amount)
            }
            QuestObjectiveType::Custom { description } => {
                description.clone()
            }
        },
        None => "No objective".to_string(),
    }
}