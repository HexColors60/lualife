use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct TutorialState {
    pub active: bool,
    pub current_step: usize,
    pub total_steps: usize,
    pub steps: Vec<TutorialStep>,
    pub completed: bool,
    pub skipped: bool,
}

#[derive(Debug, Clone)]
pub struct TutorialStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub highlight_target: Option<String>,
    pub required_action: TutorialAction,
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorialAction {
    ClickAnywhere,
    SelectUnit,
    PanCamera,
    ZoomCamera,
    OpenMinimap,
    PressKey(KeyCode),
    Wait,
    None,
}

#[derive(Event, Debug, Clone)]
pub enum TutorialEvent {
    Start,
    NextStep,
    PreviousStep,
    Skip,
    Complete,
    ActionCompleted(TutorialAction),
}

impl TutorialState {
    pub fn new() -> Self {
        Self {
            active: false,
            current_step: 0,
            total_steps: 0,
            steps: Self::create_default_steps(),
            completed: false,
            skipped: false,
        }
    }

    fn create_default_steps() -> Vec<TutorialStep> {
        vec![
            TutorialStep {
                id: "welcome".to_string(),
                title: "Welcome to Lualife!".to_string(),
                description: "This is a Screeps-like autonomous simulation where AI factions control units using Lua scripts.\n\nClick anywhere to continue.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::ClickAnywhere,
                completed: false,
            },
            TutorialStep {
                id: "world_overview".to_string(),
                title: "World Overview".to_string(),
                description: "The world is a 256x256 tile grid divided into 32x32 rooms.\nEach room is 8x8 tiles.\n\nUse WASD or Arrow keys to pan the camera.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::PanCamera,
                completed: false,
            },
            TutorialStep {
                id: "zoom".to_string(),
                title: "Zooming".to_string(),
                description: "Use the mouse scroll wheel to zoom in and out.\n\nTry zooming now to see more or less of the world.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::ZoomCamera,
                completed: false,
            },
            TutorialStep {
                id: "select_unit".to_string(),
                title: "Selecting Units".to_string(),
                description: "Click on any creep (colored circle) to select it.\nSelected units show their stats in the panel on the right.\n\nTry selecting a unit now.".to_string(),
                highlight_target: Some("unit_panel".to_string()),
                required_action: TutorialAction::SelectUnit,
                completed: false,
            },
            TutorialStep {
                id: "minimap".to_string(),
                title: "Minimap".to_string(),
                description: "Press M to toggle the minimap.\nThe minimap shows the entire world at a glance.\n\nTry opening the minimap.".to_string(),
                highlight_target: Some("minimap".to_string()),
                required_action: TutorialAction::OpenMinimap,
                completed: false,
            },
            TutorialStep {
                id: "ai_scripts".to_string(),
                title: "AI Scripts".to_string(),
                description: "Each faction runs Lua scripts from the ai/ai_xx/ folders.\nScripts control unit behavior automatically.\n\nThe game runs 32 AI factions simultaneously!".to_string(),
                highlight_target: Some("ai_status".to_string()),
                required_action: TutorialAction::ClickAnywhere,
                completed: false,
            },
            TutorialStep {
                id: "resources".to_string(),
                title: "Resources".to_string(),
                description: "There are 10 resource types:\nPower, Iron, Copper, Silicon, Crystal,\nCarbon, Stone, Sulfur, Water, Biomass.\n\nMines are shown as colored dots on the map.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::ClickAnywhere,
                completed: false,
            },
            TutorialStep {
                id: "game_speed".to_string(),
                title: "Controlling Simulation".to_string(),
                description: "Press Space to pause/resume.\nPress +/- to change simulation speed.\nPress F5 to save, F9 to load.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::ClickAnywhere,
                completed: false,
            },
            TutorialStep {
                id: "debug_tools".to_string(),
                title: "Debug Tools".to_string(),
                description: "Press F3 for debug overlay.\nPress F4 for performance display.\nPress F6 for path visualization.\nPress F7 for tower range display.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::ClickAnywhere,
                completed: false,
            },
            TutorialStep {
                id: "complete".to_string(),
                title: "Tutorial Complete!".to_string(),
                description: "You're ready to explore Lualife!\n\nRemember: You're in GOD mode - you can observe\nand inspect everything but units are controlled by AI.\n\nPress H anytime to show hints.\nPress ? for help encyclopedia.".to_string(),
                highlight_target: None,
                required_action: TutorialAction::None,
                completed: false,
            },
        ]
    }

    pub fn current_step(&self) -> Option<&TutorialStep> {
        self.steps.get(self.current_step)
    }

    pub fn advance(&mut self) -> bool {
        if self.current_step < self.steps.len() - 1 {
            self.steps[self.current_step].completed = true;
            self.current_step += 1;
            true
        } else {
            self.completed = true;
            false
        }
    }

    pub fn go_back(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }
}

#[derive(Component)]
pub struct TutorialText;

pub fn setup_tutorial_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(50.0),
                width: Val::Px(400.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.95)),
            visibility: Visibility::Hidden,
            ..default()
        },
        TutorialText,
    ));
}

pub fn tutorial_system(
    mut tutorial_state: ResMut<TutorialState>,
    mut events: EventReader<TutorialEvent>,
    mut query: Query<(&mut Text, &mut Visibility), With<TutorialText>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for event in events.read() {
        match event {
            TutorialEvent::Start => {
                tutorial_state.active = true;
                tutorial_state.current_step = 0;
                tutorial_state.completed = false;
                tutorial_state.skipped = false;
                for (_, mut vis) in query.iter_mut() {
                    *vis = Visibility::Visible;
                }
            }
            TutorialEvent::NextStep => {
                tutorial_state.advance();
                if tutorial_state.completed {
                    for (_, mut vis) in query.iter_mut() {
                        *vis = Visibility::Hidden;
                    }
                }
            }
            TutorialEvent::PreviousStep => {
                tutorial_state.go_back();
            }
            TutorialEvent::Skip => {
                tutorial_state.skipped = true;
                tutorial_state.active = false;
                for (_, mut vis) in query.iter_mut() {
                    *vis = Visibility::Hidden;
                }
            }
            TutorialEvent::Complete => {
                tutorial_state.completed = true;
                tutorial_state.active = false;
                for (_, mut vis) in query.iter_mut() {
                    *vis = Visibility::Hidden;
                }
            }
            TutorialEvent::ActionCompleted(action) => {
                if let Some(step) = tutorial_state.current_step() {
                    if step.required_action == *action {
                        tutorial_state.advance();
                    }
                }
            }
        }
    }

    if tutorial_state.active {
        if let Some(step) = tutorial_state.current_step() {
            for (mut text, _) in query.iter_mut() {
                let progress = format!(
                    "[{}/{}]",
                    tutorial_state.current_step + 1,
                    tutorial_state.steps.len()
                );
                text.sections[0].value = format!(
                    "{} {}\n\n{}\n\nPress Enter to continue, ESC to skip",
                    progress, step.title, step.description
                );
            }
        }
    }
}

pub fn handle_tutorial_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<TutorialEvent>,
    tutorial_state: Res<TutorialState>,
) {
    if keyboard.just_pressed(KeyCode::Enter) && tutorial_state.active {
        events.send(TutorialEvent::NextStep);
    }
    if keyboard.just_pressed(KeyCode::Escape) && tutorial_state.active {
        events.send(TutorialEvent::Skip);
    }
    if keyboard.just_pressed(KeyCode::F1) && !tutorial_state.active && !tutorial_state.completed {
        events.send(TutorialEvent::Start);
    }
}
