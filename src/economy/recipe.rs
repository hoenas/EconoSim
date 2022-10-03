use crate::economy::resource::ResourceHandle;
use serde::{Deserialize, Serialize};

pub type RecipeHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<(ResourceHandle, f64)>,
    pub products: Vec<(ResourceHandle, f64)>,
    pub production_speed: f64,
}
