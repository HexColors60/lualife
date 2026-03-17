use bevy::prelude::*;

use crate::factions::FactionRegistry;
use crate::lua::LuaState;

/// Marker for AI status panel
#[derive(Component)]
pub struct AiStatusPanel;

/// Marker for AI status text
#[derive(Component)]
pub struct AiStatusText;

/// Setup AI status panel
pub fn setup_ai_status_panel(mut commands: Commands) {
    // AI status panel (right side, below unit panel)
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "=== AI Status ===\nLoading...",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(250.0),
                right: Val::Px(10.0),
                width: Val::Px(200.0),
                max_height: Val::Px(200.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            ..default()
        },
        AiStatusText,
    ));
}

/// Update AI status panel
pub fn update_ai_status(
    mut query: Query<&mut Text, With<AiStatusText>>,
    factions: Res<FactionRegistry>,
    lua_state: Res<LuaState>,
) {
    if !lua_state.is_changed() && !factions.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        let mut lines = vec!["=== AI Status ===".to_string()];

        lines.push(format!("Factions: {}", factions.count()));
        lines.push(format!(
            "Lua VMs: {}",
            if lua_state.initialized {
                "Initialized"
            } else {
                "Pending"
            }
        ));

        // Show script status
        if lua_state.initialized {
            lines.push("Scripts: Loaded".to_string());
        } else {
            lines.push("Scripts: Loading...".to_string());
        }

        lines.push("---".to_string());
        lines.push("Press F3 for debug".to_string());
        lines.push("Press M for minimap".to_string());

        text.sections[0].value = lines.join("\n");
    }
}
