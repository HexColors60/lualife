use bevy::prelude::*;

use crate::mines::MineNode;

/// Marker for mine sprite entities
#[derive(Component)]
pub struct MineSprite {
    pub mine_entity: Entity,
}

/// System to spawn sprites for mines
pub fn spawn_mine_sprites(
    mut commands: Commands,
    mines: Query<(Entity, &MineNode), Without<MineSprite>>,
) {
    for (entity, mine) in mines.iter() {
        let color = mine.resource_type().color();

        // Position centered at origin (offset by -128 like rooms)
        let x = mine.position.x as f32 - 128.0;
        let y = mine.position.y as f32 - 128.0;

        // Size based on fill ratio
        let size = 1.0 + mine.fill_ratio() * 1.5;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(
                        color.0 as f32 / 255.0,
                        color.1 as f32 / 255.0,
                        color.2 as f32 / 255.0,
                    ),
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.5),
                ..default()
            },
            MineSprite { mine_entity: entity },
        ));
    }
}

/// System to update mine sprites
pub fn update_mine_sprites(
    mines: Query<&MineNode, Changed<MineNode>>,
    mut sprites: Query<(&MineSprite, &mut Sprite, &mut Transform)>,
) {
    for (sprite, mut sprite_comp, mut transform) in sprites.iter_mut() {
        if let Ok(mine) = mines.get(sprite.mine_entity) {
            let color = mine.resource_type().color();
            sprite_comp.color = Color::srgb(
                color.0 as f32 / 255.0,
                color.1 as f32 / 255.0,
                color.2 as f32 / 255.0,
            );

            // Update size based on fill ratio
            let size = 1.0 + mine.fill_ratio() * 1.5;
            sprite_comp.custom_size = Some(Vec2::new(size, size));
        }
    }
}