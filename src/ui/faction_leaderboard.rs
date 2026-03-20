//! Faction leaderboard panel - shows all factions with their stats.

use bevy::prelude::*;

use crate::factions::{FactionId, FactionIdentities, FactionRegistry};
use crate::territory::TerritoryManager;
use crate::creeps::Creep;

/// Marker for faction leaderboard panel
#[derive(Component)]
pub struct FactionLeaderboardPanel;

/// Marker for faction leaderboard text
#[derive(Component)]
pub struct FactionLeaderboardText;

/// Panel visibility state
#[derive(Resource, Debug, Clone, Default)]
pub struct FactionLeaderboardState {
    pub visible: bool,
    pub scroll_offset: usize,
}

/// Setup faction leaderboard panel
pub fn setup_faction_leaderboard(
    mut commands: Commands,
    state: Res<FactionLeaderboardState>,
) {
    if !state.visible {
        return;
    }

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "=== Factions ===\nLoading...",
                TextStyle {
                    font_size: 10.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(280.0),
                max_height: Val::Px(400.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        FactionLeaderboardText,
    ));
}

/// Update faction leaderboard
pub fn update_faction_leaderboard(
    mut query: Query<&mut Text, With<FactionLeaderboardText>>,
    identities: Res<FactionIdentities>,
    registry: Res<FactionRegistry>,
    territory: Res<TerritoryManager>,
    creeps: Query<&Creep>,
    state: Res<FactionLeaderboardState>,
) {
    if !state.visible {
        return;
    }

    // Count creeps per faction
    let mut creep_counts: std::collections::HashMap<FactionId, usize> = std::collections::HashMap::new();
    for creep in creeps.iter() {
        *creep_counts.entry(creep.faction_id).or_insert(0) += 1;
    }

    // Collect faction data
    let mut faction_data: Vec<(FactionId, String, String, usize, usize, String)> = Vec::new();
    
    for faction in registry.all() {
        let identity = identities.get(faction.id);
        let name = identity.map(|i| i.name.as_str()).unwrap_or("Unknown");
        let leader = identity.map(|i| i.leader_name.as_str()).unwrap_or("Unknown");
        let personality = identity.map(|i| format!("{:?}", i.personality)).unwrap_or_default();
        let territory_count = territory.get_territory_count(faction.id);
        let creep_count = creep_counts.get(&faction.id).copied().unwrap_or(0);
        
        faction_data.push((faction.id, name.to_string(), leader.to_string(), territory_count, creep_count, personality));
    }

    // Sort by territory count (descending)
    faction_data.sort_by(|a, b| b.3.cmp(&a.3));

    // Build display
    let mut lines = vec![
        "╔══════════════════════════════════╗".to_string(),
        "║      FACTION LEADERBOARD         ║".to_string(),
        "╠══════════════════════════════════╣".to_string(),
    ];

    // Show top 12 factions
    let display_count = 12.min(faction_data.len());
    for (i, (id, name, leader, territories, creeps, personality)) in faction_data.iter().take(display_count).enumerate() {
        let rank = i + 1;
        let short_name = if name.len() > 16 { &name[..16] } else { name };
        
        // Rank indicator (top 3 get special markers)
        let marker = match rank {
            1 => "👑",
            2 => "🥈",
            3 => "🥉",
            _ => &format!("{:2}.", rank),
        };
        
        lines.push(format!(
            "║ {} {:16} T:{:3} C:{:3}  ║",
            marker, short_name, territories, creeps
        ));
        
        // Show leader name on next line for top 6
        if rank <= 6 {
            let leader_short = if leader.len() > 20 { &leader[..20] } else { leader };
            lines.push(format!(
                "║    {} {}",
                leader_short,
                " ".repeat(28 - leader_short.len())
            ));
        }
    }

    lines.push("╠══════════════════════════════════╣".to_string());
    
    // Summary
    let total_territories: usize = faction_data.iter().map(|f| f.3).sum();
    let total_creeps: usize = faction_data.iter().map(|f| f.4).sum();
    lines.push(format!(
        "║ Total: {} factions, {} rooms, {} creeps",
        faction_data.len(), total_territories, total_creeps
    ));
    lines.push("╚══════════════════════════════════╝".to_string());

    // Update text
    for mut text in query.iter_mut() {
        text.sections[0].value = lines.join("\n");
        text.sections[0].style.font_size = 9.0;
    }
}

/// Toggle faction leaderboard visibility
pub fn toggle_faction_leaderboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<FactionLeaderboardState>,
    panel: Query<Entity, With<FactionLeaderboardText>>,
    mut commands: Commands,
) {
    // F key toggles faction leaderboard
    if keyboard.just_pressed(KeyCode::KeyF) {
        state.visible = !state.visible;
        
        if state.visible {
            // Spawn panel
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "=== Factions ===\nLoading...",
                        TextStyle {
                            font_size: 9.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        width: Val::Px(300.0),
                        max_height: Val::Px(450.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
                    ..default()
                },
                FactionLeaderboardText,
            ));
        } else {
            // Despawn panel
            for entity in panel.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Plugin for faction leaderboard
pub struct FactionLeaderboardPlugin;

impl Plugin for FactionLeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactionLeaderboardState>()
            .add_systems(Startup, setup_faction_leaderboard)
            .add_systems(Update, (update_faction_leaderboard, toggle_faction_leaderboard));
    }
}