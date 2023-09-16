use crate::economy::resource::ResourceHandle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RecipeHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: HashMap<ResourceHandle, f64>,
    pub products: HashMap<ResourceHandle, f64>,
    pub production_speed: f64,
}

impl Recipe {
    pub fn new(name: String, production_speed: f64) -> Self {
        Self {
            name: name,
            ingredients: HashMap::new(),
            products: HashMap::new(),
            production_speed: production_speed,
        }
    }
}
