use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct BeginnerGuide {
    pub objectives: Vec<Objective>,
    pub current_objective_index: usize,
    pub visible: bool,
}

#[derive(Debug, Clone)]
pub struct Objective {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub optional: bool,
}

impl Default for BeginnerGuide {
    fn default() -> Self {
        Self::new()
    }
}

impl BeginnerGuide {
    pub fn new() -> Self {
        Self {
            objectives: vec![
                Objective {
                    id: "observe_world".to_string(),
                    title: "Observe the World".to_string(),
                    description: "Use WASD and scroll to explore the 256x256 tile world."
                        .to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "select_creep".to_string(),
                    title: "Select a Creep".to_string(),
                    description: "Click on any colored circle (creep) to select it.".to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "check_ai_status".to_string(),
                    title: "Check AI Status".to_string(),
                    description: "Look at the AI status panel to see faction activities."
                        .to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "watch_mining".to_string(),
                    title: "Watch Mining".to_string(),
                    description: "Observe creeps mining resources from colored mine nodes."
                        .to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "use_minimap".to_string(),
                    title: "Use the Minimap".to_string(),
                    description: "Press M to toggle the minimap and click to navigate.".to_string(),
                    completed: false,
                    optional: true,
                },
                Objective {
                    id: "check_resources".to_string(),
                    title: "Check Resources".to_string(),
                    description: "Look at the resource bar to see available stockpiles."
                        .to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "pause_resume".to_string(),
                    title: "Pause/Resume".to_string(),
                    description: "Press Space to pause and resume the simulation.".to_string(),
                    completed: false,
                    optional: false,
                },
                Objective {
                    id: "save_game".to_string(),
                    title: "Save Your Game".to_string(),
                    description: "Press F5 to quick save your progress.".to_string(),
                    completed: false,
                    optional: true,
                },
                Objective {
                    id: "open_tech".to_string(),
                    title: "Open Technology Tree".to_string(),
                    description: "Press T to view and research new technologies.".to_string(),
                    completed: false,
                    optional: true,
                },
                Objective {
                    id: "explore_market".to_string(),
                    title: "Explore the Market".to_string(),
                    description: "Press K to open the market and see trading options.".to_string(),
                    completed: false,
                    optional: true,
                },
            ],
            current_objective_index: 0,
            visible: false,
        }
    }

    pub fn current_objective(&self) -> Option<&Objective> {
        self.objectives.get(self.current_objective_index)
    }

    pub fn complete_objective(&mut self, id: &str) {
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == id) {
            obj.completed = true;
        }

        while self.current_objective_index < self.objectives.len() {
            if !self.objectives[self.current_objective_index].completed {
                break;
            }
            self.current_objective_index += 1;
        }
    }

    pub fn progress(&self) -> (usize, usize) {
        let completed = self.objectives.iter().filter(|o| o.completed).count();
        (completed, self.objectives.len())
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

#[derive(Component)]
pub struct BeginnerGuideDisplay;

pub fn beginner_guide_system(
    mut guide: ResMut<BeginnerGuide>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Text, &mut Visibility), With<BeginnerGuideDisplay>>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        guide.toggle_visibility();
    }

    if guide.visible {
        for (mut text, mut vis) in query.iter_mut() {
            *vis = Visibility::Visible;

            let (completed, total) = guide.progress();
            let mut content = format!("=== Beginner Guide [{}/{}] ===\n\n", completed, total);

            for (i, obj) in guide.objectives.iter().enumerate() {
                let marker = if obj.completed { "[✓]" } else { "[ ]" };
                let optional = if obj.optional { " (optional)" } else { "" };
                let current = if i == guide.current_objective_index {
                    " → "
                } else {
                    "   "
                };
                content.push_str(&format!(
                    "{}{} {}{}\n",
                    current, marker, obj.title, optional
                ));
            }

            content.push_str("\nPress G to close");
            text.sections[0].value = content;
        }
    } else {
        for (_, mut vis) in query.iter_mut() {
            *vis = Visibility::Hidden;
        }
    }
}
