use bevy::prelude::*;
use std::collections::HashMap;

use crate::consts::{WORLD_TILES_X, WORLD_TILES_Y};
use crate::world::WorldPos;

pub mod batch_processing;
pub mod profiling;

pub use batch_processing::*;
pub use profiling::*;

pub const CELL_SIZE: u32 = 16;
pub const GRID_WIDTH: u32 = WORLD_TILES_X / CELL_SIZE;
pub const GRID_HEIGHT: u32 = WORLD_TILES_Y / CELL_SIZE;

#[derive(Debug, Clone, Resource)]
pub struct DenseSpatialGrid {
    cells: Box<[Vec<Entity>]>,
    entity_cells: HashMap<Entity, (i32, i32)>,
    width: usize,
    height: usize,
}

impl Default for DenseSpatialGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl DenseSpatialGrid {
    pub fn new() -> Self {
        let total_cells = (GRID_WIDTH * GRID_HEIGHT) as usize;
        Self {
            cells: vec![Vec::new(); total_cells].into_boxed_slice(),
            entity_cells: HashMap::with_capacity(1000),
            width: GRID_WIDTH as usize,
            height: GRID_HEIGHT as usize,
        }
    }

    #[inline]
    fn cell_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    #[inline]
    pub fn pos_to_cell(pos: &WorldPos) -> (i32, i32) {
        let x = (pos.x as u32 / CELL_SIZE).min(GRID_WIDTH - 1) as i32;
        let y = (pos.y as u32 / CELL_SIZE).min(GRID_HEIGHT - 1) as i32;
        (x, y)
    }

    #[inline]
    pub fn insert(&mut self, entity: Entity, pos: &WorldPos) {
        let new_cell = Self::pos_to_cell(pos);

        if let Some(&old_cell) = self.entity_cells.get(&entity) {
            if old_cell == new_cell {
                return;
            }
            if let Some(idx) = self.cell_index(old_cell.0, old_cell.1) {
                self.cells[idx].retain(|&e| e != entity);
            }
        }

        if let Some(idx) = self.cell_index(new_cell.0, new_cell.1) {
            self.cells[idx].push(entity);
        }
        self.entity_cells.insert(entity, new_cell);
    }

    #[inline]
    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_cells.remove(&entity) {
            if let Some(idx) = self.cell_index(cell.0, cell.1) {
                self.cells[idx].retain(|&e| e != entity);
            }
        }
    }

    #[inline]
    pub fn update(&mut self, entity: Entity, pos: &WorldPos) {
        self.insert(entity, pos);
    }

    #[inline]
    pub fn get_in_cell(&self, x: i32, y: i32) -> &[Entity] {
        self.cell_index(x, y)
            .map(|idx| self.cells[idx].as_slice())
            .unwrap_or(&[])
    }

    pub fn get_in_radius(&self, pos: &WorldPos, radius: i32) -> Vec<Entity> {
        let (cx, cy) = Self::pos_to_cell(pos);
        let cell_radius = (radius as u32 / CELL_SIZE + 1) as i32;

        let mut result = Vec::new();
        let r2 = radius * radius;

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell_x = cx + dx;
                let cell_y = cy + dy;

                if let Some(idx) = self.cell_index(cell_x, cell_y) {
                    for &entity in &self.cells[idx] {
                        if let Some(&entity_cell) = self.entity_cells.get(&entity) {
                            if (entity_cell.0 - cx).pow(2) + (entity_cell.1 - cy).pow(2) <= r2 {
                                result.push(entity);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn get_in_radius_fast(&self, pos: &WorldPos, radius: i32) -> SmallEntityList {
        let (cx, cy) = Self::pos_to_cell(pos);
        let cell_radius = (radius as u32 / CELL_SIZE + 1) as i32;

        let mut result = SmallEntityList::new();

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                if let Some(idx) = self.cell_index(cx + dx, cy + dy) {
                    result.extend(self.cells[idx].iter().copied());
                }
            }
        }

        result
    }

    pub fn get_in_rect(&self, min_pos: &WorldPos, max_pos: &WorldPos) -> Vec<Entity> {
        let (min_x, min_y) = Self::pos_to_cell(min_pos);
        let (max_x, max_y) = Self::pos_to_cell(max_pos);

        let mut result = Vec::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(idx) = self.cell_index(x, y) {
                    result.extend(self.cells[idx].iter().copied());
                }
            }
        }

        result
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
        self.entity_cells.clear();
    }

    pub fn entity_count(&self) -> usize {
        self.entity_cells.len()
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn average_cell_occupancy(&self) -> f32 {
        let total: usize = self.cells.iter().map(|c| c.len()).sum();
        let non_empty = self.cells.iter().filter(|c| !c.is_empty()).count();
        if non_empty > 0 {
            total as f32 / non_empty as f32
        } else {
            0.0
        }
    }
}

use smallvec::SmallVec;

pub type SmallEntityList = SmallVec<[Entity; 16]>;

#[derive(Debug, Clone, Resource, Default)]
pub struct SpatialGrid {
    cells: HashMap<(i32, i32), Vec<Entity>>,
    entity_cells: HashMap<Entity, (i32, i32)>,
}

impl SpatialGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pos_to_cell(pos: &WorldPos) -> (i32, i32) {
        let x = (pos.x as u32 / CELL_SIZE).min(GRID_WIDTH - 1) as i32;
        let y = (pos.y as u32 / CELL_SIZE).min(GRID_HEIGHT - 1) as i32;
        (x, y)
    }

    pub fn insert(&mut self, entity: Entity, pos: &WorldPos) {
        let cell = Self::pos_to_cell(pos);

        if let Some(&old_cell) = self.entity_cells.get(&entity) {
            if old_cell != cell {
                if let Some(entities) = self.cells.get_mut(&old_cell) {
                    entities.retain(|&e| e != entity);
                }
            }
        }

        self.cells.entry(cell).or_default().push(entity);
        self.entity_cells.insert(entity, cell);
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_cells.remove(&entity) {
            if let Some(entities) = self.cells.get_mut(&cell) {
                entities.retain(|&e| e != entity);
            }
        }
    }

    pub fn update(&mut self, entity: Entity, pos: &WorldPos) {
        self.insert(entity, pos);
    }

    pub fn get_in_cell(&self, cell: (i32, i32)) -> Option<&Vec<Entity>> {
        self.cells.get(&cell)
    }

    pub fn get_in_radius(&self, pos: &WorldPos, radius: i32) -> Vec<Entity> {
        let center_cell = Self::pos_to_cell(pos);
        let cell_radius = (radius as u32 / CELL_SIZE + 1) as i32;

        let mut result = Vec::new();

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if cell.0 >= 0
                    && cell.0 < GRID_WIDTH as i32
                    && cell.1 >= 0
                    && cell.1 < GRID_HEIGHT as i32
                {
                    if let Some(entities) = self.cells.get(&cell) {
                        result.extend(entities.iter().copied());
                    }
                }
            }
        }

        result
    }

    pub fn get_in_rect(&self, min_pos: &WorldPos, max_pos: &WorldPos) -> Vec<Entity> {
        let min_cell = Self::pos_to_cell(min_pos);
        let max_cell = Self::pos_to_cell(max_pos);

        let mut result = Vec::new();

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    result.extend(entities.iter().copied());
                }
            }
        }

        result
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_cells.clear();
    }

    pub fn entity_count(&self) -> usize {
        self.entity_cells.len()
    }
}

#[derive(Debug, Clone, Resource)]
pub struct ObjectPool<T: Clone> {
    available: Vec<T>,
    in_use: Vec<T>,
    create_fn: fn() -> T,
    reset_fn: fn(&mut T),
}

impl<T: Clone> ObjectPool<T> {
    pub fn new(create_fn: fn() -> T, reset_fn: fn(&mut T), initial_size: usize) -> Self {
        let mut available = Vec::with_capacity(initial_size);
        for _ in 0..initial_size {
            available.push(create_fn());
        }

        Self {
            available,
            in_use: Vec::new(),
            create_fn,
            reset_fn,
        }
    }

    pub fn acquire(&mut self) -> T {
        if let Some(mut obj) = self.available.pop() {
            (self.reset_fn)(&mut obj);
            let result = obj.clone();
            self.in_use.push(obj);
            result
        } else {
            let obj = (self.create_fn)();
            let result = obj.clone();
            self.in_use.push(obj);
            result
        }
    }

    pub fn release(&mut self, obj: T) {
        self.in_use
            .retain(|o| std::ptr::addr_of!(*o) != std::ptr::addr_of!(obj));
        self.available.push(obj);
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    pub fn in_use_count(&self) -> usize {
        self.in_use.len()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PooledCreepData {
    pub position: WorldPos,
    pub faction_id: u16,
    pub hp: u32,
    pub max_hp: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PooledBuildingData {
    pub position: WorldPos,
    pub building_type: u8,
    pub faction_id: u16,
    pub hp: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Resource, Default)]
pub struct PerformanceMetrics {
    pub frame_time_ms: f32,
    pub update_time_ms: f32,
    pub render_time_ms: f32,
    pub entity_count: usize,
    pub spatial_queries: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
    pub tick_time_us: u64,
    pub lua_time_us: u64,
    pub pathfind_time_us: u64,
}

impl PerformanceMetrics {
    pub fn cache_hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total > 0 {
            self.cache_hits as f32 / total as f32
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct HierarchicalPathMap {
    pub chunk_size: i32,
    pub chunks: HashMap<(i32, i32), PathChunk>,
    pub chunk_graph: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

#[derive(Debug, Clone, Default)]
pub struct PathChunk {
    pub traversable: bool,
    pub entry_points: Vec<WorldPos>,
    pub exit_points: Vec<WorldPos>,
}

impl HierarchicalPathMap {
    pub fn new(chunk_size: i32) -> Self {
        Self {
            chunk_size,
            chunks: HashMap::new(),
            chunk_graph: HashMap::new(),
        }
    }

    pub fn pos_to_chunk(&self, pos: &WorldPos) -> (i32, i32) {
        let x = pos.x / self.chunk_size;
        let y = pos.y / self.chunk_size;
        (x, y)
    }

    pub fn get_chunk(&self, pos: &WorldPos) -> Option<&PathChunk> {
        let chunk = self.pos_to_chunk(pos);
        self.chunks.get(&chunk)
    }

    pub fn is_chunk_traversable(&self, chunk: (i32, i32)) -> bool {
        self.chunks
            .get(&chunk)
            .map(|c| c.traversable)
            .unwrap_or(false)
    }

    pub fn find_path_chunks(&self, start: &WorldPos, end: &WorldPos) -> Vec<(i32, i32)> {
        let start_chunk = self.pos_to_chunk(start);
        let end_chunk = self.pos_to_chunk(end);

        if start_chunk == end_chunk {
            return vec![start_chunk];
        }

        let mut visited = vec![start_chunk];
        let mut queue = vec![(start_chunk, vec![start_chunk])];

        while let Some((current, path)) = queue.pop() {
            if let Some(neighbors) = self.chunk_graph.get(&current) {
                for &neighbor in neighbors {
                    if neighbor == end_chunk {
                        let mut result = path.clone();
                        result.push(neighbor);
                        return result;
                    }
                    if !visited.contains(&neighbor) {
                        visited.push(neighbor);
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        queue.push((neighbor, new_path));
                    }
                }
            }
        }

        vec![]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LodLevel {
    High,
    Medium,
    Low,
    Culled,
}

impl LodLevel {
    pub fn from_distance(distance: f32) -> Self {
        if distance < 100.0 {
            LodLevel::High
        } else if distance < 300.0 {
            LodLevel::Medium
        } else if distance < 500.0 {
            LodLevel::Low
        } else {
            LodLevel::Culled
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct ChunkStreamingState {
    pub loaded_chunks: Vec<(i32, i32)>,
    pub view_distance: i32,
    pub unload_distance: i32,
}

impl ChunkStreamingState {
    pub fn new() -> Self {
        Self {
            loaded_chunks: Vec::new(),
            view_distance: 3,
            unload_distance: 5,
        }
    }

    pub fn get_visible_chunks(&self, center: &WorldPos, chunk_size: i32) -> Vec<(i32, i32)> {
        let cx = center.x / chunk_size;
        let cy = center.y / chunk_size;

        let mut chunks = Vec::new();
        for dx in -self.view_distance..=self.view_distance {
            for dy in -self.view_distance..=self.view_distance {
                chunks.push((cx + dx, cy + dy));
            }
        }
        chunks
    }

    pub fn should_unload(&self, chunk: (i32, i32), center: &WorldPos, chunk_size: i32) -> bool {
        let cx = center.x / chunk_size;
        let cy = center.y / chunk_size;

        let dx = (chunk.0 - cx).abs();
        let dy = (chunk.1 - cy).abs();

        dx > self.unload_distance || dy > self.unload_distance
    }
}

pub fn spatial_grid_update_system(_grid: ResMut<SpatialGrid>) {}

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialGrid>()
            .init_resource::<DenseSpatialGrid>()
            .init_resource::<PerformanceMetrics>()
            .init_resource::<ChunkStreamingState>()
            .init_resource::<Profiler>()
            .init_resource::<BatchProcessor>()
            .init_resource::<UpdateBatches>()
            .init_resource::<TickBudget>()
            .add_systems(First, profiler_frame_system)
            .add_systems(Last, profiler_report_system);
    }
}
