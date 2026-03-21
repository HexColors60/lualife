//! Hero system for special units with unique abilities.
//!
//! Heroes are powerful units that can level up and gain abilities.
//! Each faction can have heroes that provide strategic advantages.

mod abilities;
mod render;
mod ui;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::creeps::Creep;
use crate::factions::FactionId;

pub use abilities::*;
pub use render::*;
pub use ui::*;

/// Hero types with unique abilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HeroType {
    #[default]
    Warrior,    // Tanky melee fighter
    Mage,       // Ranged area damage
    Scout,      // Fast movement, extended vision
    Engineer,   // Building boost, repair
    Commander,  // Buff nearby allies
    Assassin,   // High single-target damage
}

impl HeroType {
    pub fn name(&self) -> &'static str {
        match self {
            HeroType::Warrior => "Warrior",
            HeroType::Mage => "Mage",
            HeroType::Scout => "Scout",
            HeroType::Engineer => "Engineer",
            HeroType::Commander => "Commander",
            HeroType::Assassin => "Assassin",
        }
    }

    /// Base stats for each hero type
    pub fn base_stats(&self) -> HeroStats {
        match self {
            HeroType::Warrior => HeroStats {
                max_hp: 200.0,
                attack: 25.0,
                defense: 15.0,
                speed: 0.8,
                vision_range: 5,
                ability_power: 1.0,
            },
            HeroType::Mage => HeroStats {
                max_hp: 100.0,
                attack: 35.0,
                defense: 5.0,
                speed: 0.9,
                vision_range: 7,
                ability_power: 2.0,
            },
            HeroType::Scout => HeroStats {
                max_hp: 80.0,
                attack: 15.0,
                defense: 5.0,
                speed: 1.5,
                vision_range: 10,
                ability_power: 0.8,
            },
            HeroType::Engineer => HeroStats {
                max_hp: 120.0,
                attack: 10.0,
                defense: 10.0,
                speed: 0.7,
                vision_range: 6,
                ability_power: 1.2,
            },
            HeroType::Commander => HeroStats {
                max_hp: 150.0,
                attack: 20.0,
                defense: 12.0,
                speed: 0.85,
                vision_range: 8,
                ability_power: 1.5,
            },
            HeroType::Assassin => HeroStats {
                max_hp: 90.0,
                attack: 45.0,
                defense: 3.0,
                speed: 1.3,
                vision_range: 6,
                ability_power: 1.8,
            },
        }
    }

    /// Get available abilities for this hero type
    pub fn abilities(&self) -> Vec<AbilityType> {
        match self {
            HeroType::Warrior => vec![AbilityType::ShieldBash, AbilityType::Taunt, AbilityType::BerserkerRage],
            HeroType::Mage => vec![AbilityType::Fireball, AbilityType::IceStorm, AbilityType::Teleport],
            HeroType::Scout => vec![AbilityType::SwiftDash, AbilityType::Reveal, AbilityType::Trap],
            HeroType::Engineer => vec![AbilityType::RapidRepair, AbilityType::BuildTurret, AbilityType::Overcharge],
            HeroType::Commander => vec![AbilityType::Rally, AbilityType::BattleCry, AbilityType::TacticalRetreat],
            HeroType::Assassin => vec![AbilityType::Backstab, AbilityType::Vanish, AbilityType::MarkDeath],
        }
    }
}

/// Hero stat block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStats {
    pub max_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub vision_range: u32,
    pub ability_power: f32,
}

impl Default for HeroStats {
    fn default() -> Self {
        HeroType::Warrior.base_stats()
    }
}

/// Ability types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityType {
    // Warrior
    ShieldBash,      // Stun nearby enemies
    Taunt,           // Force enemies to attack hero
    BerserkerRage,   // Increased damage when low HP

    // Mage
    Fireball,        // Area damage
    IceStorm,        // Slow enemies in area
    Teleport,        // Instant movement

    // Scout
    SwiftDash,       // Quick movement
    Reveal,          // Reveal hidden enemies
    Trap,            // Place damage trap

    // Engineer
    RapidRepair,     // Heal nearby buildings
    BuildTurret,     // Create temporary turret
    Overcharge,      // Boost building production

    // Commander
    Rally,           // Heal nearby allies
    BattleCry,       // Buff ally attack
    TacticalRetreat, // Speed boost for allies

    // Assassin
    Backstab,        // Massive damage from behind
    Vanish,           // Become invisible
    MarkDeath,        // Mark target for extra damage
}

impl AbilityType {
    pub fn name(&self) -> &'static str {
        match self {
            AbilityType::ShieldBash => "Shield Bash",
            AbilityType::Taunt => "Taunt",
            AbilityType::BerserkerRage => "Berserker Rage",
            AbilityType::Fireball => "Fireball",
            AbilityType::IceStorm => "Ice Storm",
            AbilityType::Teleport => "Teleport",
            AbilityType::SwiftDash => "Swift Dash",
            AbilityType::Reveal => "Reveal",
            AbilityType::Trap => "Trap",
            AbilityType::RapidRepair => "Rapid Repair",
            AbilityType::BuildTurret => "Build Turret",
            AbilityType::Overcharge => "Overcharge",
            AbilityType::Rally => "Rally",
            AbilityType::BattleCry => "Battle Cry",
            AbilityType::TacticalRetreat => "Tactical Retreat",
            AbilityType::Backstab => "Backstab",
            AbilityType::Vanish => "Vanish",
            AbilityType::MarkDeath => "Mark for Death",
        }
    }

    pub fn cooldown_ticks(&self) -> u64 {
        match self {
            AbilityType::ShieldBash => 30,
            AbilityType::Taunt => 60,
            AbilityType::BerserkerRage => 120,
            AbilityType::Fireball => 40,
            AbilityType::IceStorm => 80,
            AbilityType::Teleport => 50,
            AbilityType::SwiftDash => 20,
            AbilityType::Reveal => 45,
            AbilityType::Trap => 60,
            AbilityType::RapidRepair => 35,
            AbilityType::BuildTurret => 100,
            AbilityType::Overcharge => 90,
            AbilityType::Rally => 50,
            AbilityType::BattleCry => 70,
            AbilityType::TacticalRetreat => 55,
            AbilityType::Backstab => 25,
            AbilityType::Vanish => 40,
            AbilityType::MarkDeath => 35,
        }
    }

    pub fn mana_cost(&self) -> f32 {
        match self {
            AbilityType::ShieldBash => 20.0,
            AbilityType::Taunt => 30.0,
            AbilityType::BerserkerRage => 50.0,
            AbilityType::Fireball => 35.0,
            AbilityType::IceStorm => 60.0,
            AbilityType::Teleport => 40.0,
            AbilityType::SwiftDash => 15.0,
            AbilityType::Reveal => 25.0,
            AbilityType::Trap => 30.0,
            AbilityType::RapidRepair => 25.0,
            AbilityType::BuildTurret => 80.0,
            AbilityType::Overcharge => 45.0,
            AbilityType::Rally => 35.0,
            AbilityType::BattleCry => 40.0,
            AbilityType::TacticalRetreat => 30.0,
            AbilityType::Backstab => 20.0,
            AbilityType::Vanish => 35.0,
            AbilityType::MarkDeath => 25.0,
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            AbilityType::ShieldBash => 2.0,
            AbilityType::Taunt => 5.0,
            AbilityType::BerserkerRage => 0.0, // Self
            AbilityType::Fireball => 8.0,
            AbilityType::IceStorm => 6.0,
            AbilityType::Teleport => 10.0,
            AbilityType::SwiftDash => 5.0,
            AbilityType::Reveal => 8.0,
            AbilityType::Trap => 3.0,
            AbilityType::RapidRepair => 5.0,
            AbilityType::BuildTurret => 2.0,
            AbilityType::Overcharge => 4.0,
            AbilityType::Rally => 6.0,
            AbilityType::BattleCry => 8.0,
            AbilityType::TacticalRetreat => 10.0,
            AbilityType::Backstab => 1.5,
            AbilityType::Vanish => 0.0, // Self
            AbilityType::MarkDeath => 6.0,
        }
    }
}

/// Hero entity component
#[derive(Component, Debug, Clone)]
pub struct Hero {
    pub hero_type: HeroType,
    pub level: u32,
    pub experience: u32,
    pub mana: f32,
    pub max_mana: f32,
    pub available_abilities: Vec<AbilityType>,
    pub cooldowns: HashMap<AbilityType, u64>,
    pub kill_count: u32,
}

impl Hero {
    pub fn new(hero_type: HeroType) -> Self {
        Self {
            hero_type,
            level: 1,
            experience: 0,
            mana: 100.0,
            max_mana: 100.0,
            available_abilities: vec![hero_type.abilities()[0]],
            cooldowns: HashMap::new(),
            kill_count: 0,
        }
    }

    /// Experience needed for next level
    pub fn exp_for_next_level(&self) -> u32 {
        100 * self.level * self.level
    }

    /// Add experience and check for level up
    pub fn add_experience(&mut self, amount: u32) -> bool {
        self.experience += amount;
        let needed = self.exp_for_next_level();
        if self.experience >= needed {
            self.level_up();
            true
        } else {
            false
        }
    }

    /// Level up the hero
    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;

        // Unlock new abilities at certain levels
        let abilities = self.hero_type.abilities();
        if self.level >= 3 && self.available_abilities.len() < 2 {
            self.available_abilities.push(abilities[1]);
        }
        if self.level >= 5 && self.available_abilities.len() < 3 {
            self.available_abilities.push(abilities[2]);
        }

        // Increase mana pool
        self.max_mana += 10.0;
        self.mana = self.max_mana;
    }

    /// Check if ability can be used
    pub fn can_use_ability(&self, ability: AbilityType, tick: u64) -> bool {
        // Check if ability is unlocked
        if !self.available_abilities.contains(&ability) {
            return false;
        }

        // Check mana
        if self.mana < ability.mana_cost() {
            return false;
        }

        // Check cooldown
        if let Some(&cooldown_end) = self.cooldowns.get(&ability) {
            if tick < cooldown_end {
                return false;
            }
        }

        true
    }

    /// Use an ability
    pub fn use_ability(&mut self, ability: AbilityType, tick: u64) {
        self.mana -= ability.mana_cost();
        self.cooldowns.insert(ability, tick + ability.cooldown_ticks());
    }

    /// Regenerate mana
    pub fn regen_mana(&mut self, amount: f32) {
        self.mana = (self.mana + amount).min(self.max_mana);
    }

    /// Update cooldowns
    pub fn update_cooldowns(&mut self, tick: u64) {
        self.cooldowns.retain(|_, &mut end| end > tick);
    }

    /// Get scaled stats based on level
    pub fn get_scaled_stats(&self) -> HeroStats {
        let base = self.hero_type.base_stats();
        let scale = 1.0 + (self.level - 1) as f32 * 0.1;
        HeroStats {
            max_hp: base.max_hp * scale,
            attack: base.attack * scale,
            defense: base.defense * scale,
            speed: base.speed,
            vision_range: base.vision_range + (self.level / 3) as u32,
            ability_power: base.ability_power * scale,
        }
    }
}

/// Ability execution event
#[derive(Event, Debug, Clone)]
pub struct AbilityExecutedEvent {
    pub hero_entity: Entity,
    pub hero: Hero,
    pub faction_id: FactionId,
    pub ability: AbilityType,
    pub target_pos: Option<crate::world::WorldPos>,
    pub target_entity: Option<Entity>,
}
/// Hero spawned event
#[derive(Event, Debug, Clone)]
pub struct HeroSpawnedEvent {
    pub hero_entity: Entity,
    pub hero_type: HeroType,
    pub faction_id: FactionId,
}

/// Hero level up event
#[derive(Event, Debug, Clone)]
pub struct HeroLevelUpEvent {
    pub hero_entity: Entity,
    pub new_level: u32,
    pub hero_type: HeroType,
}

/// Hero registry for tracking all heroes
#[derive(Resource, Debug, Clone, Default)]
pub struct HeroRegistry {
    pub heroes: Vec<(Entity, FactionId, HeroType, u32)>, // entity, faction, type, level
}

impl HeroRegistry {
    pub fn get_heroes_by_faction(&self, faction_id: FactionId) -> Vec<Entity> {
        self.heroes
            .iter()
            .filter(|(_, f, _, _)| *f == faction_id)
            .map(|(e, _, _, _)| *e)
            .collect()
    }

    pub fn get_highest_level_hero(&self) -> Option<(Entity, u32)> {
        self.heroes
            .iter()
            .map(|(e, _, _, lvl)| (*e, *lvl))
            .max_by_key(|(_, lvl)| *lvl)
    }
}

/// Mana regeneration system
pub fn mana_regen_system(
    mut heroes: Query<&mut Hero>,
    time: Res<Time>,
) {
    let regen_rate = 0.5; // Mana per second
    let delta = time.delta_seconds();

    for mut hero in heroes.iter_mut() {
        hero.regen_mana(regen_rate * delta);
    }
}

/// Hero cooldown update system
pub fn hero_cooldown_system(
    mut heroes: Query<&mut Hero>,
    tick: Res<crate::core::TickNumber>,
) {
    let current_tick = tick.0;

    for mut hero in heroes.iter_mut() {
        hero.update_cooldowns(current_tick);
    }
}

/// Hero experience and leveling system
pub fn hero_leveling_system(
    mut heroes: Query<(Entity, &mut Hero)>,
    mut events: EventWriter<HeroLevelUpEvent>,
) {
    for (entity, mut hero) in heroes.iter_mut() {
        let needed = hero.exp_for_next_level();
        if hero.experience >= needed {
            let old_level = hero.level;
            hero.level_up();

            if hero.level > old_level {
                events.send(HeroLevelUpEvent {
                    hero_entity: entity,
                    new_level: hero.level,
                    hero_type: hero.hero_type,
                });
            }
        }
    }
}

/// Update hero registry
pub fn update_hero_registry(
    mut registry: ResMut<HeroRegistry>,
    heroes: Query<(Entity, &Hero, &Creep)>,
) {
    registry.heroes.clear();
    for (entity, hero, creep) in heroes.iter() {
        registry.heroes.push((entity, creep.faction_id, hero.hero_type, hero.level));
    }
}

/// Plugin for hero system
pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeroRegistry>()
            .add_event::<AbilityExecutedEvent>()
            .add_event::<HeroSpawnedEvent>()
            .add_event::<HeroLevelUpEvent>()
            .add_systems(
                Update,
                (
                    mana_regen_system,
                    hero_cooldown_system,
                    hero_leveling_system,
                    update_hero_registry,
                ),
            );
    }
}