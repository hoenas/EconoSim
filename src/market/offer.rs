use crate::economy::company::CompanyHandle;
use crate::economy::resource::ResourceHandle;

use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UnprocessedOffer {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub price_per_unit: f64,
    pub time_to_live: usize,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Offer {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub price_per_unit: f64,
    pub company: Option<CompanyHandle>,
    pub time_to_live: usize,
}
