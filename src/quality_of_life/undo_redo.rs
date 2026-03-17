use bevy::prelude::*;
use std::collections::VecDeque;

/// Maximum undo history size
pub const MAX_UNDO_HISTORY: usize = 50;

/// Undo/Redo state resource
#[derive(Debug, Clone, Resource)]
pub struct UndoRedoState {
    pub undo_stack: VecDeque<UndoAction>,
    pub redo_stack: VecDeque<UndoAction>,
    pub max_history: usize,
}

impl Default for UndoRedoState {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(MAX_UNDO_HISTORY),
            redo_stack: VecDeque::with_capacity(MAX_UNDO_HISTORY),
            max_history: MAX_UNDO_HISTORY,
        }
    }
}

/// Undo/Redo event
#[derive(Debug, Clone, Event)]
pub enum UndoRedoEvent {
    PushAction(UndoAction),
    Undo,
    Redo,
    Clear,
}

/// Action that can be undone
#[derive(Debug, Clone)]
pub enum UndoAction {
    BuildingPlaced {
        entity: Entity,
        building_type: u8,
        position: (i32, i32),
        faction_id: u16,
    },
    BuildingRemoved {
        building_type: u8,
        position: (i32, i32),
        faction_id: u16,
        hp: u32,
    },
    BuildingMoved {
        entity: Entity,
        from: (i32, i32),
        to: (i32, i32),
    },
    UnitCommanded {
        unit: Entity,
        old_command: Option<UnitCommand>,
        new_command: UnitCommand,
    },
    ResourceTransferred {
        from: Entity,
        to: Entity,
        resource_type: u8,
        amount: u32,
    },
    TechResearched {
        tech_id: u32,
        faction_id: u16,
    },
    MultipleActions(Vec<UndoAction>),
}

/// Unit command for undo
#[derive(Debug, Clone)]
pub struct UnitCommand {
    pub command_type: UnitCommandType,
    pub target: Option<Entity>,
    pub position: Option<(i32, i32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitCommandType {
    Move,
    Attack,
    Mine,
    Build,
    Transfer,
    Repair,
    Idle,
}

impl UndoRedoState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an action to the undo stack
    pub fn push(&mut self, action: UndoAction) {
        // Clear redo stack when new action is pushed
        self.redo_stack.clear();

        // Remove oldest if at capacity
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.pop_front();
        }

        self.undo_stack.push_back(action);
    }

    /// Undo the last action
    pub fn undo(&mut self) -> Option<UndoAction> {
        if let Some(action) = self.undo_stack.pop_back() {
            self.redo_stack.push_back(action.clone());
            Some(action)
        } else {
            None
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self) -> Option<UndoAction> {
        if let Some(action) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(action.clone());
            Some(action)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get undo history count
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo history count
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

/// System to handle undo/redo
pub fn undo_redo_system(
    mut state: ResMut<UndoRedoState>,
    mut events: EventReader<UndoRedoEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Handle events
    for event in events.read() {
        match event {
            UndoRedoEvent::PushAction(action) => {
                state.push(action.clone());
            }
            UndoRedoEvent::Undo => {
                if let Some(action) = state.undo() {
                    apply_undo(&action);
                }
            }
            UndoRedoEvent::Redo => {
                if let Some(action) = state.redo() {
                    apply_redo(&action);
                }
            }
            UndoRedoEvent::Clear => {
                state.clear();
            }
        }
    }

    // Keyboard shortcuts (Ctrl+Z for undo, Ctrl+Y or Ctrl+Shift+Z for redo)
    let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    if ctrl && keyboard.just_pressed(KeyCode::KeyZ) {
        if shift {
            // Ctrl+Shift+Z = Redo
            if let Some(action) = state.redo() {
                apply_redo(&action);
            }
        } else {
            // Ctrl+Z = Undo
            if let Some(action) = state.undo() {
                apply_undo(&action);
            }
        }
    }

    if ctrl && keyboard.just_pressed(KeyCode::KeyY) {
        // Ctrl+Y = Redo
        if let Some(action) = state.redo() {
            apply_redo(&action);
        }
    }
}

/// Apply undo action
fn apply_undo(action: &UndoAction) {
    match action {
        UndoAction::BuildingPlaced { entity, .. } => {
            tracing::info!("Undo: Removing building {:?}", entity);
            // In real implementation, would remove the building
        }
        UndoAction::BuildingRemoved { building_type, position, faction_id, hp } => {
            tracing::info!("Undo: Restoring building at {:?}", position);
            // In real implementation, would restore the building
        }
        UndoAction::BuildingMoved { entity, from, .. } => {
            tracing::info!("Undo: Moving building {:?} back to {:?}", entity, from);
        }
        UndoAction::UnitCommanded { unit, old_command, .. } => {
            tracing::info!("Undo: Restoring unit {:?} command to {:?}", unit, old_command);
        }
        UndoAction::ResourceTransferred { from, to, resource_type, amount } => {
            tracing::info!("Undo: Transferring {} of resource {} back from {:?} to {:?}", 
                amount, resource_type, to, from);
        }
        UndoAction::TechResearched { tech_id, faction_id } => {
            tracing::info!("Undo: Unresearching tech {} for faction {}", tech_id, faction_id);
        }
        UndoAction::MultipleActions(actions) => {
            for action in actions.iter().rev() {
                apply_undo(action);
            }
        }
    }
}

/// Apply redo action
fn apply_redo(action: &UndoAction) {
    match action {
        UndoAction::BuildingPlaced { entity, building_type, position, faction_id } => {
            tracing::info!("Redo: Placing building {} at {:?} for faction {}", 
                building_type, position, faction_id);
        }
        UndoAction::BuildingRemoved { position, .. } => {
            tracing::info!("Redo: Removing building at {:?}", position);
        }
        UndoAction::BuildingMoved { entity, to, .. } => {
            tracing::info!("Redo: Moving building {:?} to {:?}", entity, to);
        }
        UndoAction::UnitCommanded { unit, new_command, .. } => {
            tracing::info!("Redo: Setting unit {:?} command to {:?}", unit, new_command);
        }
        UndoAction::ResourceTransferred { from, to, resource_type, amount } => {
            tracing::info!("Redo: Transferring {} of resource {} from {:?} to {:?}", 
                amount, resource_type, from, to);
        }
        UndoAction::TechResearched { tech_id, faction_id } => {
            tracing::info!("Redo: Researching tech {} for faction {}", tech_id, faction_id);
        }
        UndoAction::MultipleActions(actions) => {
            for action in actions {
                apply_redo(action);
            }
        }
    }
}