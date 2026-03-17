use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Tower {
    pub range: f32,
    pub damage: f32,
    pub attack_speed: f32,
    pub attack_cooldown: f32,
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
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cooldown <= 0.0
    }

    pub fn attack(&mut self) -> f32 {
        if self.can_attack() {
            self.attack_cooldown = 1.0 / self.attack_speed;
            return self.damage;
        }
        0.0
    }

    pub fn tick(&mut self) {
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= 1.0;
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