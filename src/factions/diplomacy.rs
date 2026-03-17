use std::collections::HashMap;

use bevy::prelude::*;

use super::FactionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelationType {
    #[default]
    Neutral,
    Allied,
    Hostile,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct DiplomacyState {
    relations: HashMap<(FactionId, FactionId), RelationType>,
}

impl DiplomacyState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_relation(
        &mut self,
        faction_a: FactionId,
        faction_b: FactionId,
        relation: RelationType,
    ) {
        self.relations.insert((faction_a, faction_b), relation);
        self.relations.insert((faction_b, faction_a), relation);
    }

    pub fn get_relation(&self, faction_a: FactionId, faction_b: FactionId) -> RelationType {
        if faction_a == faction_b {
            return RelationType::Allied;
        }
        self.relations
            .get(&(faction_a, faction_b))
            .copied()
            .unwrap_or(RelationType::Neutral)
    }

    pub fn is_hostile(&self, faction_a: FactionId, faction_b: FactionId) -> bool {
        self.get_relation(faction_a, faction_b) == RelationType::Hostile
    }

    pub fn is_allied(&self, faction_a: FactionId, faction_b: FactionId) -> bool {
        self.get_relation(faction_a, faction_b) == RelationType::Allied
    }

    pub fn set_all_hostile(&mut self, factions: &[FactionId]) {
        for i in 0..factions.len() {
            for j in (i + 1)..factions.len() {
                self.set_relation(factions[i], factions[j], RelationType::Hostile);
            }
        }
    }
}
