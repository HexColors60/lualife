use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    Paused,
    SingleStep,
    GameOver,
}

impl GameState {
    pub fn is_running(&self) -> bool {
        matches!(self, GameState::Running)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, GameState::Paused)
    }

    pub fn toggle(&mut self) {
        *self = match self {
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
            _ => GameState::Running,
        };
    }
}
