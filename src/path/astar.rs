use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

use crate::world::{WorldMap, WorldPos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: WorldPos,
    f: u32,
    g: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f) // Reverse for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStar;

impl AStar {
    pub fn find_path(
        world_map: &WorldMap,
        start: WorldPos,
        goal: WorldPos,
    ) -> Option<Vec<WorldPos>> {
        if !start.is_valid() || !goal.is_valid() {
            return None;
        }

        if !world_map.is_walkable(goal) {
            return None;
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<WorldPos, WorldPos> = HashMap::new();
        let mut g_score: HashMap<WorldPos, u32> = HashMap::new();
        let mut closed_set: HashSet<WorldPos> = HashSet::new();

        g_score.insert(start, 0);
        open_set.push(Node {
            pos: start,
            f: Self::heuristic(start, goal),
            g: 0,
        });

        while let Some(current) = open_set.pop() {
            if current.pos == goal {
                return Some(Self::reconstruct_path(&came_from, current.pos));
            }

            if closed_set.contains(&current.pos) {
                continue;
            }
            closed_set.insert(current.pos);

            for neighbor in Self::get_neighbors(world_map, current.pos) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let tentative_g = current.g + 1; // Assuming uniform cost

                let is_better = g_score.get(&neighbor).map_or(true, |&g| tentative_g < g);

                if is_better {
                    came_from.insert(neighbor, current.pos);
                    g_score.insert(neighbor, tentative_g);

                    open_set.push(Node {
                        pos: neighbor,
                        f: tentative_g + Self::heuristic(neighbor, goal),
                        g: tentative_g,
                    });
                }
            }
        }

        None
    }

    fn heuristic(a: WorldPos, b: WorldPos) -> u32 {
        a.manhattan_distance(&b) as u32
    }

    fn get_neighbors(world_map: &WorldMap, pos: WorldPos) -> Vec<WorldPos> {
        let mut neighbors = Vec::new();

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = pos.x + dx;
            let new_y = pos.y + dy;
            let new_pos = WorldPos::new(new_x, new_y);

            if new_pos.is_valid() && world_map.is_walkable(new_pos) {
                neighbors.push(new_pos);
            }
        }

        neighbors
    }

    fn reconstruct_path(came_from: &HashMap<WorldPos, WorldPos>, mut current: WorldPos) -> Vec<WorldPos> {
        let mut path = vec![current];

        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}