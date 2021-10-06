use crate::recipe::Recipe;
use crate::stock::Stock;

pub struct Processor {
    name: String,
    production_speed: f64,
    recipe: Recipe,
}

impl<'a> Processor {
    fn tick(&self, stock: &mut Stock) {
        // TODO: Get required resources from recipe
        //if stock.remove_from_stock_if_possible(resource, amount)
    }
}
