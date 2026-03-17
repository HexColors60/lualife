use bevy::prelude::*;

use crate::resources::ResourceType;

#[derive(Debug, Clone, Component)]
pub struct Refinery {
    pub input_type: ResourceType,
    pub output_type: ResourceType,
    pub input_amount: u32,
    pub output_amount: u32,
    pub progress: f32,
    pub processing_time: f32,
}

impl Refinery {
    pub fn new(input_type: ResourceType, output_type: ResourceType) -> Self {
        Self {
            input_type,
            output_type,
            input_amount: 10,
            output_amount: 5,
            progress: 0.0,
            processing_time: 20.0,
        }
    }

    pub fn tick(&mut self, has_input: bool) -> Option<ResourceType> {
        if !has_input {
            return None;
        }

        self.progress += 1.0;

        if self.progress >= self.processing_time {
            self.progress = 0.0;
            return Some(self.output_type);
        }

        None
    }
}