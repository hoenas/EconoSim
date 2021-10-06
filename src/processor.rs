use crate::recipe::Recipe;

pub struct Processor <'a> {
    name: String,
    production_speed: f64,
    recipe: Recipe <'a>,
}
