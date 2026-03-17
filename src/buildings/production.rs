use bevy::prelude::*;

use crate::resources::ResourceType;

#[derive(Debug, Clone, Component)]
pub struct Factory {
    pub recipe: Option<String>,
    pub progress: f32,
    pub production_time: f32,
    pub input_buffer: std::collections::HashMap<ResourceType, u32>,
    pub output_buffer: std::collections::HashMap<ResourceType, u32>,
}

impl Default for Factory {
    fn default() -> Self {
        Self::new()
    }
}

impl Factory {
    pub fn new() -> Self {
        Self {
            recipe: None,
            progress: 0.0,
            production_time: 50.0,
            input_buffer: std::collections::HashMap::new(),
            output_buffer: std::collections::HashMap::new(),
        }
    }

    pub fn set_recipe(&mut self, recipe: String, time: f32) {
        self.recipe = Some(recipe);
        self.production_time = time;
        self.progress = 0.0;
    }

    pub fn add_input(&mut self, resource: ResourceType, amount: u32) {
        let current = self.input_buffer.get(&resource).copied().unwrap_or(0);
        self.input_buffer.insert(resource, current + amount);
    }

    pub fn tick(&mut self) -> bool {
        if self.recipe.is_none() {
            return false;
        }

        self.progress += 1.0;

        if self.progress >= self.production_time {
            self.progress = 0.0;
            return true;
        }

        false
    }
}
