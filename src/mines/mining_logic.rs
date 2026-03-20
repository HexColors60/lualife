use crate::creeps::Creep;
use crate::mines::MineNode;
pub struct MiningLogic;

impl MiningLogic {
    pub fn calculate_extraction_rate(creep: &Creep, mine: &MineNode) -> u32 {
        let base_rate = mine.mine_type.base_extraction_rate;
        let creep_efficiency = creep.body.mining_efficiency();

        (base_rate as f32 * creep_efficiency) as u32
    }

    pub fn can_mine(creep: &Creep, mine: &MineNode) -> bool {
        // Check if creep can carry more
        if creep.inventory.is_full() {
            return false;
        }

        // Check if mine has resources
        if !mine.can_extract() {
            return false;
        }

        // Check if creep is close enough (should be at same position or adjacent)
        true
    }

    pub fn perform_mining(creep: &mut Creep, mine: &mut MineNode) -> u32 {
        if !Self::can_mine(creep, mine) {
            return 0;
        }

        let rate = Self::calculate_extraction_rate(creep, mine);
        let available_space = creep.inventory.available_capacity();
        let to_extract = rate.min(available_space);

        let extracted = mine.extract(to_extract);

        if extracted > 0 {
            creep.inventory.add(mine.resource_type(), extracted);
        }

        extracted
    }
}
