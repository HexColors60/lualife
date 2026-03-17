use bevy::prelude::*;

use super::{ResourceType, Stockpile};
use crate::factions::FactionId;

pub struct EconomySystem;

impl EconomySystem {
    pub fn transfer(
        from: &mut Stockpile,
        to: &mut Stockpile,
        resource_type: ResourceType,
        amount: u32,
    ) -> u32 {
        let actual = from.remove(resource_type, amount);
        to.add(resource_type, actual);
        actual
    }

    pub fn can_afford(stockpile: &Stockpile, costs: &[(ResourceType, u32)]) -> bool {
        costs.iter().all(|(resource, amount)| stockpile.has(*resource, *amount))
    }

    pub fn spend(stockpile: &mut Stockpile, costs: &[(ResourceType, u32)]) -> bool {
        if !Self::can_afford(stockpile, costs) {
            return false;
        }

        for (resource, amount) in costs {
            stockpile.remove(*resource, *amount);
        }
        true
    }
}