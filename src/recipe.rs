use crate::resource::Resource;

pub struct Recipe<'a> {
    ingredients: Vec<(&'a Resource, f64)>,
    product: &'a Resource,
    production_speed: f64,
}
