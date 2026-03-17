use indexmap::IndexSet;

use crate::config::WorldgenConfig;
use crate::core::GameRng;
use crate::consts::ROOM_TILE_SIZE;
use crate::resources::ResourceType;

#[derive(Debug, Clone)]
pub struct MinePlacement {
    pub local_x: usize,
    pub local_y: usize,
    pub resource_type: ResourceType,
    pub initial_amount: u32,
    pub max_amount: u32,
}

pub struct MineDistributor;

impl MineDistributor {
    pub fn assign_room_mines(
        room_x: u32,
        room_y: u32,
        config: &WorldgenConfig,
        rng: &mut GameRng,
    ) -> Vec<MinePlacement> {
        let mut mines = Vec::new();
        let mine_count = rng.gen_range(config.mines_per_room_min..=config.mines_per_room_max);

        let mut used_positions = IndexSet::new();

        for _ in 0..mine_count {
            // Find a valid position
            let mut attempts = 0;
            let (local_x, local_y) = loop {
                let x = rng.gen_range(0..ROOM_TILE_SIZE as usize);
                let y = rng.gen_range(0..ROOM_TILE_SIZE as usize);
                let pos = (x, y);

                if !used_positions.contains(&pos) {
                    used_positions.insert(pos);
                    break (x, y);
                }

                attempts += 1;
                if attempts > 100 {
                    break (0, 0);
                }
            };

            let resource_type = Self::choose_mine_type(room_x, room_y, config, rng);
            let max_amount = rng.gen_range(1000..5000);
            let initial_amount = (max_amount as f32 * rng.gen_range(0.5..1.0)) as u32;

            mines.push(MinePlacement {
                local_x,
                local_y,
                resource_type,
                initial_amount,
                max_amount,
            });
        }

        mines
    }

    fn choose_mine_type(
        _room_x: u32,
        _room_y: u32,
        config: &WorldgenConfig,
        rng: &mut GameRng,
    ) -> ResourceType {
        // Power mines are rarer
        let roll = rng.gen_range(0.0..1.0);

        if roll < config.power_mine_rarity {
            // Check if this room should have a power mine
            let power_roll = rng.gen_range(0.0..1.0);
            if power_roll < 0.3 {
                return ResourceType::Power;
            }
        }

        // Choose from other resource types
        let types = [
            ResourceType::Iron,
            ResourceType::Copper,
            ResourceType::Silicon,
            ResourceType::Crystal,
            ResourceType::Carbon,
            ResourceType::Stone,
            ResourceType::Sulfur,
            ResourceType::Water,
            ResourceType::Biomass,
        ];

        let index = rng.gen_range(0..types.len());
        types[index].clone()
    }
}