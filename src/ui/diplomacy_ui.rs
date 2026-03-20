use bevy::prelude::*;

use crate::diplomacy::DiplomacyState;
use crate::factions::FactionId;

/// Diplomacy UI state
#[derive(Resource, Debug, Clone, Default)]
pub struct DiplomacyUI {
    pub visible: bool,
    pub selected_faction: Option<FactionId>,
}

/// Marker for diplomacy panel
#[derive(Component)]
pub struct DiplomacyPanel;

/// Marker for diplomacy content text
#[derive(Component)]
pub struct DiplomacyContent;

/// System to toggle diplomacy UI
pub fn toggle_diplomacy_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut diplomacy_ui: ResMut<DiplomacyUI>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) && keyboard.pressed(KeyCode::ShiftLeft) {
        diplomacy_ui.visible = !diplomacy_ui.visible;
    }
}

/// Setup diplomacy UI panel
pub fn setup_diplomacy_panel(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                right: Val::Px(10.0),
                width: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            border_color: BorderColor(Color::srgb(0.3, 0.5, 0.7)),
            ..default()
        },
        DiplomacyPanel,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "Diplomacy Status",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.5, 0.8, 1.0),
                ..default()
            },
        ));

        // Content
        parent.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            DiplomacyContent,
        ));
    });
}

/// System to update diplomacy UI panel
pub fn update_diplomacy_panel(
    diplomacy_ui: Res<DiplomacyUI>,
    diplomacy_state: Res<DiplomacyState>,
    mut panel_query: Query<&mut Style, With<DiplomacyPanel>>,
    mut content_query: Query<&mut Text, With<DiplomacyContent>>,
) {
    // Update visibility
    for mut style in panel_query.iter_mut() {
        style.display = if diplomacy_ui.visible {
            Display::Flex
        } else {
            Display::None
        };
    }

    if !diplomacy_ui.visible {
        return;
    }

    // Update content
    let mut lines: Vec<String> = Vec::new();

    // Alliance count
    lines.push(format!("Alliances: {}", diplomacy_state.alliance_count()));
    lines.push("".to_string());

    // Show wars
    let mut war_count = 0;
    for faction_id in 0..32u16 {
        let faction = FactionId(faction_id);
        let enemies = diplomacy_state.get_enemies(faction);
        war_count += enemies.len();
    }
    war_count /= 2; // Each war counted twice
    lines.push(format!("Active Wars: {}", war_count));
    lines.push("".to_string());

    // Selected faction relations
    if let Some(faction) = diplomacy_ui.selected_faction {
        lines.push(format!("--- Faction {} ---", faction.0));

        let allies = diplomacy_state.get_allies(faction);
        let enemies = diplomacy_state.get_enemies(faction);

        if allies.is_empty() {
            lines.push("Allies: None".to_string());
        } else {
            let ally_names: Vec<String> = allies.iter().map(|f| format!("F{}", f.0)).collect();
            lines.push(format!("Allies: {}", ally_names.join(", ")));
        }

        if enemies.is_empty() {
            lines.push("Enemies: None".to_string());
        } else {
            let enemy_names: Vec<String> = enemies.iter().map(|f| format!("F{}", f.0)).collect();
            lines.push(format!("Enemies: {}", enemy_names.join(", ")));
        }
    } else {
        lines.push("Click a faction to view relations".to_string());
    }

    lines.push("".to_string());
    lines.push("Shift+D to close".to_string());

    if let Ok(mut text) = content_query.get_single_mut() {
        text.sections[0].value = lines.join("\n");
    }
}

/// Plugin for diplomacy UI
pub struct DiplomacyUIPlugin;

impl Plugin for DiplomacyUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiplomacyUI>()
            .add_systems(Startup, setup_diplomacy_panel)
            .add_systems(Update, (toggle_diplomacy_ui, update_diplomacy_panel));
    }
}