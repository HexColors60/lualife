//! Map editor tools for custom world creation.
//!
//! Provides in-game editing of terrain, mines, buildings, and faction spawns.

mod ui;
mod systems;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::factions::FactionId;
use crate::mines::MineType;
use crate::resources::ResourceType;
use crate::buildings::BuildingType;
use crate::world::{WorldMap, TerrainType};

pub use ui::*;
pub use systems::*;

/// Editor mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EditorMode {
    #[default]
    None,
    Terrain,
    Mines,
    Buildings,
    Factions,
    Resources,
}

impl EditorMode {
    pub fn name(&self) -> &'static str {
        match self {
            EditorMode::None => "View",
            EditorMode::Terrain => "Terrain",
            EditorMode::Mines => "Mines",
            EditorMode::Buildings => "Buildings",
            EditorMode::Factions => "Factions",
            EditorMode::Resources => "Resources",
        }
    }

    pub fn cycle_next(&self) -> Self {
        match self {
            EditorMode::None => EditorMode::Terrain,
            EditorMode::Terrain => EditorMode::Mines,
            EditorMode::Mines => EditorMode::Buildings,
            EditorMode::Buildings => EditorMode::Factions,
            EditorMode::Factions => EditorMode::Resources,
            EditorMode::Resources => EditorMode::None,
        }
    }
}

/// Editor tool selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EditorTool {
    #[default]
    Select,
    Paint,
    Erase,
    Fill,
}

impl EditorTool {
    pub fn name(&self) -> &'static str {
        match self {
            EditorTool::Select => "Select",
            EditorTool::Paint => "Paint",
            EditorTool::Erase => "Erase",
            EditorTool::Fill => "Fill",
        }
    }

    pub fn cycle_next(&self) -> Self {
        match self {
            EditorTool::Select => EditorTool::Paint,
            EditorTool::Paint => EditorTool::Erase,
            EditorTool::Erase => EditorTool::Fill,
            EditorTool::Fill => EditorTool::Select,
        }
    }
}

/// Brush size for painting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrushSize {
    Small = 1,
    Medium = 2,
    Large = 3,
    Huge = 5,
}

impl Default for BrushSize {
    fn default() -> Self {
        BrushSize::Small
    }
}

impl BrushSize {
    pub fn radius(&self) -> u32 {
        *self as u32
    }

    pub fn cycle_next(&self) -> Self {
        match self {
            BrushSize::Small => BrushSize::Medium,
            BrushSize::Medium => BrushSize::Large,
            BrushSize::Large => BrushSize::Huge,
            BrushSize::Huge => BrushSize::Small,
        }
    }
}

/// Editor state resource
#[derive(Resource, Debug, Clone, Default)]
pub struct EditorState {
    pub enabled: bool,
    pub mode: EditorMode,
    pub tool: EditorTool,
    pub brush_size: BrushSize,
    pub selected_terrain: TerrainType,
    pub selected_mine_type: MineType,
    pub selected_building_type: BuildingType,
    pub selected_faction: FactionId,
    pub selected_resource: ResourceType,
    pub cursor_position: Option<crate::world::WorldPos>,
    pub selection_start: Option<crate::world::WorldPos>,
    pub selection_end: Option<crate::world::WorldPos>,
    pub has_unsaved_changes: bool,
    pub history: Vec<EditorAction>,
    pub history_index: usize,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            mode: EditorMode::None,
            tool: EditorTool::Select,
            brush_size: BrushSize::Small,
            selected_terrain: TerrainType::Plains,
            selected_mine_type: MineType::new(ResourceType::Power),
            selected_building_type: BuildingType::BaseCore,
            selected_faction: FactionId(0),
            selected_resource: ResourceType::Power,
            cursor_position: None,
            selection_start: None,
            selection_end: None,
            has_unsaved_changes: false,
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if !self.enabled {
            self.mode = EditorMode::None;
        }
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn set_tool(&mut self, tool: EditorTool) {
        self.tool = tool;
    }

    pub fn cycle_brush_size(&mut self) {
        self.brush_size = self.brush_size.cycle_next();
    }

    pub fn add_action(&mut self, action: EditorAction) {
        // Truncate future history if we're not at the end
        self.history.truncate(self.history_index);
        self.history.push(action);
        self.history_index = self.history.len();
        self.has_unsaved_changes = true;
    }

    pub fn undo(&mut self) -> Option<&EditorAction> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.history.get(self.history_index)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&EditorAction> {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            self.history.get(self.history_index - 1)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.history_index < self.history.len()
    }
}

/// Editor action for undo/redo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorAction {
    TerrainChange {
        positions: Vec<(u32, u32, TerrainType, TerrainType)>, // x, y, new, old
    },
    MinePlaced {
        x: u32,
        y: u32,
        mine_type: MineType,
    },
    MineRemoved {
        x: u32,
        y: u32,
        mine_type: MineType,
    },
    BuildingPlaced {
        x: u32,
        y: u32,
        building_type: BuildingType,
        faction_id: FactionId,
    },
    BuildingRemoved {
        x: u32,
        y: u32,
        building_type: BuildingType,
    },
    FactionSpawnSet {
        faction_id: FactionId,
        x: u32,
        y: u32,
        old_x: Option<u32>,
        old_y: Option<u32>,
    },
}

/// Custom map data for saving/loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMapData {
    pub name: String,
    pub version: u32,
    pub world_size: (u32, u32),
    pub terrain: Vec<Vec<TerrainType>>,
    pub mines: Vec<MinePlacement>,
    pub buildings: Vec<BuildingPlacement>,
    pub faction_spawns: Vec<FactionSpawn>,
}

impl CustomMapData {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            version: 1,
            world_size: (width, height),
            terrain: vec![vec![TerrainType::Plains; height as usize]; width as usize],
            mines: Vec::new(),
            buildings: Vec::new(),
            faction_spawns: Vec::new(),
        }
    }

    pub fn from_world_map(world_map: &WorldMap, name: &str) -> Self {
        let (width, height) = (256u32, 256u32);
        let mut terrain = vec![vec![TerrainType::Plains; height as usize]; width as usize];

        // Extract terrain from world map
        for room in world_map.all_rooms() {
            for (y, row) in room.tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    let world_x = room.coord.x * 8 + x as u32;
                    let world_y = room.coord.y * 8 + y as u32;
                    if world_x < width && world_y < height {
                        terrain[world_x as usize][world_y as usize] = tile.terrain;
                    }
                }
            }
        }

        Self {
            name: name.to_string(),
            version: 1,
            world_size: (width, height),
            terrain,
            mines: Vec::new(),
            buildings: Vec::new(),
            faction_spawns: Vec::new(),
        }
    }
}

/// Mine placement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinePlacement {
    pub x: u32,
    pub y: u32,
    pub mine_type: MineType,
    pub amount: u32,
}

/// Building placement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingPlacement {
    pub x: u32,
    pub y: u32,
    pub building_type: BuildingType,
    pub faction_id: FactionId,
}

/// Faction spawn point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSpawn {
    pub faction_id: FactionId,
    pub x: u32,
    pub y: u32,
}

/// Event for editor actions
#[derive(Event, Debug, Clone)]
pub struct EditorActionEvent {
    pub action: EditorAction,
}

/// Event for map save/load
#[derive(Event, Debug, Clone)]
pub struct MapSavedEvent {
    pub path: String,
    pub name: String,
}

#[derive(Event, Debug, Clone)]
pub struct MapLoadedEvent {
    pub path: String,
    pub name: String,
}

/// System to handle editor keyboard shortcuts
pub fn editor_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<EditorState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    if !state.enabled {
        return;
    }

    // Mode switching (1-6 keys)
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.set_mode(EditorMode::Terrain);
        game_log.add(format!("Editor mode: {}", state.mode.name()));
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.set_mode(EditorMode::Mines);
        game_log.add(format!("Editor mode: {}", state.mode.name()));
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        state.set_mode(EditorMode::Buildings);
        game_log.add(format!("Editor mode: {}", state.mode.name()));
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        state.set_mode(EditorMode::Factions);
        game_log.add(format!("Editor mode: {}", state.mode.name()));
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        state.set_mode(EditorMode::Resources);
        game_log.add(format!("Editor mode: {}", state.mode.name()));
    }
    if keyboard.just_pressed(KeyCode::Digit0) {
        state.set_mode(EditorMode::None);
        game_log.add("Editor mode: View".to_string());
    }

    // Tool switching (Q, W, E, R)
    if keyboard.just_pressed(KeyCode::KeyQ) {
        state.set_tool(EditorTool::Select);
        game_log.add(format!("Tool: {}", state.tool.name()));
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        state.set_tool(EditorTool::Paint);
        game_log.add(format!("Tool: {}", state.tool.name()));
    }
    if keyboard.just_pressed(KeyCode::KeyE) {
        state.set_tool(EditorTool::Erase);
        game_log.add(format!("Tool: {}", state.tool.name()));
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        state.set_tool(EditorTool::Fill);
        game_log.add(format!("Tool: {}", state.tool.name()));
    }

    // Brush size (B)
    if keyboard.just_pressed(KeyCode::KeyB) {
        state.cycle_brush_size();
        game_log.add(format!("Brush size: {}", state.brush_size.radius()));
    }

    // Undo/Redo (Ctrl+Z, Ctrl+Y)
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        if keyboard.just_pressed(KeyCode::KeyZ) {
            if state.can_undo() {
                game_log.add("Undo".to_string());
            }
        }
        if keyboard.just_pressed(KeyCode::KeyY) {
            if state.can_redo() {
                game_log.add("Redo".to_string());
            }
        }
    }
}

/// Toggle editor mode
pub fn toggle_editor(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<EditorState>,
    mut game_log: ResMut<crate::ui::GameLog>,
) {
    // F8 toggles editor
    if keyboard.just_pressed(KeyCode::F8) {
        state.toggle();
        if state.enabled {
            game_log.add("🗺 Map Editor enabled (F8 to disable)".to_string());
        } else {
            game_log.add("Map Editor disabled".to_string());
        }
    }
}

/// Plugin for map editor
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorState::new())
            .add_event::<EditorActionEvent>()
            .add_event::<MapSavedEvent>()
            .add_event::<MapLoadedEvent>()
            .add_systems(
                Update,
                (
                    toggle_editor,
                    editor_input_system,
                ),
            );
    }
}