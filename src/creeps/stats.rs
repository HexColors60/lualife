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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creep_body_new() {
        let body = CreepBody::new();
        assert!(body.parts.is_empty());
    }

    #[test]
    fn test_creep_body_add_part() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        assert_eq!(body.parts.len(), 1);
        assert_eq!(body.parts[0], PartType::Move);
    }

    #[test]
    fn test_creep_body_part_count() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Move);
        body.add_part(PartType::Fight);
        assert_eq!(body.part_count(PartType::Move), 2);
        assert_eq!(body.part_count(PartType::Fight), 1);
        assert_eq!(body.part_count(PartType::Work), 0);
    }

    #[test]
    fn test_creep_body_convenience_methods() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Work);
        body.add_part(PartType::Fight);
        body.add_part(PartType::Mine);
        body.add_part(PartType::Build);
        body.add_part(PartType::Eat);
        body.add_part(PartType::Transport);

        assert_eq!(body.move_parts(), 1);
        assert_eq!(body.work_parts(), 1);
        assert_eq!(body.fight_parts(), 1);
        assert_eq!(body.mine_parts(), 1);
        assert_eq!(body.build_parts(), 1);
        assert_eq!(body.eat_parts(), 1);
        assert_eq!(body.transport_parts(), 1);
    }

    #[test]
    fn test_creep_body_default_harvester() {
        let body = CreepBody::default_harvester();
        assert_eq!(body.parts.len(), 6);
        assert_eq!(body.move_parts(), 2);
        assert_eq!(body.mine_parts(), 2);
        assert_eq!(body.transport_parts(), 1);
        assert_eq!(body.eat_parts(), 1);
    }

    #[test]
    fn test_creep_body_default_builder() {
        let body = CreepBody::default_builder();
        assert_eq!(body.parts.len(), 6);
        assert_eq!(body.move_parts(), 2);
        assert_eq!(body.build_parts(), 2);
        assert_eq!(body.work_parts(), 1);
        assert_eq!(body.eat_parts(), 1);
    }

    #[test]
    fn test_creep_body_default_fighter() {
        let body = CreepBody::default_fighter();
        assert_eq!(body.parts.len(), 6);
        assert_eq!(body.move_parts(), 2);
        assert_eq!(body.fight_parts(), 3);
        assert_eq!(body.eat_parts(), 1);
    }

    #[test]
    fn test_creep_body_max_hp() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Work);
        assert!((body.max_hp() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_creep_body_max_power() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Eat);
        body.add_part(PartType::Eat);
        assert!((body.max_power() - 250.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_creep_body_carry_capacity() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Transport);
        body.add_part(PartType::Transport);
        assert_eq!(body.carry_capacity(), 100);
    }

    #[test]
    fn test_creep_body_speed() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Move);
        body.add_part(PartType::Move);
        body.add_part(PartType::Work);
        assert!((body.speed() - 0.6666667).abs() < 0.001);
    }

    #[test]
    fn test_creep_body_speed_no_move_parts() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Work);
        assert_eq!(body.speed(), 0.0);
    }

    #[test]
    fn test_creep_body_mining_efficiency() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Mine);
        body.add_part(PartType::Mine);
        assert!((body.mining_efficiency() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_creep_body_build_efficiency() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Build);
        body.add_part(PartType::Build);
        body.add_part(PartType::Build);
        assert!((body.build_efficiency() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_creep_body_attack_power() {
        let mut body = CreepBody::new();
        body.add_part(PartType::Fight);
        body.add_part(PartType::Fight);
        assert!((body.attack_power() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_part_type_equality() {
        assert_eq!(PartType::Move, PartType::Move);
        assert_ne!(PartType::Move, PartType::Work);
    }
}
