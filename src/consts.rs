//! Global constants for the game world.

/// World dimensions in tiles
pub const WORLD_TILES_X: u32 = 256;
pub const WORLD_TILES_Y: u32 = 256;

/// Room grid dimensions
pub const ROOM_GRID_X: u32 = 32;
pub const ROOM_GRID_Y: u32 = 32;

/// Tiles per room
pub const ROOM_TILE_SIZE: u32 = 8;

/// Default number of AI factions
pub const DEFAULT_AI_COUNT: usize = 32;

/// Default tick rate (ticks per second)
pub const DEFAULT_TICK_RATE: u64 = 20;

/// Default Lua instruction budget per tick per faction
pub const DEFAULT_LUA_BUDGET: usize = 1_000_000;

/// Mines per room range
pub const MINES_PER_ROOM_MIN: usize = 2;
pub const MINES_PER_ROOM_MAX: usize = 3;

/// Number of mine types
pub const MINE_TYPE_COUNT: usize = 10;

/// Creep power consumption per tick
pub const CREEP_POWER_CONSUMPTION: f32 = 0.1;

/// Creep death threshold (power reserve)
pub const CREEP_DEATH_THRESHOLD: f32 = 0.0;

/// Default creep vision range in tiles
pub const DEFAULT_CREEP_VISION_RANGE: u32 = 5;

/// Default creep movement speed (tiles per tick)
pub const DEFAULT_CREEP_SPEED: f32 = 1.0;

/// Default creep max health
pub const DEFAULT_CREEP_HEALTH: f32 = 100.0;

/// Default creep carry capacity per transport part
pub const CARRY_CAPACITY_PER_PART: u32 = 50;

/// Default mine extraction rate per tick
pub const DEFAULT_MINE_EXTRACTION_RATE: u32 = 10;

/// Default mine regeneration rate per tick
pub const DEFAULT_MINE_REGEN_RATE: f32 = 0.1;

/// AI folder path pattern
pub const AI_FOLDER_PATTERN: &str = "ai/ai_{:02}";

/// Config folder path
pub const CONFIG_FOLDER: &str = "config";

/// Default config file names
pub const GAME_CONFIG_FILE: &str = "game.ron";
pub const WORLDGEN_CONFIG_FILE: &str = "worldgen.ron";
pub const AI_CONFIG_FILE: &str = "ai.ron";
pub const UI_CONFIG_FILE: &str = "ui.ron";