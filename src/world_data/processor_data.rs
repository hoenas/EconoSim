use crate::economy::company::{Company, CompanyHandle};
use crate::economy::recipe::{Recipe, RecipeHandle};
use crate::economy::resource::{Resource, ResourceHandle};
use crate::market::marketplace::Marketplace;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessorData {
    pub processor_price: f64,
}

impl ProcessorData {
    pub fn new() -> ProcessorData {
        ProcessorData {
            processor_price: 1000.0,
        }
    }
}
