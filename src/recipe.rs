use crate::resource::ResourceHandle;

pub struct Recipe {
    pub ingredients: Vec<(ResourceHandle, f64)>,
    pub products: Vec<(ResourceHandle, f64)>,
    pub production_speed: f64,
}
