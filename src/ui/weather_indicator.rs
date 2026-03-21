//! Weather UI indicator showing current weather and effects.
//!
//! Displays weather type, modifiers, and visual feedback.

use bevy::prelude::*;

use crate::weather::{WeatherState, WeatherType};

/// Marker for the weather indicator panel.
#[derive(Component)]
pub struct WeatherIndicatorPanel;

/// Marker for weather effect text.
#[derive(Component)]
pub struct WeatherEffectText;

/// Plugin for weather indicator UI.
pub struct WeatherIndicatorPlugin;

impl Plugin for WeatherIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_weather_indicator)
            .add_systems(Update, update_weather_indicator);
    }
}

fn setup_weather_indicator(mut commands: Commands) {
    // Weather indicator at top-right, below any other UI
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.85)),
            border_color: BorderColor(Color::srgb(0.3, 0.3, 0.4)),
            ..default()
        },
        WeatherIndicatorPanel,
    )).with_children(|parent| {
        // Weather icon and name
        parent.spawn((
            TextBundle::from_section(
                "☀ Clear",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
        ));

        // Effects display
        parent.spawn((
            TextBundle::from_section(
                "Movement: 100% | Mining: 100%",
                TextStyle {
                    font_size: 10.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ),
            WeatherEffectText,
        ));
    });
}

fn update_weather_indicator(
    weather: Res<WeatherState>,
    mut query: Query<&mut Text, With<WeatherIndicatorPanel>>,
    mut effect_query: Query<&mut Text, (With<WeatherEffectText>, Without<WeatherIndicatorPanel>)>,
) {
    if !weather.is_changed() {
        return;
    }

    // Get weather icon and color
    let (icon, color) = weather_icon_and_color(weather.current_weather);

    // Update main weather text
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{} {}", icon, weather.current_weather.name());
        text.sections[0].style.color = color;
    }

    // Update effects text
    let movement = weather.current_weather.movement_modifier();
    let mining = weather.current_weather.mining_modifier();
    let combat = weather.current_weather.combat_modifier();
    let power = weather.current_weather.power_consumption_modifier();

    for mut text in effect_query.iter_mut() {
        text.sections[0].value = format!(
            "Move: {:.0}% | Mine: {:.0}% | Combat: {:.0}% | Power: {:.0}%",
            movement * 100.0,
            mining * 100.0,
            combat * 100.0,
            power * 100.0
        );

        // Color code based on severity
        let severity = weather_severity(weather.current_weather);
        text.sections[0].style.color = match severity {
            0 => Color::srgb(0.7, 0.7, 0.7), // Normal
            1 => Color::srgb(0.9, 0.9, 0.5), // Minor effects
            2 => Color::srgb(0.9, 0.6, 0.3), // Moderate effects
            _ => Color::srgb(0.9, 0.3, 0.3), // Severe effects
        };
    }
}

/// Get weather icon and display color
fn weather_icon_and_color(weather: WeatherType) -> (&'static str, Color) {
    match weather {
        WeatherType::Clear => ("☀", Color::srgb(1.0, 0.95, 0.6)),
        WeatherType::Cloudy => ("☁", Color::srgb(0.7, 0.7, 0.75)),
        WeatherType::Rain => ("🌧", Color::srgb(0.5, 0.6, 0.8)),
        WeatherType::Storm => ("⛈", Color::srgb(0.6, 0.4, 0.7)),
        WeatherType::Snow => ("❄", Color::srgb(0.85, 0.9, 1.0)),
        WeatherType::Fog => ("🌫", Color::srgb(0.6, 0.6, 0.65)),
        WeatherType::Heatwave => ("🔥", Color::srgb(1.0, 0.6, 0.3)),
    }
}

/// Get weather severity (0 = normal, 1 = minor, 2 = moderate, 3 = severe)
fn weather_severity(weather: WeatherType) -> u32 {
    match weather {
        WeatherType::Clear => 0,
        WeatherType::Cloudy => 0,
        WeatherType::Rain => 1,
        WeatherType::Storm => 3,
        WeatherType::Snow => 2,
        WeatherType::Fog => 2,
        WeatherType::Heatwave => 2,
    }
}