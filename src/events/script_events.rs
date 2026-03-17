use bevy::prelude::*;

use crate::factions::FactionId;

#[derive(Event, Debug, Clone)]
pub enum ScriptEvent {
    ScriptLoaded { faction_id: FactionId, path: String },
    ScriptError { faction_id: FactionId, error: String },
    ScriptTimeout { faction_id: FactionId },
    ScriptReloaded { faction_id: FactionId },
    HookTriggered { faction_id: FactionId, hook: String },
}