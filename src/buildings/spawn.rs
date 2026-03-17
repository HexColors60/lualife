use bevy::prelude::*;

use crate::creeps::CreepBody;
use crate::creeps::Creep;
use crate::factions::FactionId;
use crate::world::WorldPos;

#[derive(Debug, Clone, Component)]
pub struct SpawnBuilding {
    pub spawning: bool,
    pub spawn_queue: Vec<CreepBody>,
    pub spawn_progress: f32,
    pub spawn_time: f32,
    pub faction_id: FactionId,
}

impl Default for SpawnBuilding {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnBuilding {
    pub fn new() -> Self {
        Self {
            spawning: false,
            spawn_queue: Vec::new(),
            spawn_progress: 0.0,
            spawn_time: 50.0, // ticks to spawn
            faction_id: FactionId(0),
        }
    }

    pub fn with_faction(mut self, faction_id: FactionId) -> Self {
        self.faction_id = faction_id;
        self
    }

    pub fn queue_spawn(&mut self, body: CreepBody) {
        self.spawn_queue.push(body);
        if !self.spawning {
            self.spawning = true;
        }
    }

    pub fn tick(&mut self) -> Option<CreepBody> {
        if !self.spawning || self.spawn_queue.is_empty() {
            return None;
        }

        self.spawn_progress += 1.0;

        if self.spawn_progress >= self.spawn_time {
            self.spawn_progress = 0.0;
            let body = self.spawn_queue.remove(0);

            if self.spawn_queue.is_empty() {
                self.spawning = false;
            }

            return Some(body);
        }

        None
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.spawn_time > 0.0 {
            self.spawn_progress / self.spawn_time
        } else {
            0.0
        }
    }
}

/// System to spawn creeps from spawn buildings
pub fn spawn_creep_system(
    mut spawns: Query<(&mut SpawnBuilding, &Transform, &super::Building)>,
    mut commands: Commands,
    mut creep_id_gen: ResMut<crate::creeps::CreepIdGenerator>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    for (mut spawn, transform, building) in spawns.iter_mut() {
        if let Some(body) = spawn.tick() {
            let pos = WorldPos::new(
                (transform.translation.x + 128.0) as i32,
                (transform.translation.y + 128.0) as i32,
            );

            let creep_id = creep_id_gen.next();
            let creep = Creep::new(
                creep_id,
                building.faction_id,
                pos,
                body,
            );

            commands.spawn(creep);

            game_log.add(format!(
                "Spawned creep {} for faction {}",
                creep_id,
                building.faction_id.0
            ));
        }
    }
}