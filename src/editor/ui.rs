//! Map editor UI for tool selection and controls.
//!
//! Provides editor toolbar and property panels.

use bevy::prelude::*;

use super::{EditorMode, EditorState};

/// Marker for editor UI root
#[derive(Component)]
pub struct EditorUI;

/// Marker for editor toolbar
#[derive(Component)]
pub struct EditorToolbar;

/// Marker for editor status text
#[derive(Component)]
pub struct EditorStatusText;

/// Plugin for editor UI
pub struct EditorUIPlugin;

impl Plugin for EditorUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_editor_ui)
            .add_systems(Update, update_editor_ui);
    }
}

fn setup_editor_ui(mut commands: Commands) {
    // Editor toolbar (hidden by default)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                left: Val::Percent(50.0),
                margin: UiRect {
                    left: Val::Px(-200.0),
                    ..default()
                },
                padding: UiRect::all(Val::Px(8.0)),
                column_gap: Val::Px(4.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.15, 0.12, 0.1, 0.95)),
            border_color: BorderColor(Color::srgb(0.6, 0.5, 0.3)),
            ..default()
        },
        EditorToolbar,
    )).with_children(|parent| {
        // Mode buttons
        parent.spawn(TextBundle::from_section(
            "Mode:",
            TextStyle {
                font_size: 12.0,
                color: Color::srgb(0.8, 0.75, 0.6),
                ..default()
            },
        ));

        parent.spawn(TextBundle::from_section(
            "1-Terrain 2-Mines 3-Build 4-Faction 5-Res",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.7, 0.7, 0.7),
                ..default()
            },
        ));
    });

    // Editor status panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(10.0),
                width: Val::Px(200.0),
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.12, 0.15, 0.9)),
            border_color: BorderColor(Color::srgb(0.4, 0.5, 0.6)),
            ..default()
        },
        EditorUI,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Map Editor",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.85, 0.7),
                    ..default()
                },
            ),
        ));

        parent.spawn((
            TextBundle::from_section(
                "Mode: View\nTool: Select\nBrush: 1",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            EditorStatusText,
        ));

        parent.spawn(TextBundle::from_section(
            "F8: Toggle Editor\n0: View Mode\nQ/W/E/R: Tools\nB: Brush Size\nCtrl+Z: Undo\nCtrl+Y: Redo",
            TextStyle {
                font_size: 10.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ));
    });
}

fn update_editor_ui(
    state: Res<EditorState>,
    mut toolbar: Query<&mut Style, With<EditorToolbar>>,
    mut ui: Query<&mut Style, (With<EditorUI>, Without<EditorToolbar>)>,
    mut status: Query<&mut Text, With<EditorStatusText>>,
) {
    if !state.is_changed() {
        return;
    }

    // Show/hide UI based on editor enabled state
    for mut style in toolbar.iter_mut() {
        style.display = if state.enabled {
            Display::Flex
        } else {
            Display::None
        };
    }

    for mut style in ui.iter_mut() {
        style.display = if state.enabled {
            Display::Flex
        } else {
            Display::None
        };
    }

    // Update status text
    for mut text in status.iter_mut() {
        text.sections[0].value = format!(
            "Mode: {}\nTool: {}\nBrush: {}",
            state.mode.name(),
            state.tool.name(),
            state.brush_size.radius()
        );

        // Color based on mode
        text.sections[0].style.color = match state.mode {
            EditorMode::Terrain => Color::srgb(0.6, 0.8, 0.6),
            EditorMode::Mines => Color::srgb(0.8, 0.7, 0.5),
            EditorMode::Buildings => Color::srgb(0.6, 0.7, 0.8),
            EditorMode::Factions => Color::srgb(0.8, 0.6, 0.6),
            EditorMode::Resources => Color::srgb(0.7, 0.8, 0.7),
            EditorMode::None => Color::srgb(0.7, 0.7, 0.7),
        };
    }
}