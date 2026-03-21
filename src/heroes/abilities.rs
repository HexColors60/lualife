//! Ability execution system for hero powers.
//!
//! Handles targeting, effects, and visual feedback for abilities.

use bevy::prelude::*;

use crate::core::TickNumber;
use crate::creeps::Creep;
use crate::factions::FactionId;
use crate::world::WorldPos;

use super::{AbilityExecutedEvent, AbilityType, Hero};

/// Active ability effect
#[derive(Component)]
pub struct AbilityEffect {
    pub ability_type: AbilityType,
    pub timer: Timer,
    pub source_faction: FactionId,
}

/// Temporary turret from Engineer ability
#[derive(Component)]
pub struct TemporaryTurret {
    pub faction_id: FactionId,
    pub timer: Timer,
    pub damage: f32,
    pub range: f32,
}

/// Trap from Scout ability
#[derive(Component)]
pub struct ScoutTrap {
    pub faction_id: FactionId,
    pub damage: f32,
    pub trigger_radius: f32,
}

/// Mark for Death debuff
#[derive(Component)]
pub struct MarkForDeathDebuff {
    pub extra_damage: f32,
    pub timer: Timer,
}

/// Vanish invisibility
#[derive(Component)]
pub struct VanishInvisibility {
    pub timer: Timer,
}

/// Taunt effect
#[derive(Component)]
pub struct TauntedBy {
    pub hero_entity: Entity,
    pub timer: Timer,
}

/// Process ability execution events
pub fn process_ability_events(
    mut events: EventReader<AbilityExecutedEvent>,
    mut commands: Commands,
    mut heroes: Query<&mut Hero>,
    mut creeps: Query<(Entity, &mut Creep, &Transform, Option<&mut TauntedBy>)>,
    mut damage_events: EventWriter<crate::render::DamageEvent>,
    mut shake_events: EventWriter<crate::render::ScreenShakeEvent>,
    mut game_log: ResMut<crate::ui::GameLog>,
    tick: Res<TickNumber>,
) {
    for event in events.read() {
        // Deduct mana and set cooldown
        if let Ok(mut hero) = heroes.get_mut(event.hero_entity) {
            hero.use_ability(event.ability, tick.0);
        }

        // Execute ability effect
        match event.ability {
            AbilityType::ShieldBash => {
                // Stun enemies in small area
                if let Some(pos) = event.target_pos {
                    apply_area_stun(&mut creeps, pos, 2.0, event.hero_entity);
                }
            }
            AbilityType::Taunt => {
                // Force enemies to target hero
                if let Some(pos) = event.target_pos {
                    apply_taunt(&mut creeps, pos, 5.0, event.hero_entity, tick.0);
                }
            }
            AbilityType::BerserkerRage => {
                // Self buff - handled in combat
            }
            AbilityType::Fireball => {
                // Area damage
                if let Some(pos) = event.target_pos {
                    let damage = 50.0; // Base fireball damage
                    apply_area_damage(&mut creeps, &mut damage_events, pos, 3.0, damage, event.hero_entity);

                    // Screen shake
                    shake_events.send(crate::render::ScreenShakeEvent::from_type(
                        crate::render::ShakeEventType::Medium,
                    ));

                    game_log.add("🔥 Fireball unleashed!".to_string());
                }
            }
            AbilityType::IceStorm => {
                // Area slow and damage
                if let Some(pos) = event.target_pos {
                    let damage = 30.0;
                    apply_area_damage(&mut creeps, &mut damage_events, pos, 4.0, damage, event.hero_entity);
                    // TODO: Apply slow effect

                    game_log.add("❄ Ice Storm freezes the area!".to_string());
                }
            }
            AbilityType::Teleport => {
                // Instant movement - handled by movement system
                game_log.add("✨ Hero teleported!".to_string());
            }
            AbilityType::SwiftDash => {
                // Quick movement burst
            }
            AbilityType::Reveal => {
                // Reveal hidden enemies
                if let Some(pos) = event.target_pos {
                    game_log.add(format!("👁 Reveal cast at ({}, {})", pos.x, pos.y));
                }
            }
            AbilityType::Trap => {
                // Place trap at location
                if let Some(pos) = event.target_pos {
                    commands.spawn((
                        ScoutTrap {
                            faction_id: event.faction_id,
                            damage: 40.0,
                            trigger_radius: 1.0,
                        },
                        // Position component would go here
                    ));
                }
            }
            AbilityType::RapidRepair => {
                // Heal nearby buildings
                game_log.add("🔧 Rapid Repair activated!".to_string());
            }
            AbilityType::BuildTurret => {
                // Create temporary turret
                if let Some(pos) = event.target_pos {
                    commands.spawn((
                        TemporaryTurret {
                            faction_id: event.faction_id,
                            timer: Timer::from_seconds(30.0, TimerMode::Once),
                            damage: 15.0,
                            range: 5.0,
                        },
                        // Position component
                    ));
                    game_log.add("🔨 Temporary turret deployed!".to_string());
                }
            }
            AbilityType::Overcharge => {
                // Boost building production
            }
            AbilityType::Rally => {
                // Heal nearby allies
                if let Some(pos) = event.target_pos {
                    let heal_amount = 20.0;
                    apply_area_heal(&mut creeps, pos, 5.0, heal_amount, event.faction_id);
                    game_log.add("🎺 Allies rally and recover!".to_string());
                }
            }
            AbilityType::BattleCry => {
                // Buff ally attack
            }
            AbilityType::TacticalRetreat => {
                // Speed boost for allies
            }
            AbilityType::Backstab => {
                // Massive single-target damage
                if let Some(target) = event.target_entity {
                    if let Ok((_, mut creep, transform, _)) = creeps.get_mut(target) {
                        let damage = 80.0;
                        creep.take_damage(damage);

                        damage_events.send(crate::render::DamageEvent {
                            target,
                            attacker_faction: event.faction_id,
                            damage: damage as u32,
                            position: transform.translation,
                        });

                        game_log.add("🗡 Backstab critical hit!".to_string());
                    }
                }
            }
            AbilityType::Vanish => {
                // Become invisible
                commands.entity(event.hero_entity).insert(VanishInvisibility {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                });
            }
            AbilityType::MarkDeath => {
                // Mark target for extra damage
                if let Some(target) = event.target_entity {
                    commands.entity(target).insert(MarkForDeathDebuff {
                        extra_damage: 0.5, // 50% extra damage
                        timer: Timer::from_seconds(10.0, TimerMode::Once),
                    });
                }
            }
        }
    }
}

/// Apply area damage to creeps
fn apply_area_damage(
    creeps: &mut Query<(Entity, &mut Creep, &Transform, Option<&mut TauntedBy>)>,
    damage_events: &mut EventWriter<crate::render::DamageEvent>,
    center: WorldPos,
    radius: f32,
    damage: f32,
    source_hero: Entity,
) {
    for (entity, mut creep, transform, _) in creeps.iter_mut() {
        let dist = ((creep.position.x as f32 - center.x as f32).powi(2)
            + (creep.position.y as f32 - center.y as f32).powi(2))
        .sqrt();

        if dist <= radius {
            creep.take_damage(damage);

            damage_events.send(crate::render::DamageEvent {
                target: entity,
                attacker_faction: FactionId(0), // Would need to get from hero
                damage: damage as u32,
                position: transform.translation,
            });
        }
    }
}

/// Apply stun to enemies in area
fn apply_area_stun(
    creeps: &mut Query<(Entity, &mut Creep, &Transform, Option<&mut TauntedBy>)>,
    center: WorldPos,
    radius: f32,
    _source_hero: Entity,
) {
    // Stun effect would disable creep actions temporarily
    for (_, mut creep, _, _) in creeps.iter_mut() {
        let dist = ((creep.position.x as f32 - center.x as f32).powi(2)
            + (creep.position.y as f32 - center.y as f32).powi(2))
        .sqrt();

        if dist <= radius {
            // Cancel current action
            creep.current_action = None;
        }
    }
}

/// Apply taunt to enemies
fn apply_taunt(
    creeps: &mut Query<(Entity, &mut Creep, &Transform, Option<&mut TauntedBy>)>,
    center: WorldPos,
    radius: f32,
    source_hero: Entity,
    tick: u64,
) {
    for (_, mut creep, _, taunt) in creeps.iter_mut() {
        let dist = ((creep.position.x as f32 - center.x as f32).powi(2)
            + (creep.position.y as f32 - center.y as f32).powi(2))
        .sqrt();

        if dist <= radius {
            // Force attack the taunting hero
            // Would set attack target to source_hero

            if let Some(mut t) = taunt {
                t.hero_entity = source_hero;
                t.timer = Timer::from_seconds(5.0, TimerMode::Once);
            }
        }
    }
}

/// Apply healing to allies in area
fn apply_area_heal(
    creeps: &mut Query<(Entity, &mut Creep, &Transform, Option<&mut TauntedBy>)>,
    center: WorldPos,
    radius: f32,
    heal_amount: f32,
    faction: FactionId,
) {
    for (_, mut creep, _, _) in creeps.iter_mut() {
        if creep.faction_id != faction {
            continue;
        }

        let dist = ((creep.position.x as f32 - center.x as f32).powi(2)
            + (creep.position.y as f32 - center.y as f32).powi(2))
        .sqrt();

        if dist <= radius {
            creep.hp = (creep.hp + heal_amount).min(creep.body.max_hp());
        }
    }
}

/// Update ability effects
pub fn update_ability_effects(
    mut commands: Commands,
    mut effects: Query<(Entity, &mut AbilityEffect)>,
    mut turrets: Query<(Entity, &mut TemporaryTurret)>,
    mut traps: Query<(Entity, &ScoutTrap)>,
    mut debuffs: Query<(Entity, &mut MarkForDeathDebuff)>,
    mut vanish: Query<(Entity, &mut VanishInvisibility)>,
    mut taunts: Query<(Entity, &mut TauntedBy)>,
    time: Res<Time>,
) {
    // Update effect timers
    for (entity, mut effect) in effects.iter_mut() {
        effect.timer.tick(time.delta());
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Update temporary turrets
    for (entity, mut turret) in turrets.iter_mut() {
        turret.timer.tick(time.delta());
        if turret.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Update debuffs
    for (entity, mut debuff) in debuffs.iter_mut() {
        debuff.timer.tick(time.delta());
        if debuff.timer.finished() {
            commands.entity(entity).remove::<MarkForDeathDebuff>();
        }
    }

    // Update vanish
    for (entity, mut invis) in vanish.iter_mut() {
        invis.timer.tick(time.delta());
        if invis.timer.finished() {
            commands.entity(entity).remove::<VanishInvisibility>();
        }
    }

    // Update taunts
    for (entity, mut taunt) in taunts.iter_mut() {
        taunt.timer.tick(time.delta());
        if taunt.timer.finished() {
            commands.entity(entity).remove::<TauntedBy>();
        }
    }
}

/// Temporary turret attack system
pub fn turret_attack_system(
    turrets: Query<(Entity, &TemporaryTurret, &Transform)>,
    mut creeps: Query<(Entity, &mut Creep, &Transform)>,
    mut damage_events: EventWriter<crate::render::DamageEvent>,
) {
    for (_, turret, turret_transform) in turrets.iter() {
        // Find nearest enemy
        let mut nearest_enemy: Option<(Entity, f32)> = None;

        for (entity, creep, transform) in creeps.iter() {
            if creep.faction_id == turret.faction_id {
                continue;
            }

            let dist = turret_transform.translation.distance(transform.translation);
            if dist <= turret.range {
                if nearest_enemy.is_none() || dist < nearest_enemy.unwrap().1 {
                    nearest_enemy = Some((entity, dist));
                }
            }
        }

        // Attack nearest enemy
        if let Some((target, _)) = nearest_enemy {
            if let Ok((_, mut creep, transform)) = creeps.get_mut(target) {
                creep.take_damage(turret.damage);

                damage_events.send(crate::render::DamageEvent {
                    target,
                    attacker_faction: turret.faction_id,
                    damage: turret.damage as u32,
                    position: transform.translation,
                });
            }
        }
    }
}

/// Plugin for ability systems
pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                process_ability_events,
                update_ability_effects,
                turret_attack_system,
            ),
        );
    }
}