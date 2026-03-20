mod ai_registry;
mod diplomacy;
mod faction;
mod identity;
mod memory;
mod spawn_logic;

pub use ai_registry::*;
pub use diplomacy::*;
pub use faction::*;
pub use identity::*;
pub use memory::*;
pub use spawn_logic::*;

use bevy::prelude::*;

pub struct FactionsPlugin;

impl Plugin for FactionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactionRegistry>()
            .init_resource::<DiplomacyState>()
            .init_resource::<FactionIdentities>()
            .add_event::<FactionEvent>()
            .add_systems(Startup, initialize_factions);
    }
}

fn initialize_factions(
    mut registry: ResMut<FactionRegistry>,
    mut identities: ResMut<FactionIdentities>,
    config: Res<crate::config::GameConfig>,
) {
    let count = config.ai_count;
    registry.initialize_default_factions(count);
    identities.initialize(count);
    tracing::info!("Initialized {} factions", registry.count());
}

#[derive(Event, Debug, Clone)]
pub enum FactionEvent {
    FactionCreated { faction_id: FactionId },
    FactionDestroyed { faction_id: FactionId },
}
