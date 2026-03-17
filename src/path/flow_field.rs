use std::collections::HashMap;

use crate::world::WorldPos;

#[derive(Debug, Clone)]
pub struct FlowField {
    directions: HashMap<(i32, i32), (i8, i8)>,
}

impl FlowField {
    pub fn new() -> Self {
        Self {
            directions: HashMap::new(),
        }
    }

    pub fn get_direction(&self, pos: WorldPos) -> Option<(i8, i8)> {
        self.directions.get(&(pos.x, pos.y)).copied()
    }

    pub fn set_direction(&mut self, pos: WorldPos, dx: i8, dy: i8) {
        self.directions.insert((pos.x, pos.y), (dx, dy));
    }

    pub fn clear(&mut self) {
        self.directions.clear();
    }
}
