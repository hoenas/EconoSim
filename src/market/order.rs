use crate::economy::company::CompanyHandle;
use crate::economy::resource::ResourceHandle;
use serde::{Deserialize, Serialize};

pub type OrderHandle = usize;
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UnprocessedOrder {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
    pub time_to_live: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    pub company: Option<CompanyHandle>,
    pub resource: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
    pub time_to_live: usize,
}
