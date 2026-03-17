use bevy::prelude::*;

use crate::creeps::Creep;
use crate::factions::FactionRegistry;
use crate::debug::SelectionState;

/// Marker for screep status panel
#[derive(Component)]
pub struct ScreepStatusPanel;

/// Marker for screep list text
#[derive(Component)]
pub struct ScreepListText;

/// Setup screep status panel
pub fn setup_screep_status_panel(mut commands: Commands) {
    // Screep status panel (left side, below log)
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "=== All Creeps ===\nLoading...",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(170.0),
                left: Val::Px(10.0),
                width: Val::Px(200.0),
                max_height: Val::Px(200.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            ..default()
        },
        ScreepListText,
    ));
}

/// Update screep status panel
pub fn update_screep_status(
    mut query: Query<&mut Text, With<ScreepListText>>,
    creeps: Query<&Creep>,
    factions: Res<FactionRegistry>,
    selection: Res<SelectionState>,
) {
    // Only update every 60 frames or when selection changes
    if !selection.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        let mut lines = vec!["=== All Creeps ===".to_string()];

        // Group creeps by faction
        let mut faction_counts: std::collections::HashMap<crate::factions::FactionId, usize> =
            std::collections::HashMap::new();

        for creep in creeps.iter() {
            *faction_counts.entry(creep.faction_id).or_insert(0) += 1;
        }

        // Show top 10 factions by creep count
        let mut sorted: Vec<_> = faction_counts.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (faction_id, count) in sorted.iter().take(10) {
            let faction_name = factions
                .get(**faction_id)
                .map(|f| f.name.clone())
                .unwrap_or_else(|| format!("Faction {}", faction_id.0));
            lines.push(format!("{}: {} creeps", faction_name, count));
        }

        let total = creeps.iter().count();
        lines.push(format!("---\nTotal: {} creeps", total));

        text.sections[0].value = lines.join("\n");
    }
}