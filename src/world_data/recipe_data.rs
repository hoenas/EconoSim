use crate::economy::recipe::{Recipe, RecipeHandle};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RecipeData {
    pub recipes: Vec<Recipe>,
}

impl RecipeData {
    pub fn new() -> RecipeData {
        RecipeData {
            recipes: Vec::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) -> RecipeHandle {
        self.recipes.push(recipe);
        self.recipes.len() - 1
    }

    pub fn get_recipe_by_handle(&self, recipe_handle: RecipeHandle) -> Option<&Recipe> {
        if recipe_handle < self.recipes.len() {
            Some(&self.recipes[recipe_handle])
        } else {
            None
        }
    }
}
