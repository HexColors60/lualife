//! Scarcity indicator widget for displaying global resource scarcity state.

use bevy::prelude::*;

use crate::resources::{GlobalResourceLevels, ResourceType, ScarcityConfig};

/// Marker for scarcity indicator text
#[derive(Component)]
pub struct ScarcityText;

/// Setup scarcity indicator UI
pub fn setup_scarcity_indicator(mut commands: Commands) {
    // Scarcity indicator at top-right
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Global Resources: ",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Loading...",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(30.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        ScarcityText,
    ));
}

/// Update scarcity indicator with current global resource levels
pub fn update_scarcity_indicator(
    mut query: Query<&mut Text, With<ScarcityText>>,
    levels: Option<Res<GlobalResourceLevels>>,
    config: Option<Res<ScarcityConfig>>,
) {
    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let Some(levels) = levels else {
        text.sections[1].value = "Initializing...".to_string();
        return;
    };

    let config = config.map(|c| c.clone()).unwrap_or_default();

    // Calculate scarcity for each resource type
    let resource_types = [
        ResourceType::Power,
        ResourceType::Iron,
        ResourceType::Copper,
        ResourceType::Silicon,
    ];

    let mut summary_parts: Vec<String> = Vec::new();
    let mut critical_count = 0;
    let mut scarce_count = 0;
    let mut abundant_count = 0;

    for resource_type in resource_types {
        let ratio = levels.fill_ratio(resource_type);
        let _status = if ratio < config.critical_threshold {
            critical_count += 1;
            "CRIT"
        } else if ratio < config.scarcity_threshold {
            scarce_count += 1;
            "LOW"
        } else if ratio > config.abundant_threshold {
            abundant_count += 1;
            "HIGH"
        } else {
            "OK"
        };

        let icon = match resource_type {
            ResourceType::Power => "⚡",
            ResourceType::Iron => "⚙",
            ResourceType::Copper => "⚡",
            ResourceType::Silicon => "◈",
            _ => "?",
        };

        summary_parts.push(format!("{}:{}%", icon, (ratio * 100.0) as u32));
    }

    // Overall status
    let (overall_status, color) = if critical_count > 0 {
        ("CRITICAL", Color::srgb(1.0, 0.3, 0.3))
    } else if scarce_count > 0 {
        ("LOW", Color::srgb(1.0, 0.8, 0.3))
    } else if abundant_count >= 2 {
        ("ABUNDANT", Color::srgb(0.3, 1.0, 0.5))
    } else {
        ("STABLE", Color::srgb(0.7, 0.9, 0.7))
    };

    text.sections[1].value = format!("[{}] {}", overall_status, summary_parts.join(" "));
    text.sections[1].style.color = color;
}