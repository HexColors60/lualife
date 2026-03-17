use std::collections::HashMap;

use crate::world::RoomCoord;

#[derive(Debug, Clone)]
pub struct RoomGraph {
    edges: HashMap<RoomCoord, Vec<RoomCoord>>,
}

impl RoomGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: RoomCoord, to: RoomCoord) {
        self.edges.entry(from).or_default().push(to);
    }

    pub fn get_neighbors(&self, room: RoomCoord) -> Option<&Vec<RoomCoord>> {
        self.edges.get(&room)
    }

    pub fn build_full_graph() -> Self {
        let mut graph = Self::new();

        for y in 0..crate::consts::ROOM_GRID_Y {
            for x in 0..crate::consts::ROOM_GRID_X {
                let coord = RoomCoord::new(x, y);

                // Add edges to adjacent rooms
                if x > 0 {
                    graph.add_edge(coord, RoomCoord::new(x - 1, y));
                }
                if x < crate::consts::ROOM_GRID_X - 1 {
                    graph.add_edge(coord, RoomCoord::new(x + 1, y));
                }
                if y > 0 {
                    graph.add_edge(coord, RoomCoord::new(x, y - 1));
                }
                if y < crate::consts::ROOM_GRID_Y - 1 {
                    graph.add_edge(coord, RoomCoord::new(x, y + 1));
                }
            }
        }

        graph
    }
}