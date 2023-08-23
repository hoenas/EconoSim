use crate::economy::company::Company;
use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::market::offer::OfferHandle;
use crate::market::order::Order;
use crate::market::order::OrderHandle;
use crate::world_data::market_data::MarketData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Marketplace {
    next_offer_id: OfferHandle,
    next_order_id: OrderHandle,
}

impl Marketplace {
    pub fn new() -> Marketplace {
        Marketplace {
            next_offer_id: 0,
            next_order_id: 0,
        }
    }

    pub fn update_price_index(&self, market_data: &mut MarketData) {
        for resource_handle in 0..market_data.resource_count {
            let offer = self.get_cheapest_offer(resource_handle, &market_data.offers);
            market_data.price_index.insert(resource_handle, offer);
        }
    }

    pub fn update_order_index(&self, market_data: &mut MarketData) {
        for resource_handle in 0..market_data.resource_count {
            let order = self.get_highest_order(resource_handle, market_data);
            market_data.order_index.insert(resource_handle, order);
        }
    }

    pub fn get_cheapest_offer(
        &self,
        resource: ResourceHandle,
        offers: &HashMap<OfferHandle, Offer>,
    ) -> Option<(OfferHandle, f64)> {
        let mut cheapest_offer: Option<(OfferHandle, f64)> = None;
        for (offer_handle, offer) in offers.iter() {
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
    pub fn get_highest_order(
        &self,
        resource: ResourceHandle,
        market_data: &mut MarketData,
    ) -> Option<(OrderHandle, f64)> {
        let mut highest_order: Option<(OrderHandle, f64)> = None;
        for (order_handle, order) in market_data.orders.iter() {
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

    pub fn place_offer(
        &mut self,
        offer: Offer,
        market_data: &mut MarketData,
    ) -> Option<OfferHandle> {
        // Offer sanity checks
        if offer.amount <= 0.0 || offer.resource >= market_data.resource_count {
            return None;
        }
        self.next_offer_id += 1;
        market_data.offers.insert(self.next_offer_id, offer);
        self.update_price_index(market_data);
        Some(self.next_offer_id)
    }

    pub fn place_order(
        &mut self,
        order: Order,
        market_data: &mut MarketData,
    ) -> Option<OfferHandle> {
        // Order sanity checks
        if order.amount <= 0.0 || order.resource >= market_data.resource_count {
            return None;
        }
        self.next_order_id += 1;
        market_data.orders.insert(self.next_order_id, order);
        self.update_order_index(market_data);
        Some(self.next_order_id)
    }

    pub fn get_offer_by_handle(
        self,
        offer_handle: OfferHandle,
        market_data: &mut MarketData,
    ) -> Option<&Offer> {
        Some(&market_data.offers[&offer_handle])
    }

    pub fn get_order_by_handle(
        self,
        order_handle: OrderHandle,
        market_data: &mut MarketData,
    ) -> Option<&Order> {
        Some(&market_data.orders[&order_handle])
    }

    fn execute_orders(&self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
        // Check all orders
        for order in market_data.orders.values_mut() {
            // We are trying to fulfill the hole order
            while order.amount > 0.0 {
                match self.get_cheapest_offer(order.resource, &market_data.offers) {
                    Some(value) => {
                        let offer_handle = value.0;
                        let offer_price = value.1;
                        if offer_price <= order.max_price_per_unit {
                            match market_data.offers.get_mut(&offer_handle) {
                                Some(offer) => {
                                    if offer.amount < order.amount {
                                        // Consume offer
                                        order.amount -= offer.amount;
                                        // Give resources to company
                                        companies[order.company]
                                            .stock
                                            .add_to_stock(order.resource, offer.amount);
                                        // Give delta currency from max price back
                                        let price_delta = (order.max_price_per_unit
                                            - offer.price_per_unit)
                                            * offer.amount;
                                        companies[order.company].add_currency(price_delta);
                                        // Pay out offering company
                                        companies[offer.company]
                                            .add_currency(offer.amount * offer.price_per_unit);
                                        // We consumed the hole amount of the offer and must therefore remove it from the market
                                        market_data.offers.remove(&offer_handle);
                                    } else {
                                        // Give resources to ordering company
                                        companies[order.company]
                                            .stock
                                            .add_to_stock(order.resource, order.amount);
                                        // Give delta currency from max price back
                                        let price_delta = (order.max_price_per_unit
                                            - offer.price_per_unit)
                                            * order.amount;
                                        companies[order.company].add_currency(price_delta);
                                        // Pay out offering company
                                        companies[offer.company]
                                            .add_currency(offer.amount * order.amount);
                                        // Reduce offer and order amount
                                        offer.amount -= order.amount;
                                        order.amount = 0.0;
                                    }
                                }
                                None => {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    fn cleanup_orders(&self, market_data: &mut MarketData) {
        let mut complete_orders: Vec<OrderHandle> = vec![];
        for (order_handle, order) in market_data.orders.iter() {
            if order.amount <= 0.0 {
                complete_orders.push(*order_handle);
            }
        }
        for order_handle in complete_orders {
            market_data.orders.remove(&order_handle);
        }
    }

    pub fn tick(&self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
        self.execute_orders(market_data, companies);
        self.cleanup_orders(market_data);
        self.update_order_index(market_data);
    }
}
