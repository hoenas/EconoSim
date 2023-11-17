use std::collections::HashMap;

use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::market::order::Order;
use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;
pub type OrderHandle = usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub offers: HashMap<OfferHandle, Offer>,
    pub orders: HashMap<OrderHandle, Order>,
    pub price_index: HashMap<ResourceHandle, Option<(OfferHandle, f64)>>,
    pub order_index: HashMap<ResourceHandle, Option<(OrderHandle, f64)>>,
    pub resource_count: usize,
}

impl MarketData {
    pub fn new(resource_count: usize) -> MarketData {
        let mut price_index: HashMap<ResourceHandle, Option<(OfferHandle, f64)>> = HashMap::new();
        let mut order_index: HashMap<ResourceHandle, Option<(OrderHandle, f64)>> = HashMap::new();
        for resource in 0..resource_count {
            price_index.insert(resource, None);
            order_index.insert(resource, None);
        }
        MarketData {
            offers: HashMap::new(),
            orders: HashMap::new(),
            price_index: price_index,
            order_index: order_index,
            resource_count: 0,
        }
    }
}
