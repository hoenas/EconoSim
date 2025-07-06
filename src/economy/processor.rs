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
        if self.productive && stock.remove_resources_from_stock_if_possible(transaction) {
            // Transaction can be done, add generated resources to stock
            for (resource, production_factor) in recipe.products.iter() {
                let amount = production_factor * self.production_speed;
                stock.add_resource_to_stock(*resource, amount);
            }
            self.produced_last_tick = true;
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::Processor;
    use crate::economy::recipe::{Recipe, RecipeHandle};
    use crate::economy::resource::ResourceHandle;
    use crate::economy::stock::Stock;

    use crate::world_data::recipe_data::RecipeData;

    #[test]
    fn tick() {
        // Test setup
        // Create processor
        let mut processor = Processor {
            name: String::from("testprocessor"),
            produced_last_tick: false,
            production_speed: 1.0,
            recipe: 0,
            productive: true,
        };
        // Create recipe data
        let mut ingredients: HashMap<ResourceHandle, f64> = HashMap::new();
        ingredients.insert(0, 1.0);
        ingredients.insert(1, 2.0);
        ingredients.insert(2, 0.0);
        let mut products: HashMap<RecipeHandle, f64> = HashMap::new();
        products.insert(2, 5.0);
        let recipe = Recipe {
            name: String::from("test"),
            ingredients: ingredients,
            production_speed: 1.0,
            products: products,
        };
        let mut recipe_data = RecipeData::new();
        recipe_data.add_recipe(recipe);
        // Create stock
        let mut stock = Stock::new();
        stock.add_resource_to_stock(0, 1.0);
        stock.add_resource_to_stock(1, 2.0);

        // Test
        // First tick - everything should work
        // Resources should be down to zero - except the produced one
        processor.tick(&mut stock, &recipe_data);
        assert!(processor.produced_last_tick);
        assert_eq!(*stock.resources.get(&0).unwrap(), 0.0);
        assert_eq!(*stock.resources.get(&1).unwrap(), 0.0);
        assert_eq!(*stock.resources.get(&2).unwrap(), 5.0);
        // Second tick - producing should not have worked
        // Resources should be unchanged
        processor.tick(&mut stock, &recipe_data);
        assert!(!processor.produced_last_tick);
        assert_eq!(*stock.resources.get(&0).unwrap(), 0.0);
        assert_eq!(*stock.resources.get(&1).unwrap(), 0.0);
        assert_eq!(*stock.resources.get(&2).unwrap(), 5.0);
    }
}
