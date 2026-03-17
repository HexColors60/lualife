use thiserror::Error;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Lua error: {0}")]
    Lua(String),

    #[error("World generation error: {0}")]
    WorldGen(String),

    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Faction not found: {0}")]
    FactionNotFound(u16),

    #[error("Room not found at ({0}, {1})")]
    RoomNotFound(u32, u32),

    #[error("Mine not found: {0}")]
    MineNotFound(u32),

    #[error("Creep not found: {0}")]
    CreepNotFound(u32),

    #[error("Building not found: {0}")]
    BuildingNotFound(u32),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Resource error: {0}")]
    Resource(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] ron::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Script budget exceeded for faction {0}")]
    ScriptBudgetExceeded(u16),

    #[error("Script error in faction {faction}: {message}")]
    ScriptError { faction: u16, message: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<mlua::Error> for GameError {
    fn from(err: mlua::Error) -> Self {
        GameError::Lua(err.to_string())
    }
}