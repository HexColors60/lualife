use bevy::prelude::*;
use std::collections::HashMap;

/// Unit group manager resource
#[derive(Debug, Clone, Resource, Default)]
pub struct UnitGroupManager {
    pub groups: HashMap<u8, UnitGroup>,
    pub selected_group: Option<u8>,
    pub max_groups: u8,
}

/// Unit group
#[derive(Debug, Clone)]
pub struct UnitGroup {
    pub id: u8,
    pub name: String,
    pub units: Vec<Entity>,
    pub hotkey: Option<u8>,
}

/// Unit group event
#[derive(Debug, Clone, Event)]
pub enum UnitGroupEvent {
    Create(u8, String),
    Delete(u8),
    AddUnit(u8, Entity),
    RemoveUnit(u8, Entity),
    SelectGroup(u8),
    ClearSelection,
    Rename(u8, String),
    AssignHotkey(u8, u8),
}

impl UnitGroupManager {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            selected_group: None,
            max_groups: 10,
        }
    }

    /// Create a new group
    pub fn create(&mut self, id: u8, name: String) -> bool {
        if self.groups.contains_key(&id) || id >= self.max_groups {
            return false;
        }
        self.groups.insert(id, UnitGroup {
            id,
            name,
            units: Vec::new(),
            hotkey: None,
        });
        true
    }

    /// Delete a group
    pub fn delete(&mut self, id: u8) -> bool {
        self.groups.remove(&id).is_some()
    }

    /// Add a unit to a group
    pub fn add_unit(&mut self, group_id: u8, entity: Entity) -> bool {
        if let Some(group) = self.groups.get_mut(&group_id) {
            if !group.units.contains(&entity) {
                group.units.push(entity);
                return true;
            }
        }
        false
    }

    /// Remove a unit from a group
    pub fn remove_unit(&mut self, group_id: u8, entity: Entity) -> bool {
        if let Some(group) = self.groups.get_mut(&group_id) {
            if let Some(pos) = group.units.iter().position(|&e| e == entity) {
                group.units.remove(pos);
                return true;
            }
        }
        false
    }

    /// Get a group by ID
    pub fn get(&self, id: u8) -> Option<&UnitGroup> {
        self.groups.get(&id)
    }

    /// Get a mutable group by ID
    pub fn get_mut(&mut self, id: u8) -> Option<&mut UnitGroup> {
        self.groups.get_mut(&id)
    }

    /// Select a group
    pub fn select(&mut self, id: u8) -> bool {
        if self.groups.contains_key(&id) {
            self.selected_group = Some(id);
            true
        } else {
            false
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_group = None;
    }

    /// Get all units in selected group
    pub fn get_selected_units(&self) -> Vec<Entity> {
        if let Some(id) = self.selected_group {
            self.groups.get(&id).map(|g| g.units.clone()).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Get group count
    pub fn count(&self) -> usize {
        self.groups.len()
    }

    /// Get unit count in a group
    pub fn unit_count(&self, group_id: u8) -> usize {
        self.groups.get(&group_id).map(|g| g.units.len()).unwrap_or(0)
    }

    /// Find which groups a unit belongs to
    pub fn find_unit_groups(&self, entity: Entity) -> Vec<u8> {
        self.groups.iter()
            .filter(|(_, group)| group.units.contains(&entity))
            .map(|(&id, _)| id)
            .collect()
    }

    /// Remove unit from all groups
    pub fn remove_from_all(&mut self, entity: Entity) {
        for group in self.groups.values_mut() {
            group.units.retain(|&e| e != entity);
        }
    }
}

/// Component for units that can be grouped
#[derive(Debug, Clone, Component, Default)]
pub struct Groupable {
    pub groups: Vec<u8>,
}

/// System to handle unit groups
pub fn unit_group_system(
    mut manager: ResMut<UnitGroupManager>,
    mut events: EventReader<UnitGroupEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        match event {
            UnitGroupEvent::Create(id, name) => {
                manager.create(*id, name.clone());
                tracing::info!("Created unit group {}: {}", id, name);
            }
            UnitGroupEvent::Delete(id) => {
                manager.delete(*id);
                tracing::info!("Deleted unit group {}", id);
            }
            UnitGroupEvent::AddUnit(group_id, entity) => {
                manager.add_unit(*group_id, *entity);
            }
            UnitGroupEvent::RemoveUnit(group_id, entity) => {
                manager.remove_unit(*group_id, *entity);
            }
            UnitGroupEvent::SelectGroup(id) => {
                manager.select(*id);
                tracing::info!("Selected group {}", id);
            }
            UnitGroupEvent::ClearSelection => {
                manager.clear_selection();
            }
            UnitGroupEvent::Rename(id, name) => {
                if let Some(group) = manager.get_mut(*id) {
                    group.name = name.clone();
                }
            }
            UnitGroupEvent::AssignHotkey(group_id, hotkey) => {
                if let Some(group) = manager.get_mut(*group_id) {
                    group.hotkey = Some(*hotkey);
                }
            }
        }
    }

    // Keyboard shortcuts for group selection (Ctrl+1-9)
    let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

    // Number keys for group selection
    let number_keys = [
        (KeyCode::Digit1, 0u8),
        (KeyCode::Digit2, 1u8),
        (KeyCode::Digit3, 2u8),
        (KeyCode::Digit4, 3u8),
        (KeyCode::Digit5, 4u8),
        (KeyCode::Digit6, 5u8),
        (KeyCode::Digit7, 6u8),
        (KeyCode::Digit8, 7u8),
        (KeyCode::Digit9, 8u8),
    ];

    for (key, group_id) in number_keys {
        if ctrl && keyboard.just_pressed(key) {
            // Ctrl+Number: Assign selected units to group
            tracing::info!("Assigning to group {}", group_id);
        } else if keyboard.just_pressed(key) {
            // Number: Select group
            manager.select(group_id);
        }
    }

    // Tab to cycle through groups
    if keyboard.just_pressed(KeyCode::Tab) {
        let next_group = if let Some(current) = manager.selected_group {
            (current + 1) % manager.max_groups
        } else {
            0
        };
        manager.select(next_group);
    }
}