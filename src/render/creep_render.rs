use bevy::prelude::*;

use crate::creeps::Creep;
use crate::factions::FactionRegistry;

/// Marker for creep sprite entities
#[derive(Component)]
pub struct CreepSprite {
    pub creep_entity: Entity,
}

/// System to spawn sprites for newly created creeps
pub fn spawn_creep_sprites(
    mut commands: Commands,
    creeps: Query<(Entity, &Creep), Without<CreepSprite>>,
    factions: Res<FactionRegistry>,
) {
    for (entity, creep) in creeps.iter() {
        // Get faction color
        let (r, g, b) = factions
            .get(creep.faction_id)
            .map(|f| f.color)
            .unwrap_or((255, 255, 255));

        let color = Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

        // Position centered at origin (offset by -128 like rooms)
        let x = creep.position.x as f32 - 128.0;
        let y = creep.position.y as f32 - 128.0;

        // Spawn creep sprite
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(2.0, 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 1.0),
                ..default()
            },
            CreepSprite { creep_entity: entity },
        ));
    }
}

/// System to update creep sprite positions
pub fn update_creep_sprites(
    creeps: Query<&Creep, Changed<Creep>>,
    mut sprites: Query<(&CreepSprite, &mut Transform)>,
) {
    for (sprite, mut transform) in sprites.iter_mut() {
        if let Ok(creep) = creeps.get(sprite.creep_entity) {
            let x = creep.position.x as f32 - 128.0;
            let y = creep.position.y as f32 - 128.0;
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}