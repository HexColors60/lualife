use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::debug::SelectionState;
use crate::creeps::Creep;
use crate::render::CreepSprite;
use crate::render::MainCamera;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            mouse_click_selection,
            keyboard_deselect,
        ));
    }
}

/// System to handle mouse clicks for entity selection
fn mouse_click_selection(
    mut selection: ResMut<SelectionState>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    creep_sprites: Query<(&CreepSprite, &Transform)>,
    creeps: Query<&Creep>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // Only handle left click press
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    // Convert screen position to world position
    let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // Find the closest creep sprite to the click position
    let mut closest_creep_entity: Option<Entity> = None;
    let mut closest_distance: f32 = f32::MAX;
    const CLICK_RADIUS: f32 = 5.0; // Selection radius in world units

    for (sprite, transform) in creep_sprites.iter() {
        let sprite_pos = transform.translation.truncate();
        let distance = world_position.distance(sprite_pos);

        if distance < CLICK_RADIUS && distance < closest_distance {
            closest_distance = distance;
            closest_creep_entity = Some(sprite.creep_entity);
        }
    }

    // Update selection
    if let Some(entity) = closest_creep_entity {
        // Get the creep data
        if let Ok(creep) = creeps.get(entity) {
            selection.select(entity);
            game_log.add(format!(
                "Selected creep {} (Faction {:?})",
                creep.id, creep.faction_id
            ));
        } else {
            selection.select(entity);
            game_log.add("Selected entity".to_string());
        }
    } else {
        // Clicked on empty space - deselect
        if selection.selected_entity.is_some() {
            selection.deselect();
            game_log.add("Deselected".to_string());
        }
    }
}

/// System to handle keyboard deselection
fn keyboard_deselect(
    mut selection: ResMut<SelectionState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        selection.deselect();
    }
}