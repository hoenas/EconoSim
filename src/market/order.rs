use crate::economy::company::CompanyHandle;
use crate::economy::resource::ResourceHandle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UnprocessedOrder {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    pub company: CompanyHandle,
    pub resource: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
}
