use std::collections::HashMap;

use bevy::prelude::*;

use crate::world::WorldPos;

#[derive(Resource, Debug, Clone, Default)]
pub struct PathCache {
    paths: HashMap<(WorldPos, WorldPos), Vec<WorldPos>>,
    max_size: usize,
}

impl PathCache {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            max_size: 10000,
        }
    }

    pub fn get(&self, from: WorldPos, to: WorldPos) -> Option<&Vec<WorldPos>> {
        self.paths.get(&(from, to))
    }

    pub fn insert(&mut self, from: WorldPos, to: WorldPos, path: Vec<WorldPos>) {
        if self.paths.len() >= self.max_size {
            // Simple eviction: clear half
            let keys: Vec<_> = self.paths.keys().take(self.max_size / 2).cloned().collect();
            for key in keys {
                self.paths.remove(&key);
            }
        }

        self.paths.insert((from, to), path);
    }

    pub fn clear(&mut self) {
        self.paths.clear();
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}
