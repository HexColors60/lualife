//! Time control UI for simulation speed management.

use bevy::prelude::*;

use crate::core::{GameState, SimClock};

/// Marker for the time control panel container.
#[derive(Component)]
pub struct TimeControlPanel;

/// Marker for pause button.
#[derive(Component)]
pub struct PauseButton;

/// Marker for speed buttons.
#[derive(Component)]
pub struct SpeedButton {
    pub speed: f32,
}

/// Resource tracking current speed selection for UI highlighting.
#[derive(Resource, Debug, Clone)]
pub struct TimeControlState {
    pub current_speed: f32,
    pub is_paused: bool,
}

impl Default for TimeControlState {
    fn default() -> Self {
        Self {
            current_speed: 1.0,
            is_paused: false,
        }
    }
}

/// Plugin for time control UI.
pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeControlState>()
            .add_systems(Startup, setup_time_control_panel)
            .add_systems(
                Update,
                (
                    handle_speed_buttons,
                    handle_pause_button,
                    handle_time_control_keyboard,
                    update_time_control_display,
                ),
            );
    }
}

fn setup_time_control_panel(mut commands: Commands) {
    // Time control panel at top-center
    let button_style = Style {
        width: Val::Px(50.0),
        height: Val::Px(28.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::horizontal(Val::Px(2.0)),
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 12.0,
        color: Color::srgb(1.0, 1.0, 1.0),
        ..default()
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(50.0),
                // Center the panel
                margin: UiRect {
                    left: Val::Px(-160.0), // Half of panel width
                    ..default()
                },
                padding: UiRect::all(Val::Px(5.0)),
                column_gap: Val::Px(2.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            border_color: BorderColor(Color::srgb(0.3, 0.3, 0.4)),
            ..default()
        },
        TimeControlPanel,
    )).with_children(|parent| {
        // Pause button
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                ..default()
            },
            PauseButton,
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "⏸",
                button_text_style.clone(),
            ));
        });

        // Slow button (0.5x)
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                ..default()
            },
            SpeedButton { speed: 0.5 },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "0.5x",
                button_text_style.clone(),
            ));
        });

        // Normal button (1x)
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.2, 0.5, 0.2)),
                ..default()
            },
            SpeedButton { speed: 1.0 },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "1x",
                button_text_style.clone(),
            ));
        });

        // Fast button (2x)
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                ..default()
            },
            SpeedButton { speed: 2.0 },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "2x",
                button_text_style.clone(),
            ));
        });

        // Faster button (4x)
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                ..default()
            },
            SpeedButton { speed: 4.0 },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "4x",
                button_text_style.clone(),
            ));
        });

        // Max button (8x)
        parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                ..default()
            },
            SpeedButton { speed: 8.0 },
        )).with_children(|button| {
            button.spawn(TextBundle::from_section(
                "8x",
                button_text_style.clone(),
            ));
        });
    });
}

fn handle_speed_buttons(
    mut interaction_query: Query<(&Interaction, &SpeedButton, &mut BackgroundColor), Changed<Interaction>>,
    mut sim_clock: ResMut<SimClock>,
    mut time_state: ResMut<TimeControlState>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, speed_button, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                // Set the speed
                sim_clock.set_speed(speed_button.speed);
                time_state.current_speed = speed_button.speed;
                
                // Unpause if paused
                if *game_state == GameState::Paused {
                    *game_state = GameState::Running;
                    time_state.is_paused = false;
                }
                
                // Update button colors
                *bg_color = if speed_button.speed == time_state.current_speed && !time_state.is_paused {
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2))
                } else {
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.4))
                };
            }
            Interaction::Hovered => {
                if speed_button.speed != time_state.current_speed || time_state.is_paused {
                    *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.5));
                }
            }
            Interaction::None => {
                *bg_color = if speed_button.speed == time_state.current_speed && !time_state.is_paused {
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2))
                } else {
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.4))
                };
            }
        }
    }
}

fn handle_pause_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PauseButton>)>,
    mut game_state: ResMut<GameState>,
    mut time_state: ResMut<TimeControlState>,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                // Toggle pause
                match *game_state {
                    GameState::Running => {
                        *game_state = GameState::Paused;
                        time_state.is_paused = true;
                    }
                    GameState::Paused => {
                        *game_state = GameState::Running;
                        time_state.is_paused = false;
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.6, 0.3, 0.3));
            }
            Interaction::None => {
                *bg_color = if time_state.is_paused {
                    BackgroundColor(Color::srgb(0.7, 0.3, 0.3))
                } else {
                    BackgroundColor(Color::srgb(0.4, 0.2, 0.2))
                };
            }
        }
    }
}

fn update_time_control_display(
    time_state: Res<TimeControlState>,
    mut speed_buttons: Query<(&SpeedButton, &mut BackgroundColor), Without<PauseButton>>,
    mut pause_button: Query<&mut BackgroundColor, With<PauseButton>>,
) {
    if time_state.is_changed() {
        // Update speed button colors
        for (speed_button, mut bg_color) in speed_buttons.iter_mut() {
            *bg_color = if speed_button.speed == time_state.current_speed && !time_state.is_paused {
                BackgroundColor(Color::srgb(0.2, 0.6, 0.2))
            } else {
                BackgroundColor(Color::srgb(0.3, 0.3, 0.4))
            };
        }

        // Update pause button color
        for mut bg_color in pause_button.iter_mut() {
            *bg_color = if time_state.is_paused {
                BackgroundColor(Color::srgb(0.7, 0.3, 0.3))
            } else {
                BackgroundColor(Color::srgb(0.4, 0.2, 0.2))
            };
        }
    }
}

/// Handle keyboard shortcuts for time control
fn handle_time_control_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut sim_clock: ResMut<SimClock>,
    mut time_state: ResMut<TimeControlState>,
) {
    // Space toggles pause
    if keyboard.just_pressed(KeyCode::Space) {
        match *game_state {
            GameState::Running => {
                *game_state = GameState::Paused;
                time_state.is_paused = true;
            }
            GameState::Paused => {
                *game_state = GameState::Running;
                time_state.is_paused = false;
            }
            _ => {}
        }
    }

    // Speed controls: - to decrease, = to increase
    let speeds = [0.5, 1.0, 2.0, 4.0, 8.0];
    
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        // Increase speed
        let current_idx = speeds.iter().position(|&s| s == time_state.current_speed).unwrap_or(1);
        if current_idx < speeds.len() - 1 {
            let new_speed = speeds[current_idx + 1];
            sim_clock.set_speed(new_speed);
            time_state.current_speed = new_speed;
            if *game_state == GameState::Paused {
                *game_state = GameState::Running;
                time_state.is_paused = false;
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        // Decrease speed
        let current_idx = speeds.iter().position(|&s| s == time_state.current_speed).unwrap_or(1);
        if current_idx > 0 {
            let new_speed = speeds[current_idx - 1];
            sim_clock.set_speed(new_speed);
            time_state.current_speed = new_speed;
            if *game_state == GameState::Paused {
                *game_state = GameState::Running;
                time_state.is_paused = false;
            }
        }
    }
}