use crate::recipe::Recipe;
use crate::stock::Stock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Processor {
    pub name: String,
    pub production_speed: f64,
    pub recipe: Recipe,
}

impl<'a> Processor {
    pub fn tick(&self, stock: &mut Stock) {
        // Check if transaction can be done
        if stock.make_transaction(&self.recipe.ingredients) {
            // Transaction can be done, add generated resources to stock
            for (resource, production_factor) in self.recipe.products.iter() {
                let amount = production_factor * self.production_speed;
                stock.add_to_stock(*resource, amount);
            }
        }
    }
}
