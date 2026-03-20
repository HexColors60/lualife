use bevy::prelude::*;

use crate::buildings::Building;
use crate::creeps::Creep;

#[derive(Debug, Clone, Component)]
pub struct Tower {
    pub range: f32,
    pub damage: f32,
    pub attack_speed: f32,
    pub attack_cooldown: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub energy_per_attack: f32,
}

impl Default for Tower {
    fn default() -> Self {
        Self::new()
    }
}

impl Tower {
    pub fn new() -> Self {
        Self {
            range: 5.0,
            damage: 50.0,
            attack_speed: 1.0,
            attack_cooldown: 0.0,
            energy: 100.0,
            max_energy: 100.0,
            energy_per_attack: 10.0,
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cooldown <= 0.0 && self.energy >= self.energy_per_attack
    }

    pub fn attack(&mut self) -> f32 {
        if self.can_attack() {
            self.attack_cooldown = 1.0 / self.attack_speed;
            self.energy -= self.energy_per_attack;
            return self.damage;
        }
        0.0
    }

    pub fn tick(&mut self) {
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= 1.0;
        }
        // Slowly regenerate energy
        if self.energy < self.max_energy {
            self.energy = (self.energy + 0.5).min(self.max_energy);
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct Wall {
    pub hp: f32,
    pub max_hp: f32,
}

impl Wall {
    pub fn new() -> Self {
        Self {
            hp: 500.0,
            max_hp: 500.0,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    pub fn repair(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

/// System for towers to attack enemy creeps
pub fn tower_attack_system(
    mut towers: Query<(&mut Tower, &Building, &Transform)>,
    mut creeps: Query<(Entity, &mut Creep, &Transform)>,
    mut particle_events: EventWriter<crate::render::ParticleEvent>,
    mut shake_events: EventWriter<crate::render::ScreenShakeEvent>,
) {
    // Update tower cooldowns
    for (mut tower, _, _) in towers.iter_mut() {
        tower.tick();
    }

    // Find targets and attack
    for (mut tower, building, tower_transform) in towers.iter_mut() {
        if !tower.can_attack() {
            continue;
        }

        let tower_x = tower_transform.translation.x + 128.0;
        let tower_y = tower_transform.translation.y + 128.0;

        // Find nearest enemy creep in range
        let mut nearest_enemy: Option<Entity> = None;
        let mut nearest_dist = f32::MAX;
        let mut nearest_pos: Option<Vec3> = None;

        for (entity, creep, creep_transform) in creeps.iter() {
            // Skip same faction
            if creep.faction_id == building.faction_id {
                continue;
            }

            let creep_x = creep_transform.translation.x + 128.0;
            let creep_y = creep_transform.translation.y + 128.0;

            let dx = tower_x - creep_x;
            let dy = tower_y - creep_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= tower.range && dist < nearest_dist {
                nearest_dist = dist;
                nearest_enemy = Some(entity);
                nearest_pos = Some(creep_transform.translation);
            }
        }

        // Attack if enemy found
        if let Some(enemy_entity) = nearest_enemy {
            let damage = tower.attack();
            // Apply damage to the target creep
            if let Ok((_, mut enemy_creep, creep_transform)) = creeps.get_mut(enemy_entity) {
                enemy_creep.take_damage(damage);
                
                // Spawn spark particles at impact point
                particle_events.send(crate::render::ParticleEvent::Sparks {
                    position: creep_transform.translation,
                });
                
                // Light screen shake for tower hit
                shake_events.send(crate::render::ScreenShakeEvent::from_type(
                    crate::render::ShakeEventType::Light
                ));
            }
        }
    }
}