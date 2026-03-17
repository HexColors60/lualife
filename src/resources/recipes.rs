use std::collections::HashMap;

use super::ResourceType;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub inputs: HashMap<ResourceType, u32>,
    pub outputs: HashMap<ResourceType, u32>,
    pub time_ticks: u64,
}

impl Recipe {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            time_ticks: 10,
        }
    }

    pub fn input(mut self, resource_type: ResourceType, amount: u32) -> Self {
        self.inputs.insert(resource_type, amount);
        self
    }

    pub fn output(mut self, resource_type: ResourceType, amount: u32) -> Self {
        self.outputs.insert(resource_type, amount);
        self
    }

    pub fn time(mut self, ticks: u64) -> Self {
        self.time_ticks = ticks;
        self
    }
}

pub struct RecipeRegistry {
    recipes: HashMap<String, Recipe>,
}

impl Default for RecipeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl RecipeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            recipes: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        // Basic refinement recipes
        self.register(
            Recipe::new("refine_iron")
                .input(ResourceType::Iron, 10)
                .output(ResourceType::Iron, 5) // Refined iron (could be a separate type)
                .time(20)
        );

        self.register(
            Recipe::new("refine_copper")
                .input(ResourceType::Copper, 10)
                .output(ResourceType::Copper, 5)
                .time(15)
        );
    }

    pub fn register(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.name.clone(), recipe);
    }

    pub fn get(&self, name: &str) -> Option<&Recipe> {
        self.recipes.get(name)
    }

    pub fn all(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.values()
    }
}