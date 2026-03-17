use bevy::prelude::*;

use crate::buildings::Building;
use crate::factions::FactionRegistry;

/// Marker for building sprite entities
#[derive(Component)]
pub struct BuildingSprite {
    pub building_entity: Entity,
}

/// Get color for building type
fn building_color(building_type: crate::buildings::BuildingType) -> Color {
    use crate::buildings::BuildingType;

    match building_type {
        BuildingType::BaseCore => Color::srgb(0.9, 0.9, 0.2),
        BuildingType::Spawn => Color::srgb(0.3, 0.8, 0.3),
        BuildingType::Storage => Color::srgb(0.6, 0.5, 0.3),
        BuildingType::PowerDepot => Color::srgb(0.9, 0.9, 0.1),
        BuildingType::MineExtractor => Color::srgb(0.5, 0.5, 0.6),
        BuildingType::Refinery => Color::srgb(0.7, 0.4, 0.2),
        BuildingType::Workshop => Color::srgb(0.4, 0.6, 0.7),
        BuildingType::Wall => Color::srgb(0.4, 0.4, 0.4),
        BuildingType::Tower => Color::srgb(0.8, 0.3, 0.3),
        BuildingType::Road => Color::srgb(0.3, 0.3, 0.3),
        BuildingType::ScriptRelay => Color::srgb(0.5, 0.2, 0.8),
        BuildingType::Scanner => Color::srgb(0.2, 0.7, 0.8),
        BuildingType::RepairBay => Color::srgb(0.3, 0.8, 0.5),
        BuildingType::Factory => Color::srgb(0.6, 0.6, 0.3),
        BuildingType::Lab => Color::srgb(0.7, 0.3, 0.7),
        BuildingType::Barracks => Color::srgb(0.7, 0.4, 0.3),
    }
}

/// Get size for building type
fn building_size(building_type: crate::buildings::BuildingType) -> Vec2 {
    use crate::buildings::BuildingType;

    match building_type {
        BuildingType::BaseCore => Vec2::new(4.0, 4.0),
        BuildingType::Spawn => Vec2::new(3.0, 3.0),
        BuildingType::Storage => Vec2::new(3.0, 3.0),
        BuildingType::PowerDepot => Vec2::new(2.5, 2.5),
        BuildingType::MineExtractor => Vec2::new(2.0, 2.0),
        BuildingType::Refinery => Vec2::new(3.0, 3.0),
        BuildingType::Workshop => Vec2::new(2.5, 2.5),
        BuildingType::Wall => Vec2::new(1.0, 1.0),
        BuildingType::Tower => Vec2::new(2.0, 2.0),
        BuildingType::Road => Vec2::new(1.0, 1.0),
        BuildingType::ScriptRelay => Vec2::new(2.0, 2.0),
        BuildingType::Scanner => Vec2::new(2.0, 2.0),
        BuildingType::RepairBay => Vec2::new(2.5, 2.5),
        BuildingType::Factory => Vec2::new(3.0, 3.0),
        BuildingType::Lab => Vec2::new(2.5, 2.5),
        BuildingType::Barracks => Vec2::new(3.0, 3.0),
    }
}

/// System to spawn sprites for buildings
pub fn spawn_building_sprites(
    mut commands: Commands,
    buildings: Query<(Entity, &Building), Without<BuildingSprite>>,
    factions: Res<FactionRegistry>,
) {
    for (entity, building) in buildings.iter() {
        // Get faction color tint
        let faction_tint = factions
            .get(building.faction_id)
            .map(|f| {
                let (r, g, b) = f.color;
                Color::srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.3)
            })
            .unwrap_or(Color::srgba(1.0, 1.0, 1.0, 0.3));

        let base_color = building_color(building.building_type);
        let size = building_size(building.building_type);

        // Position centered at origin (offset by -128 like rooms)
        let x = building.position.x as f32 - 128.0;
        let y = building.position.y as f32 - 128.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: base_color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.8),
                ..default()
            },
            BuildingSprite { building_entity: entity },
        ));
    }
}

/// System to update building sprites
pub fn update_building_sprites(
    buildings: Query<&Building, Changed<Building>>,
    mut sprites: Query<(&BuildingSprite, &mut Sprite, &mut Transform)>,
) {
    for (sprite, mut sprite_comp, mut transform) in sprites.iter_mut() {
        if let Ok(building) = buildings.get(sprite.building_entity) {
            // Update position
            let x = building.position.x as f32 - 128.0;
            let y = building.position.y as f32 - 128.0;
            transform.translation.x = x;
            transform.translation.y = y;

            // Update color based on HP
            let hp_ratio = building.hp / building.max_hp;
            let base_color = building_color(building.building_type);

            // Darken when damaged
            sprite_comp.color = Color::srgb(
                base_color.to_srgba().red * hp_ratio,
                base_color.to_srgba().green * hp_ratio,
                base_color.to_srgba().blue * hp_ratio,
            );
        }
    }
}