use crate::economy::company::{Company, CompanyHandle};
use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::market::order::Order;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type OfferHandle = usize;
pub type OrderHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Marketplace {
    pub offers: HashMap<OfferHandle, Offer>,
    pub orders: HashMap<OrderHandle, Order>,
    pub price_index: HashMap<ResourceHandle, Option<(OfferHandle, f64)>>,
    pub order_index: HashMap<ResourceHandle, Option<(OfferHandle, f64)>>,
    pub resource_count: ResourceHandle,
    paybacks: Vec<(CompanyHandle, f64)>,
    next_offer_id: OfferHandle,
    next_order_id: OrderHandle,
}

impl Marketplace {
    pub fn new() -> Marketplace {
        Marketplace {
            offers: HashMap::new(),
            orders: HashMap::new(),
            price_index: HashMap::new(),
            order_index: HashMap::new(),
            resource_count: 0,
            paybacks: Vec::new(),
            next_offer_id: 0,
            next_order_id: 0,
        }
    }

    pub fn update_price_index(&mut self) {
        for resource_handle in 0..self.resource_count {
            let offer = self.get_cheapest_offer(resource_handle);
            self.price_index.insert(resource_handle, offer);
        }
    }

    pub fn update_order_index(&mut self) {
        for resource_handle in 0..self.resource_count {
            let order = self.get_highest_order(resource_handle);
            self.order_index.insert(resource_handle, order);
        }
    }

    pub fn get_cheapest_offer(&self, resource: ResourceHandle) -> Option<(OfferHandle, f64)> {
        let mut cheapest_offer: Option<(OfferHandle, f64)> = None;
        for offer_handle in 0..self.offers.len() - 1 {
            let offer = self.get_offer_by_handle(offer_handle).unwrap();
            if offer.resource == resource {
                if cheapest_offer.is_none() {
                    cheapest_offer = Some((offer_handle, offer.price_per_unit));
                } else if cheapest_offer.unwrap().1 > offer.price_per_unit {
                    cheapest_offer = Some((offer_handle, offer.price_per_unit));
                }
            }
        }
        cheapest_offer
    }

    pub fn get_highest_order(&self, resource: ResourceHandle) -> Option<(OrderHandle, f64)> {
        let mut highest_order: Option<(OrderHandle, f64)> = None;
        for order_handle in 0..self.orders.len() - 1 {
            let order = self.get_order_by_handle(order_handle).unwrap();
            if order.resource == resource {
                if highest_order.is_none() {
                    highest_order = Some((order_handle, order.max_price_per_unit));
                } else if highest_order.unwrap().1 < order.max_price_per_unit {
                    highest_order = Some((order_handle, order.max_price_per_unit));
                }
            }
        }
        highest_order
    }

    pub fn place_offer(&mut self, offer: Offer) -> OfferHandle {
        self.next_offer_id += 1;
        self.offers.insert(self.next_offer_id, offer);
        self.update_price_index();
        self.next_offer_id
    }

    pub fn place_order(&mut self, order: Order) -> OfferHandle {
        self.next_order_id += 1;
        self.orders.insert(self.next_order_id, order);
        self.update_order_index();
        self.next_order_id
    }

    pub fn get_offer_by_handle(&self, offer_handle: OfferHandle) -> Option<&Offer> {
        Some(&self.offers[&offer_handle])
    }

    pub fn get_order_by_handle(&self, order_handle: OrderHandle) -> Option<&Order> {
        Some(&self.orders[&order_handle])
    }

    pub fn accept_offer(&mut self, offer: &mut Offer, player: &mut Company, amount: f64) {
        if amount <= offer.amount {
            let price = offer.price_per_unit * amount;
            if price <= player.currency {
                offer.amount -= amount;
                player.currency -= price;
                // Clean up offers and update price index
                if offer.amount <= 0.0 {
                    self.offers.retain(|_, v| v.amount <= 0.0);
                    self.update_price_index();
                }
            }
        }
    }

    pub fn perform_paybacks(&mut self, companies: &mut Vec<Company>) {
        let mut payback = self.paybacks.pop();
        while payback.is_some() {
            let company_handle = payback.unwrap().0;
            let currency = payback.unwrap().1;
            let player = &mut companies[company_handle];
            player.currency += currency;
            payback = self.paybacks.pop();
        }
    }
}
