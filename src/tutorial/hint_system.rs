use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource, Debug, Clone)]
pub struct HintState {
    pub enabled: bool,
    pub hints: Vec<Hint>,
    pub shown_hints: Vec<String>,
    pub current_hint: Option<String>,
    pub hint_queue: VecDeque<String>,
    pub display_timer: Timer,
}

#[derive(Debug, Clone)]
pub struct Hint {
    pub id: String,
    pub text: String,
    pub trigger: HintTrigger,
    pub priority: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintTrigger {
    OnFirstRun,
    OnUnitSelected,
    OnResourceLow,
    OnCombat,
    OnBuildingPlaced,
    OnResearchComplete,
    OnTradeComplete,
    Manual,
}

#[derive(Event, Debug, Clone)]
pub enum HintEvent {
    Show(String),
    Dismiss,
    Trigger(HintTrigger),
    Enable,
    Disable,
}

impl Default for HintState {
    fn default() -> Self {
        Self::new()
    }
}

impl HintState {
    pub fn new() -> Self {
        let hints = vec![
            Hint {
                id: "first_run".to_string(),
                text: "Press F1 to start the interactive tutorial!".to_string(),
                trigger: HintTrigger::OnFirstRun,
                priority: 100,
            },
            Hint {
                id: "select_unit".to_string(),
                text: "Click on a creep to see its details in the right panel.".to_string(),
                trigger: HintTrigger::OnUnitSelected,
                priority: 50,
            },
            Hint {
                id: "camera_control".to_string(),
                text: "Use WASD or Arrow keys to pan the camera. Scroll to zoom.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 30,
            },
            Hint {
                id: "minimap".to_string(),
                text: "Press M to toggle the minimap. Click on it to jump to that area."
                    .to_string(),
                trigger: HintTrigger::Manual,
                priority: 30,
            },
            Hint {
                id: "pause_game".to_string(),
                text: "Press Space to pause/resume the simulation.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 30,
            },
            Hint {
                id: "save_load".to_string(),
                text: "Press F5 to quick save, F9 to quick load.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 20,
            },
            Hint {
                id: "debug_overlays".to_string(),
                text: "Press F3 for debug info, F4 for performance stats.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 20,
            },
            Hint {
                id: "tech_tree".to_string(),
                text: "Press T to open the technology tree and research new abilities.".to_string(),
                trigger: HintTrigger::OnResearchComplete,
                priority: 40,
            },
            Hint {
                id: "market".to_string(),
                text: "Press K to open the market and trade resources with other factions."
                    .to_string(),
                trigger: HintTrigger::OnTradeComplete,
                priority: 40,
            },
            Hint {
                id: "diplomacy".to_string(),
                text: "Press Shift+D to open diplomacy and manage faction relations.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 30,
            },
            Hint {
                id: "help".to_string(),
                text: "Press ? or H anytime to show this help panel.".to_string(),
                trigger: HintTrigger::Manual,
                priority: 10,
            },
        ];

        Self {
            enabled: true,
            hints,
            shown_hints: Vec::new(),
            current_hint: None,
            hint_queue: VecDeque::new(),
            display_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }

    pub fn show_hint(&mut self, id: &str) {
        if !self.shown_hints.contains(&id.to_string()) && self.enabled {
            if let Some(hint) = self.hints.iter().find(|h| h.id == id) {
                self.current_hint = Some(hint.text.clone());
                self.shown_hints.push(id.to_string());
                self.display_timer.reset();
            }
        }
    }

    pub fn dismiss(&mut self) {
        self.current_hint = None;
    }

    pub fn is_hint_shown(&self, id: &str) -> bool {
        self.shown_hints.contains(&id.to_string())
    }
}

#[derive(Component)]
pub struct HintDisplay;

pub fn hint_system(
    mut hint_state: ResMut<HintState>,
    mut events: EventReader<HintEvent>,
    mut query: Query<&mut Text, With<HintDisplay>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        match event {
            HintEvent::Show(id) => {
                hint_state.show_hint(id);
            }
            HintEvent::Dismiss => {
                hint_state.dismiss();
            }
            HintEvent::Trigger(trigger) => {
                // Find matching hint
                let hint_id = hint_state.hints.iter()
                    .find(|h| h.trigger == *trigger && !hint_state.shown_hints.contains(&h.id))
                    .map(|h| h.id.clone());
                
                if let Some(id) = hint_id {
                    hint_state.show_hint(&id);
                }
            }
            HintEvent::Enable => {
                hint_state.enabled = true;
            }
            HintEvent::Disable => {
                hint_state.enabled = false;
            }
        }
    }

    // Get current hint text before mutating
    let hint_text = hint_state.current_hint.clone();
    let timer_finished = hint_state.display_timer.finished();

    if hint_text.is_some() {
        hint_state.display_timer.tick(time.delta());
        if timer_finished {
            hint_state.dismiss();
        }
    }

    // Update display
    for mut text in query.iter_mut() {
        if let Some(ref hint) = hint_text {
            text.sections[0].value = format!("💡 {}", hint);
        } else {
            text.sections[0].value = String::new();
        }
    }

    if keyboard.just_pressed(KeyCode::KeyH) {
        hint_state.enabled = !hint_state.enabled;
    }
}
