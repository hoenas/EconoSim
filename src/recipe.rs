use crate::resource::ResourceHandle;

pub struct Recipe {
    ingredients: Vec<(ResourceHandle, f64)>,
    product: ResourceHandle,
    production_speed: f64,
}
