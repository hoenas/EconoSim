use crate::economy::company::Company;
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
        for (offer_handle, offer) in self.offers.iter() {
            if offer.resource == resource {
                if cheapest_offer.is_none() {
                    cheapest_offer = Some((*offer_handle, offer.price_per_unit));
                } else if cheapest_offer.unwrap().1 > offer.price_per_unit {
                    cheapest_offer = Some((*offer_handle, offer.price_per_unit));
                }
            }
        }
        cheapest_offer
    }

    pub fn get_highest_order(&self, resource: ResourceHandle) -> Option<(OrderHandle, f64)> {
        let mut highest_order: Option<(OrderHandle, f64)> = None;
        for (order_handle, order) in self.orders.iter() {
            if order.resource == resource {
                if highest_order.is_none() {
                    highest_order = Some((*order_handle, order.max_price_per_unit));
                } else if highest_order.unwrap().1 < order.max_price_per_unit {
                    highest_order = Some((*order_handle, order.max_price_per_unit));
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

    fn check_orders(&mut self, companies: &mut Vec<Company>) {
        // Check all orders
        // for (order_handle, order) in self.orders.iter_mut() {
        //     while order.amount > 0.0 {
        //         match self.price_index.get(&order.resource) {
        //             Some(value) => {
        //                 let offer_handle = value.unwrap().0;
        //                 let offer_price = value.unwrap().1;
        //                 if offer_price <= order.max_price_per_unit {
        //                     let offer_object = self.get_offer_by_handle(offer_handle).unwrap();
        //                     if offer_object.amount < order.amount {
        //                         // Consume offer
        //                         order.amount -= offer_object.amount;
        //                         // Give resources to company
        //                         companies[order.company]
        //                             .stock
        //                             .add_to_stock(order.resource, offer_object.amount);
        //                         // We consumed the hole amount of the offer and must therefore remove it from the market
        //                         self.offers.remove(&offer_handle);
        //                         // The prices might have changed, we need to update the index
        //                         self.update_price_index();
        //                     } else {
        //                         // Give resources to company
        //                         companies[order.company]
        //                             .stock
        //                             .add_to_stock(order.resource, order.amount);
        //                         // We consumed the hole amount of the order and must therefore remove it from the market
        //                         self.orders.remove(&order_handle);
        //                         // The orders might have changed, we need to update the index
        //                         self.update_order_index();
        //                     }
        //                 }
        //             }
        //             None => {
        //                 break;
        //             }
        //         }
        //     }
        // }
    }

    pub fn tick(&mut self, companies: &mut Vec<Company>) {
        self.check_orders(companies);
        self.update_order_index();
    }
}
