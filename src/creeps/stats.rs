use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartType {
    Move,
    Work,
    Fight,
    Mine,
    Build,
    Eat,
    Transport,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreepBody {
    pub parts: SmallVec<[PartType; 16]>,
}

impl CreepBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_part(&mut self, part: PartType) {
        self.parts.push(part);
    }

    pub fn part_count(&self, part_type: PartType) -> usize {
        self.parts.iter().filter(|&&p| p == part_type).count()
    }

    pub fn move_parts(&self) -> usize {
        self.part_count(PartType::Move)
    }

    pub fn work_parts(&self) -> usize {
        self.part_count(PartType::Work)
    }

    pub fn fight_parts(&self) -> usize {
        self.part_count(PartType::Fight)
    }

    pub fn mine_parts(&self) -> usize {
        self.part_count(PartType::Mine)
    }

    pub fn build_parts(&self) -> usize {
        self.part_count(PartType::Build)
    }

    pub fn eat_parts(&self) -> usize {
        self.part_count(PartType::Eat)
    }

    pub fn transport_parts(&self) -> usize {
        self.part_count(PartType::Transport)
    }

    pub fn default_harvester() -> Self {
        let mut body = Self::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Move);
        body.add_part(PartType::Mine);
        body.add_part(PartType::Mine);
        body.add_part(PartType::Transport);
        body.add_part(PartType::Eat);
        body
    }

    /// Alias for default_harvester
    pub fn harvester() -> Self {
        Self::default_harvester()
    }

    pub fn default_builder() -> Self {
        let mut body = Self::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Move);
        body.add_part(PartType::Build);
        body.add_part(PartType::Build);
        body.add_part(PartType::Work);
        body.add_part(PartType::Eat);
        body
    }

    pub fn default_fighter() -> Self {
        let mut body = Self::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Move);
        body.add_part(PartType::Fight);
        body.add_part(PartType::Fight);
        body.add_part(PartType::Fight);
        body.add_part(PartType::Eat);
        body
    }

    pub fn max_hp(&self) -> f32 {
        self.parts.len() as f32 * 50.0
    }

    pub fn max_power(&self) -> f32 {
        self.eat_parts() as f32 * 100.0 + 50.0
    }

    pub fn carry_capacity(&self) -> u32 {
        self.transport_parts() as u32 * 50
    }

    pub fn speed(&self) -> f32 {
        let move_parts = self.move_parts();
        let total_parts = self.parts.len();

        if move_parts == 0 || total_parts == 0 {
            return 0.0;
        }

        // More move parts = faster, more total parts = slower
        1.0 * (move_parts as f32 / total_parts as f32)
    }

    pub fn mining_efficiency(&self) -> f32 {
        self.mine_parts() as f32 * 0.5 + 0.5
    }

    pub fn build_efficiency(&self) -> f32 {
        self.build_parts() as f32 * 0.5 + 0.5
    }

    pub fn attack_power(&self) -> f32 {
        self.fight_parts() as f32 * 10.0
    }
}