//! Hero rendering and visual effects.
//!
//! Renders heroes with distinct appearances based on type and level.

use bevy::prelude::*;

use crate::factions::FactionRegistry;

use super::{Hero, HeroType};

/// Marker for hero sprite
#[derive(Component)]
pub struct HeroSprite {
    pub hero_type: HeroType,
}

/// Spawn hero sprites
pub fn spawn_hero_sprites(
    mut commands: Commands,
    heroes: Query<(Entity, &Hero, &crate::creeps::Creep), Added<Hero>>,
    factions: Res<FactionRegistry>,
) {
    for (entity, hero, creep) in heroes.iter() {
        let _faction_color = factions.get(creep.faction_id)
            .map(|f| Color::srgb(
                f.color.0 as f32 / 255.0,
                f.color.1 as f32 / 255.0,
                f.color.2 as f32 / 255.0
            ))
            .unwrap_or(Color::srgb(0.5, 0.5, 0.5));

        // Hero color varies by type
        let hero_color = match hero.hero_type {
            HeroType::Warrior => Color::srgb(0.9, 0.3, 0.3),    // Red
            HeroType::Mage => Color::srgb(0.3, 0.5, 0.9),        // Blue
            HeroType::Scout => Color::srgb(0.3, 0.9, 0.3),       // Green
            HeroType::Engineer => Color::srgb(0.9, 0.7, 0.2),    // Yellow
            HeroType::Commander => Color::srgb(0.8, 0.5, 0.9),   // Purple
            HeroType::Assassin => Color::srgb(0.3, 0.3, 0.3),    // Dark gray
        };

        // Size scales with level
        let base_size = 12.0;
        let size = base_size + (hero.level as f32 * 1.5);

        // Calculate position
        let x = creep.position.x as f32 - 128.0;
        let y = creep.position.y as f32 - 128.0;

        // Spawn hero sprite (larger than regular creeps)
        let sprite_entity = commands.spawn((
            Sprite {
                color: hero_color,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Transform::from_xyz(x, y, 15.0), // Higher Z than creeps
            HeroSprite {
                hero_type: hero.hero_type,
            },
        )).id();

        // Link sprite to hero entity (parent-child relationship could be used)
    }
}

/// Update hero sprite positions and appearance
pub fn update_hero_sprites(
    heroes: Query<(&Hero, &crate::creeps::Creep)>,
    mut sprites: Query<(&mut Sprite, &mut Transform), With<HeroSprite>>,
) {
    // This would need a proper linking mechanism between heroes and their sprites
    // For now, this is a placeholder
}

/// Spawn level-up effect
pub fn spawn_level_up_effect(
    mut events: EventReader<super::HeroLevelUpEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        // Spawn particle effect for level up
        // Would integrate with the particle system

        // Flash effect
        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 0.9, 0.3, 0.8),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 20.0), // Would use actual hero position
            super::abilities::AbilityEffect {
                ability_type: super::AbilityType::Rally, // Reuse for visual
                timer: Timer::from_seconds(0.5, TimerMode::Once),
                source_faction: crate::factions::FactionId(0),
            },
        ));
    }
}

/// Render ability effects
pub fn render_ability_effects(
    mut effects: Query<(&mut Sprite, &super::abilities::AbilityEffect)>,
    time: Res<Time>,
) {
    for (mut sprite, effect) in effects.iter_mut() {
        // Fade out effect over time
        let progress = effect.timer.fraction();
        let alpha = 1.0 - progress;

        sprite.color = match effect.ability_type {
            super::AbilityType::Fireball => Color::srgba(1.0, 0.5, 0.1, alpha * 0.8),
            super::AbilityType::IceStorm => Color::srgba(0.5, 0.7, 1.0, alpha * 0.6),
            super::AbilityType::Rally => Color::srgba(0.3, 1.0, 0.3, alpha * 0.5),
            _ => Color::srgba(1.0, 1.0, 1.0, alpha * 0.5),
        };
    }
}

/// Plugin for hero rendering
pub struct HeroRenderPlugin;

impl Plugin for HeroRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_hero_sprites,
                update_hero_sprites,
                spawn_level_up_effect,
                render_ability_effects,
            ),
        );
    }
}