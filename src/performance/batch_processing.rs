use bevy::prelude::*;
use std::time::Instant;

use crate::performance::{DenseSpatialGrid, PerformanceMetrics};
use crate::world::WorldPos;

pub const BATCH_SIZE: usize = 64;

#[derive(Resource, Debug, Clone, Default)]
pub struct BatchProcessor {
    pub processed_count: usize,
    pub batch_count: usize,
    pub last_batch_time_us: u64,
}

impl BatchProcessor {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct EntityBatch {
    pub entities: Vec<Entity>,
    pub positions: Vec<WorldPos>,
    pub faction_ids: Vec<u16>,
}

impl EntityBatch {
    pub fn new() -> Self {
        Self {
            entities: Vec::with_capacity(BATCH_SIZE),
            positions: Vec::with_capacity(BATCH_SIZE),
            faction_ids: Vec::with_capacity(BATCH_SIZE),
        }
    }

    pub fn push(&mut self, entity: Entity, pos: WorldPos, faction_id: u16) {
        self.entities.push(entity);
        self.positions.push(pos);
        self.faction_ids.push(faction_id);
    }

    pub fn is_full(&self) -> bool {
        self.entities.len() >= BATCH_SIZE
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.positions.clear();
        self.faction_ids.clear();
    }
}

impl Default for EntityBatch {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct UpdateBatches {
    pub movement_batch: EntityBatch,
    pub combat_batch: EntityBatch,
    pub mining_batch: EntityBatch,
    pub building_batch: EntityBatch,
}

impl UpdateBatches {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_all(&mut self) {
        self.movement_batch.clear();
        self.combat_batch.clear();
        self.mining_batch.clear();
        self.building_batch.clear();
    }

    pub fn total_entities(&self) -> usize {
        self.movement_batch.len()
            + self.combat_batch.len()
            + self.mining_batch.len()
            + self.building_batch.len()
    }
}

pub fn process_movement_batch(
    batch: &EntityBatch,
    grid: &mut DenseSpatialGrid,
    metrics: &mut PerformanceMetrics,
) -> Vec<(Entity, WorldPos)> {
    let start = Instant::now();
    let mut results = Vec::with_capacity(batch.len());

    for i in 0..batch.len() {
        let entity = batch.entities[i];
        let pos = batch.positions[i];

        grid.update(entity, &pos);
        results.push((entity, pos));
    }

    metrics.tick_time_us += start.elapsed().as_micros() as u64;
    results
}

pub fn process_combat_batch(
    batch: &EntityBatch,
    grid: &DenseSpatialGrid,
    metrics: &mut PerformanceMetrics,
) -> Vec<(Entity, Vec<Entity>)> {
    let start = Instant::now();
    let mut results = Vec::with_capacity(batch.len());
    let attack_range = 3;

    for i in 0..batch.len() {
        let entity = batch.entities[i];
        let pos = &batch.positions[i];
        let faction = batch.faction_ids[i];

        let nearby = grid.get_in_radius_fast(pos, attack_range);
        let enemies: Vec<Entity> = nearby
            .into_iter()
            .filter(|&e| {
                if let Some(&f) = batch
                    .faction_ids
                    .get(batch.entities.iter().position(|&x| x == e).unwrap_or(0))
                {
                    f != faction
                } else {
                    false
                }
            })
            .collect();

        results.push((entity, enemies));
        metrics.spatial_queries += 1;
    }

    metrics.tick_time_us += start.elapsed().as_micros() as u64;
    results
}

pub fn process_mining_batch(
    batch: &EntityBatch,
    metrics: &mut PerformanceMetrics,
) -> Vec<(Entity, bool)> {
    let start = Instant::now();
    let results: Vec<(Entity, bool)> = batch
        .entities
        .iter()
        .enumerate()
        .map(|(i, &entity)| (entity, i % 3 == 0))
        .collect();

    metrics.tick_time_us += start.elapsed().as_micros() as u64;
    results
}

#[derive(Resource, Debug, Clone)]
pub struct TickBudget {
    pub max_time_us: u64,
    pub lua_budget_us: u64,
    pub pathfind_budget_us: u64,
    pub sim_budget_us: u64,
    pub used_lua_us: u64,
    pub used_pathfind_us: u64,
    pub used_sim_us: u64,
}

impl Default for TickBudget {
    fn default() -> Self {
        Self {
            max_time_us: 16_667,
            lua_budget_us: 10_000,
            pathfind_budget_us: 3_000,
            sim_budget_us: 3_000,
            used_lua_us: 0,
            used_pathfind_us: 0,
            used_sim_us: 0,
        }
    }
}

impl TickBudget {
    pub fn can_run_lua(&self) -> bool {
        self.used_lua_us < self.lua_budget_us
    }

    pub fn can_run_pathfind(&self) -> bool {
        self.used_pathfind_us < self.pathfind_budget_us
    }

    pub fn can_run_sim(&self) -> bool {
        self.used_sim_us < self.sim_budget_us
    }

    pub fn remaining_lua(&self) -> u64 {
        self.lua_budget_us.saturating_sub(self.used_lua_us)
    }

    pub fn remaining_pathfind(&self) -> u64 {
        self.pathfind_budget_us
            .saturating_sub(self.used_pathfind_us)
    }

    pub fn remaining_sim(&self) -> u64 {
        self.sim_budget_us.saturating_sub(self.used_sim_us)
    }

    pub fn reset(&mut self) {
        self.used_lua_us = 0;
        self.used_pathfind_us = 0;
        self.used_sim_us = 0;
    }
}

pub struct BatchProcessingPlugin;

impl Plugin for BatchProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BatchProcessor>()
            .init_resource::<UpdateBatches>()
            .init_resource::<TickBudget>();
    }
}
