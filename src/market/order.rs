use crate::economy::company::CompanyHandle;
use crate::economy::resource::ResourceHandle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UnprocessedOrder {
    pub resource_handle: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    pub company_handle: CompanyHandle,
    pub resource_handle: ResourceHandle,
    pub amount: f64,
    pub max_price_per_unit: f64,
}
