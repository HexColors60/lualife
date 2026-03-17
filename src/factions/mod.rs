mod ai_registry;
mod diplomacy;
mod faction;
mod memory;
mod spawn_logic;

pub use ai_registry::*;
pub use diplomacy::*;
pub use faction::*;
pub use memory::*;
pub use spawn_logic::*;

use bevy::prelude::*;

use crate::config::AiConfig;
use crate::resources::Stockpile;

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactionRegistry>()
            .init_resource::<DiplomacyState>();
        app.add_event::<FactionEvent>();
    }
}

#[derive(Event, Debug, Clone)]
pub enum FactionEvent {
    FactionCreated { faction_id: FactionId },
    FactionDestroyed { faction_id: FactionId },
}