use std::collections::HashMap;

use bevy::prelude::*;

use crate::resources::ResourceType;

/// Resource bar widget for displaying faction resources
pub struct ResourceBar {
    pub resources: HashMap<ResourceType, (u32, u32)>, // (current, max)
}

impl Default for ResourceBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceBar {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn set(&mut self, resource: ResourceType, current: u32, max: u32) {
        self.resources.insert(resource, (current, max));
    }

    pub fn get(&self, resource: ResourceType) -> (u32, u32) {
        self.resources.get(&resource).copied().unwrap_or((0, 0))
    }

    pub fn fill_ratio(&self, resource: ResourceType) -> f32 {
        let (current, max) = self.get(resource);
        if max == 0 {
            return 0.0;
        }
        current as f32 / max as f32
    }
}

/// Marker for resource text
#[derive(Component)]
pub struct ResourceText;

/// Setup resource bar UI
pub fn setup_resource_bar(mut commands: Commands) {
    // Resource bar at top-center
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Resources: 32 AI factions competing",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(300.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        ResourceText,
    ));
}

/// Update resource bar display (placeholder for now)
pub fn update_resource_bar(
    mut query: Query<&mut Text, With<ResourceText>>,
    selection: Res<crate::debug::SelectionState>,
) {
    if !selection.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        if selection.selected_entity.is_some() {
            text.sections[0].value = "Resources: (see unit panel)".to_string();
        } else {
            text.sections[0].value = "Resources: 32 AI factions competing".to_string();
        }
    }
}
