//! Victory screen overlay for displaying game end results.

use bevy::prelude::*;

use crate::victory::{VictoryCondition, VictoryState};

/// Marker for victory screen container
#[derive(Component)]
pub struct VictoryScreen;

/// Marker for restart button
#[derive(Component)]
pub struct RestartButton;

/// Setup victory screen (spawned but hidden initially)
pub fn setup_victory_screen(mut commands: Commands) {
    // Victory screen container - centered overlay
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None, // Hidden initially
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ..default()
        },
        VictoryScreen,
    )).with_children(|parent| {
        // Victory panel
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                border_color: BorderColor(Color::srgb(0.4, 0.6, 0.4)),
                ..default()
            },
        )).with_children(|panel| {
            // Title
            panel.spawn((
                TextBundle::from_section(
                    "VICTORY!",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(0.4, 1.0, 0.4),
                        ..default()
                    },
                ),
            ));

            // Winner text
            panel.spawn((
                TextBundle::from_section(
                    "Faction X has won!",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            )).insert(WinnerText);

            // Victory condition text
            panel.spawn((
                TextBundle::from_section(
                    "Domination Victory - 50% territory controlled",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                ),
            )).insert(ConditionText);

            // Stats section
            panel.spawn((
                TextBundle::from_section(
                    "--- Game Statistics ---",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                        ..default()
                    },
                ),
            )).insert(StatsText);

            // Restart button
            panel.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.4, 0.2)),
                    ..default()
                },
                RestartButton,
            )).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "New Game",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
        });
    });
}

/// Marker for winner text component
#[derive(Component)]
pub struct WinnerText;

/// Marker for condition text component
#[derive(Component)]
pub struct ConditionText;

/// Marker for stats text component
#[derive(Component)]
pub struct StatsText;

/// Update victory screen visibility and content
pub fn update_victory_screen(
    victory_state: Res<VictoryState>,
    mut screen_query: Query<&mut Style, With<VictoryScreen>>,
    mut winner_query: Query<&mut Text, (With<WinnerText>, Without<ConditionText>, Without<StatsText>)>,
    mut condition_query: Query<&mut Text, (With<ConditionText>, Without<WinnerText>, Without<StatsText>)>,
    mut stats_query: Query<&mut Text, (With<StatsText>, Without<WinnerText>, Without<ConditionText>)>,
    eliminated: Query<&crate::creeps::Creep>,
) {
    if !victory_state.is_changed() {
        return;
    }

    // Show/hide screen based on game_over state
    for mut style in screen_query.iter_mut() {
        style.display = if victory_state.game_over {
            Display::Flex
        } else {
            Display::None
        };
    }

    if !victory_state.game_over {
        return;
    }

    // Update winner text
    if let Some(winner) = victory_state.winner {
        if let Ok(mut text) = winner_query.get_single_mut() {
            text.sections[0].value = format!("Faction {} has won!", winner.0);
        }
    }

    // Update condition text
    let condition_str = match victory_state.condition {
        VictoryCondition::Domination => {
            let threshold = (victory_state.threshold * 100.0) as u32;
            format!("Domination Victory - {}% territory controlled", threshold)
        }
        VictoryCondition::Elimination => "Last Faction Standing".to_string(),
        VictoryCondition::Economic => "Economic Victory".to_string(),
        VictoryCondition::Technological => "Technological Victory".to_string(),
        VictoryCondition::Alliance => "Diplomatic Victory".to_string(),
    };

    if let Ok(mut text) = condition_query.get_single_mut() {
        text.sections[0].value = condition_str;
    }

    // Update stats
    let eliminated_count = victory_state.eliminated_factions.len();
    let remaining_count = victory_state.active_factions.len();

    let stats_str = format!(
        "Factions Eliminated: {}\nFactions Remaining: {}\nTotal Creeps: {}",
        eliminated_count,
        remaining_count,
        eliminated.iter().count()
    );

    if let Ok(mut text) = stats_query.get_single_mut() {
        text.sections[0].value = stats_str;
    }
}

/// Handle restart button click
pub fn handle_restart_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut victory_state: ResMut<VictoryState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Reset victory state
            victory_state.winner = None;
            victory_state.game_over = false;
            victory_state.eliminated_factions.clear();
            victory_state.active_factions.clear();
            victory_state.progress.clear();

            game_log.add("New game started!");
        }
    }
}