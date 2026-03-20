mod ai_status;
mod creep_customization;
mod diplomacy_ui;
mod faction_leaderboard;
mod layout;
mod log_panel;
mod map_view;
mod market_ui;
mod minimap;
mod perf_panel;
mod screep_status;
mod selection;
mod tech_ui;
mod time_control;
mod unit_panel;
mod victory_screen;
mod widgets;

pub use ai_status::*;
pub use creep_customization::*;
pub use diplomacy_ui::*;
pub use faction_leaderboard::*;
pub use layout::*;
pub use log_panel::*;
pub use map_view::*;
pub use market_ui::*;
pub use minimap::*;
pub use perf_panel::*;
pub use screep_status::*;
pub use selection::*;
pub use tech_ui::*;
pub use time_control::*;
pub use unit_panel::*;
pub use victory_screen::*;
pub use widgets::*;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .init_resource::<GameLog>()
            .init_resource::<MinimapState>()
            .add_plugins(SelectionPlugin)
            .add_plugins(TimeControlPlugin)
            .add_plugins(DiplomacyUIPlugin)
            .add_plugins(FactionLeaderboardPlugin)
            .add_systems(
                Startup,
                (
                    setup_ui,
                    setup_minimap,
                    setup_resource_bar,
                    setup_scarcity_indicator,
                    setup_victory_screen,
                    setup_screep_status_panel,
                    setup_ai_status_panel,
                ),
            )
            .add_systems(
                Update,
                (
                    update_log_display,
                    update_unit_panel,
                    update_tick_display,
                    update_minimap_indicator,
                    toggle_minimap,
                ),
            )
            .add_systems(
                Update,
                (
                    update_resource_bar,
                    update_scarcity_indicator,
                    update_victory_screen,
                    handle_restart_button,
                    update_screep_status,
                    update_ai_status,
                ),
            );
    }
}



#[derive(Resource, Debug, Clone, Default)]
pub struct UiState {
    pub show_minimap: bool,
    pub show_log_panel: bool,
    pub show_perf_panel: bool,
    pub show_unit_panel: bool,
}

/// Game log for displaying messages
#[derive(Resource, Debug, Clone, Default)]
pub struct GameLog {
    pub messages: Vec<String>,
}

impl GameLog {
    pub fn add(&mut self, message: impl Into<String>) {
        let msg = message.into();
        tracing::info!("GameLog: {}", msg);
        self.messages.push(msg);
        // Keep only last 100 messages
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
}

/// Marker for the log text entity
#[derive(Component)]
pub struct LogText;

fn setup_ui(mut commands: Commands) {
    // Spawn UI camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });

    // Log panel at bottom-left
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Game started...\n",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(400.0),
                max_height: Val::Px(150.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        LogText,
    ));

    // Unit panel at top-right
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Click a creep to select",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(250.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        UnitPanelText,
    ));

    // Info panel at top-left
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "bevy_screeps_lua - 32 AI factions\nWASD/Arrows: Pan | Scroll: Zoom | Click: Select | ESC: Deselect",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
    ));

    // Tick counter panel at bottom-right
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Tick: 0",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(1.0, 1.0, 0.5),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        TickText,
    ));

    tracing::info!("UI initialized");
}

/// Marker for tick counter text
#[derive(Component)]
pub struct TickText;

/// System to update tick display
pub fn update_tick_display(
    tick: Res<crate::core::TickNumber>,
    time_state: Res<crate::ui::TimeControlState>,
    mut query: Query<&mut Text, With<TickText>>,
) {
    if tick.is_changed() || time_state.is_changed() {
        for mut text in query.iter_mut() {
            let speed_text = if time_state.is_paused {
                "PAUSED".to_string()
            } else {
                format!("{:.1}x", time_state.current_speed)
            };
            text.sections[0].value = format!("Tick: {} | Speed: {}", tick.0, speed_text);
        }
    }
}

fn update_log_display(game_log: Res<GameLog>, mut query: Query<&mut Text, With<LogText>>) {
    if game_log.is_changed() {
        for mut text in query.iter_mut() {
            // Show last 10 messages
            let display_text = game_log
                .messages
                .iter()
                .rev()
                .take(10)
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n");
            text.sections[0].value = display_text;
        }
    }
}
