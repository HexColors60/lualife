use bevy::prelude::*;
use std::collections::HashMap;

use super::ScreenReaderState;

/// Keybindings configuration resource
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct KeybindingsConfig {
    pub bindings: HashMap<String, Vec<KeyCode>>,
    pub mouse_bindings: HashMap<String, MouseButton>,
    pub modifiers: HashMap<String, KeyModifiers>,
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();

        // Camera controls
        bindings.insert(
            "camera_up".to_string(),
            vec![KeyCode::KeyW, KeyCode::ArrowUp],
        );
        bindings.insert(
            "camera_down".to_string(),
            vec![KeyCode::KeyS, KeyCode::ArrowDown],
        );
        bindings.insert(
            "camera_left".to_string(),
            vec![KeyCode::KeyA, KeyCode::ArrowLeft],
        );
        bindings.insert(
            "camera_right".to_string(),
            vec![KeyCode::KeyD, KeyCode::ArrowRight],
        );
        bindings.insert("camera_zoom_in".to_string(), vec![]);
        bindings.insert("camera_zoom_out".to_string(), vec![]);

        // Selection
        bindings.insert("select".to_string(), vec![]);
        bindings.insert("deselect".to_string(), vec![KeyCode::Escape]);
        bindings.insert("select_all".to_string(), vec![KeyCode::KeyA]);
        bindings.insert("box_select".to_string(), vec![]);

        // UI
        bindings.insert("toggle_minimap".to_string(), vec![KeyCode::KeyM]);
        bindings.insert("toggle_log".to_string(), vec![KeyCode::KeyL]);
        bindings.insert("toggle_perf".to_string(), vec![KeyCode::F3]);
        bindings.insert("toggle_debug".to_string(), vec![KeyCode::F6]);
        bindings.insert("toggle_tech_tree".to_string(), vec![KeyCode::KeyT]);
        bindings.insert("toggle_market".to_string(), vec![KeyCode::KeyK]);
        bindings.insert("toggle_diplomacy".to_string(), vec![]);

        // Game
        bindings.insert("pause".to_string(), vec![KeyCode::Space]);
        bindings.insert("speed_up".to_string(), vec![KeyCode::Equal]);
        bindings.insert("speed_down".to_string(), vec![KeyCode::Minus]);
        bindings.insert("save".to_string(), vec![KeyCode::F5]);
        bindings.insert("load".to_string(), vec![KeyCode::F9]);
        bindings.insert("quick_save".to_string(), vec![]);

        // Building
        bindings.insert("build_spawn".to_string(), vec![KeyCode::Digit1]);
        bindings.insert("build_tower".to_string(), vec![KeyCode::Digit2]);
        bindings.insert("build_storage".to_string(), vec![KeyCode::Digit3]);
        bindings.insert("build_refinery".to_string(), vec![KeyCode::Digit4]);
        bindings.insert("build_research".to_string(), vec![KeyCode::Digit5]);
        bindings.insert("build_market".to_string(), vec![KeyCode::Digit6]);
        bindings.insert("build_road".to_string(), vec![KeyCode::Digit7]);
        bindings.insert("build_wall".to_string(), vec![KeyCode::Digit8]);
        bindings.insert("cancel_build".to_string(), vec![KeyCode::Escape]);

        // Unit commands
        bindings.insert("unit_move".to_string(), vec![KeyCode::KeyM]);
        bindings.insert("unit_attack".to_string(), vec![KeyCode::KeyA]);
        bindings.insert("unit_mine".to_string(), vec![KeyCode::KeyG]);
        bindings.insert("unit_build".to_string(), vec![KeyCode::KeyB]);
        bindings.insert("unit_transfer".to_string(), vec![KeyCode::KeyT]);
        bindings.insert("unit_stop".to_string(), vec![KeyCode::KeyS]);

        // Accessibility
        bindings.insert("accessibility_menu".to_string(), vec![KeyCode::F2]);
        bindings.insert("screen_reader_read".to_string(), vec![]);
        bindings.insert("screen_reader_help".to_string(), vec![]);

        Self {
            bindings,
            mouse_bindings: Self::default_mouse_bindings(),
            modifiers: Self::default_modifiers(),
        }
    }
}

impl KeybindingsConfig {
    fn default_mouse_bindings() -> HashMap<String, MouseButton> {
        let mut mouse = HashMap::new();
        mouse.insert("select".to_string(), MouseButton::Left);
        mouse.insert("box_select".to_string(), MouseButton::Left);
        mouse.insert("context_menu".to_string(), MouseButton::Right);
        mouse.insert("camera_pan".to_string(), MouseButton::Middle);
        mouse
    }

    fn default_modifiers() -> HashMap<String, KeyModifiers> {
        let mut mods = HashMap::new();
        mods.insert("select_all".to_string(), KeyModifiers::ctrl());
        mods.insert("toggle_diplomacy".to_string(), KeyModifiers::shift());
        mods.insert("quick_save".to_string(), KeyModifiers::ctrl());
        mods.insert("screen_reader_read".to_string(), KeyModifiers::alt());
        mods.insert("screen_reader_help".to_string(), KeyModifiers::alt());
        mods
    }

    /// Get keybinding by name
    pub fn get(&self, name: &str) -> Option<&Vec<KeyCode>> {
        self.bindings.get(name)
    }

    /// Check if a key is bound to an action
    pub fn is_bound(&self, name: &str, key: KeyCode) -> bool {
        if let Some(keys) = self.bindings.get(name) {
            keys.contains(&key)
        } else {
            false
        }
    }

    /// Set a keybinding
    pub fn set(&mut self, name: String, keys: Vec<KeyCode>) {
        self.bindings.insert(name, keys);
    }

    /// Reset to default keybindings
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Get all action names
    pub fn get_action_names(&self) -> Vec<&String> {
        self.bindings.keys().collect()
    }

    /// Get human-readable name for a key
    pub fn key_name(key: KeyCode) -> String {
        match key {
            KeyCode::KeyA => "A".to_string(),
            KeyCode::KeyB => "B".to_string(),
            KeyCode::KeyC => "C".to_string(),
            KeyCode::KeyD => "D".to_string(),
            KeyCode::KeyE => "E".to_string(),
            KeyCode::KeyF => "F".to_string(),
            KeyCode::KeyG => "G".to_string(),
            KeyCode::KeyH => "H".to_string(),
            KeyCode::KeyI => "I".to_string(),
            KeyCode::KeyJ => "J".to_string(),
            KeyCode::KeyK => "K".to_string(),
            KeyCode::KeyL => "L".to_string(),
            KeyCode::KeyM => "M".to_string(),
            KeyCode::KeyN => "N".to_string(),
            KeyCode::KeyO => "O".to_string(),
            KeyCode::KeyP => "P".to_string(),
            KeyCode::KeyQ => "Q".to_string(),
            KeyCode::KeyR => "R".to_string(),
            KeyCode::KeyS => "S".to_string(),
            KeyCode::KeyT => "T".to_string(),
            KeyCode::KeyU => "U".to_string(),
            KeyCode::KeyV => "V".to_string(),
            KeyCode::KeyW => "W".to_string(),
            KeyCode::KeyX => "X".to_string(),
            KeyCode::KeyY => "Y".to_string(),
            KeyCode::KeyZ => "Z".to_string(),
            KeyCode::Digit0 => "0".to_string(),
            KeyCode::Digit1 => "1".to_string(),
            KeyCode::Digit2 => "2".to_string(),
            KeyCode::Digit3 => "3".to_string(),
            KeyCode::Digit4 => "4".to_string(),
            KeyCode::Digit5 => "5".to_string(),
            KeyCode::Digit6 => "6".to_string(),
            KeyCode::Digit7 => "7".to_string(),
            KeyCode::Digit8 => "8".to_string(),
            KeyCode::Digit9 => "9".to_string(),
            KeyCode::ArrowUp => "Up".to_string(),
            KeyCode::ArrowDown => "Down".to_string(),
            KeyCode::ArrowLeft => "Left".to_string(),
            KeyCode::ArrowRight => "Right".to_string(),
            KeyCode::Space => "Space".to_string(),
            KeyCode::Escape => "Esc".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "Page Up".to_string(),
            KeyCode::PageDown => "Page Down".to_string(),
            KeyCode::F1 => "F1".to_string(),
            KeyCode::F2 => "F2".to_string(),
            KeyCode::F3 => "F3".to_string(),
            KeyCode::F4 => "F4".to_string(),
            KeyCode::F5 => "F5".to_string(),
            KeyCode::F6 => "F6".to_string(),
            KeyCode::F7 => "F7".to_string(),
            KeyCode::F8 => "F8".to_string(),
            KeyCode::F9 => "F9".to_string(),
            KeyCode::F10 => "F10".to_string(),
            KeyCode::F11 => "F11".to_string(),
            KeyCode::F12 => "F12".to_string(),
            KeyCode::ShiftLeft | KeyCode::ShiftRight => "Shift".to_string(),
            KeyCode::ControlLeft | KeyCode::ControlRight => "Ctrl".to_string(),
            KeyCode::AltLeft | KeyCode::AltRight => "Alt".to_string(),
            _ => format!("{:?}", key),
        }
    }

    /// Get human-readable binding string
    pub fn get_binding_string(&self, name: &str) -> String {
        let mut parts = Vec::new();

        // Add modifiers
        if let Some(mods) = self.modifiers.get(name) {
            if mods.ctrl {
                parts.push("Ctrl".to_string());
            }
            if mods.alt {
                parts.push("Alt".to_string());
            }
            if mods.shift {
                parts.push("Shift".to_string());
            }
        }

        // Add keys
        if let Some(keys) = self.bindings.get(name) {
            for key in keys {
                parts.push(Self::key_name(*key));
            }
        }

        // Add mouse buttons
        if let Some(mouse) = self.mouse_bindings.get(name) {
            parts.push(format!("{:?}", mouse));
        }

        parts.join(" + ")
    }
}

/// Key modifiers for keybindings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifiers {
    pub fn new() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            ..Self::new()
        }
    }

    pub fn alt() -> Self {
        Self {
            alt: true,
            ..Self::new()
        }
    }

    pub fn shift() -> Self {
        Self {
            shift: true,
            ..Self::new()
        }
    }

    /// Check if current keyboard state matches these modifiers
    pub fn matches(&self, keyboard: &ButtonInput<KeyCode>) -> bool {
        let ctrl =
            keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
        let alt = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
        let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

        self.ctrl == ctrl && self.alt == alt && self.shift == shift
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::new()
    }
}

/// System to handle keybinding input
pub fn handle_keybinding_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    config: Res<KeybindingsConfig>,
    mut screen_reader: ResMut<ScreenReaderState>,
) {
    // Check each binding
    for (action, keys) in &config.bindings {
        let modifiers_match = if let Some(mods) = config.modifiers.get(action) {
            mods.matches(&keyboard)
        } else {
            // No modifiers required - check that no modifiers are pressed
            !keyboard.pressed(KeyCode::ControlLeft)
                && !keyboard.pressed(KeyCode::ControlRight)
                && !keyboard.pressed(KeyCode::AltLeft)
                && !keyboard.pressed(KeyCode::AltRight)
                && !keyboard.pressed(KeyCode::ShiftLeft)
                && !keyboard.pressed(KeyCode::ShiftRight)
        };

        if modifiers_match {
            for key in keys {
                if keyboard.just_pressed(*key) {
                    // Emit event for the action (would be handled by game systems)
                    tracing::debug!("Keybinding triggered: {}", action);

                    // Announce for screen reader
                    if screen_reader.enabled {
                        screen_reader.say(format!("{} activated", action.replace("_", " ")));
                    }
                }
            }
        }
    }
}

/// Event for keybinding changes
#[derive(Debug, Clone, Event)]
pub struct KeybindingChangedEvent {
    pub action: String,
    pub old_keys: Vec<KeyCode>,
    pub new_keys: Vec<KeyCode>,
}

/// Component for focusable UI elements
#[derive(Debug, Clone, Component)]
pub struct Focusable {
    pub focused: bool,
    pub focus_order: i32,
    pub announce_on_focus: bool,
}

impl Default for Focusable {
    fn default() -> Self {
        Self {
            focused: false,
            focus_order: 0,
            announce_on_focus: true,
        }
    }
}
