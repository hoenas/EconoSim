use crate::economy::recipe::RecipeHandle;
use crate::economy::stock::Stock;
use crate::world_data::recipe_data::RecipeData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Processor {
    pub name: String,
    pub production_speed: f64,
    pub recipe: RecipeHandle,
    pub productive: bool,
}

impl<'a> Processor {
    pub fn tick(&self, stock: &mut Stock, recipe_data: &RecipeData) {
        // Get recipe
        let recipe = recipe_data.get_recipe_by_handle(self.recipe).unwrap();
        // Check if transaction can be done
        if self.productive && stock.make_transaction(&recipe.ingredients) {
            // Transaction can be done, add generated resources to stock
            for (resource, production_factor) in recipe.products.iter() {
                let amount = production_factor * self.production_speed;
                stock.add_to_stock(*resource, amount);
            }
        }
    }
}
