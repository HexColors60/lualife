use bevy::prelude::*;

/// Simulation phase ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimPhase {
    Input,
    ScriptSchedule,
    LuaExecution,
    CommandValidation,
    Movement,
    Mining,
    Transport,
    Building,
    Combat,
    Upkeep,
    Economy,
    DeathCleanup,
    EventFlush,
    UiRefresh,
}

impl SimPhase {
    pub fn order() -> Vec<Self> {
        vec![
            SimPhase::Input,
            SimPhase::ScriptSchedule,
            SimPhase::LuaExecution,
            SimPhase::CommandValidation,
            SimPhase::Movement,
            SimPhase::Mining,
            SimPhase::Transport,
            SimPhase::Building,
            SimPhase::Combat,
            SimPhase::Upkeep,
            SimPhase::Economy,
            SimPhase::DeathCleanup,
            SimPhase::EventFlush,
            SimPhase::UiRefresh,
        ]
    }
}