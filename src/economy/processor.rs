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
    pub produced_last_tick: bool,
}

impl<'a> Processor {
    pub fn tick(&mut self, stock: &mut Stock, recipe_data: &RecipeData) {
        // Get recipe
        let recipe = recipe_data.get_recipe_by_handle(self.recipe).unwrap();
        // Check if transaction can be done
        self.produced_last_tick = false;
        let transaction = &recipe.ingredients.iter().map(|x| (*x.0, *x.1)).collect();
        if self.productive && stock.make_transaction(transaction) {
            // Transaction can be done, add generated resources to stock
            for (resource, production_factor) in recipe.products.iter() {
                let amount = production_factor * self.production_speed;
                stock.add_to_stock(*resource, amount);
            }
            self.produced_last_tick = true;
        }
    }
}
