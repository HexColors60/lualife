use bevy::prelude::*;

use crate::creeps::CreepBody;

#[derive(Debug, Clone, Component)]
pub struct SpawnBuilding {
    pub spawning: bool,
    pub spawn_queue: Vec<CreepBody>,
    pub spawn_progress: f32,
    pub spawn_time: f32,
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
        }
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
}