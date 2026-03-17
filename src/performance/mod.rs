use std::collections::HashMap;
use bevy::prelude::*;

use crate::world::WorldPos;
use crate::consts::{WORLD_TILES_X, WORLD_TILES_Y};

/// Grid cell size for spatial partitioning
pub const CELL_SIZE: u32 = 16;

/// Number of cells in each dimension
pub const GRID_WIDTH: u32 = WORLD_TILES_X / CELL_SIZE;
pub const GRID_HEIGHT: u32 = WORLD_TILES_Y / CELL_SIZE;

/// Spatial hash grid for efficient entity queries
#[derive(Debug, Clone, Resource, Default)]
pub struct SpatialGrid {
    cells: HashMap<(i32, i32), Vec<Entity>>,
    entity_cells: HashMap<Entity, (i32, i32)>,
}

impl SpatialGrid {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get cell coordinates from world position
    pub fn pos_to_cell(pos: &WorldPos) -> (i32, i32) {
        let x = (pos.x as u32 / CELL_SIZE).min(GRID_WIDTH - 1) as i32;
        let y = (pos.y as u32 / CELL_SIZE).min(GRID_HEIGHT - 1) as i32;
        (x, y)
    }

    /// Insert entity into grid
    pub fn insert(&mut self, entity: Entity, pos: &WorldPos) {
        let cell = Self::pos_to_cell(pos);
        
        // Remove from old cell if exists
        if let Some(&old_cell) = self.entity_cells.get(&entity) {
            if old_cell != cell {
                if let Some(entities) = self.cells.get_mut(&old_cell) {
                    entities.retain(|&e| e != entity);
                }
            }
        }

        // Insert into new cell
        self.cells.entry(cell).or_default().push(entity);
        self.entity_cells.insert(entity, cell);
    }

    /// Remove entity from grid
    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_cells.remove(&entity) {
            if let Some(entities) = self.cells.get_mut(&cell) {
                entities.retain(|&e| e != entity);
            }
        }
    }

    /// Update entity position
    pub fn update(&mut self, entity: Entity, pos: &WorldPos) {
        self.insert(entity, pos);
    }

    /// Get entities in a cell
    pub fn get_in_cell(&self, cell: (i32, i32)) -> Option<&Vec<Entity>> {
        self.cells.get(&cell)
    }

    /// Get entities in radius around position
    pub fn get_in_radius(&self, pos: &WorldPos, radius: i32) -> Vec<Entity> {
        let center_cell = Self::pos_to_cell(pos);
        let cell_radius = (radius as u32 / CELL_SIZE + 1) as i32;
        
        let mut result = Vec::new();
        
        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if cell.0 >= 0 && cell.0 < GRID_WIDTH as i32 && cell.1 >= 0 && cell.1 < GRID_HEIGHT as i32 {
                    if let Some(entities) = self.cells.get(&cell) {
                        result.extend(entities.iter().copied());
                    }
                }
            }
        }
        
        result
    }

    /// Get entities in rectangle
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

    /// Clear all entities
    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_cells.clear();
    }

    /// Get total entity count
    pub fn entity_count(&self) -> usize {
        self.entity_cells.len()
    }
}

/// Object pool for reusable entities
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
        self.in_use.retain(|o| std::ptr::addr_of!(*o) != std::ptr::addr_of!(obj));
        self.available.push(obj);
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    pub fn in_use_count(&self) -> usize {
        self.in_use.len()
    }
}

/// Pooled entity data for creeps
#[derive(Debug, Clone, Default)]
pub struct PooledCreepData {
    pub position: WorldPos,
    pub faction_id: u16,
    pub hp: u32,
    pub max_hp: u32,
    pub active: bool,
}

/// Pooled entity data for buildings
#[derive(Debug, Clone, Default)]
pub struct PooledBuildingData {
    pub position: WorldPos,
    pub building_type: u8,
    pub faction_id: u16,
    pub hp: u32,
    pub active: bool,
}

/// Performance metrics
#[derive(Debug, Clone, Resource, Default)]
pub struct PerformanceMetrics {
    pub frame_time_ms: f32,
    pub update_time_ms: f32,
    pub render_time_ms: f32,
    pub entity_count: usize,
    pub spatial_queries: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
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

/// Hierarchical path map for optimized pathfinding
#[derive(Debug, Clone, Resource)]
pub struct HierarchicalPathMap {
    pub chunk_size: i32,
    pub chunks: HashMap<(i32, i32), PathChunk>,
    pub chunk_graph: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

/// Path chunk for hierarchical pathfinding
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
        self.chunks.get(&chunk).map(|c| c.traversable).unwrap_or(false)
    }

    pub fn find_path_chunks(&self, start: &WorldPos, end: &WorldPos) -> Vec<(i32, i32)> {
        let start_chunk = self.pos_to_chunk(start);
        let end_chunk = self.pos_to_chunk(end);
        
        if start_chunk == end_chunk {
            return vec![start_chunk];
        }

        // Simple BFS on chunk graph
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

/// LOD level for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LodLevel {
    High,    // Full detail
    Medium,  // Reduced detail
    Low,     // Minimal detail
    Culled,  // Not rendered
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

/// System to update spatial grid
pub fn spatial_grid_update_system(
    mut grid: ResMut<SpatialGrid>,
    // Using a simple approach - this would be called manually or with specific components
) {
    // This is a placeholder - in a real implementation, you would query entities
    // with position components and update the grid
}

/// Plugin for performance systems
pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialGrid>()
            .init_resource::<PerformanceMetrics>();
    }
}