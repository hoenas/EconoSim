use std::collections::HashMap;

use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::market::order::Order;
use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;
pub type OrderHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct MarketData {
    pub offers: HashMap<OfferHandle, Offer>,
    pub orders: HashMap<OrderHandle, Order>,
    pub price_index: HashMap<ResourceHandle, Option<(OfferHandle, f64)>>,
    pub order_index: HashMap<ResourceHandle, Option<(OrderHandle, f64)>>,
    pub resource_count: usize,
}

impl MarketData {
    pub fn new() -> MarketData {
        MarketData {
            offers: HashMap::new(),
            orders: HashMap::new(),
            price_index: HashMap::new(),
            order_index: HashMap::new(),
            resource_count: 0,
        }
    }
}
