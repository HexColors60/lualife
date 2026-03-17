mod layout;
mod log_panel;
mod map_view;
mod minimap;
mod perf_panel;
mod selection;
mod unit_panel;
mod widgets;

pub use layout::*;
pub use log_panel::*;
pub use map_view::*;
pub use minimap::*;
pub use perf_panel::*;
pub use selection::*;
pub use unit_panel::*;
pub use widgets::*;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .init_resource::<GameLog>()
            .add_plugins(SelectionPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_log_display,
                update_unit_panel,
                update_tick_display,
            ));
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
    mut query: Query<&mut Text, With<TickText>>,
) {
    if tick.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Tick: {}", tick.0);
        }
    }
}

fn update_log_display(
    game_log: Res<GameLog>,
    mut query: Query<&mut Text, With<LogText>>,
) {
    if game_log.is_changed() {
        for mut text in query.iter_mut() {
            // Show last 10 messages
            let display_text = game_log.messages
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