use crate::economy::company::Company;
use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::market::offer::OfferHandle;
use crate::market::order::Order;
use crate::market::order::OrderHandle;
use crate::world_data::market_data::MarketData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MarketplaceStatistics {
    pub company_orders_placed: usize,
    pub company_offers_placed: usize,
    pub company_orders_partly_fulfilled: usize,
    pub company_offers_partly_fulfilled: usize,
    pub company_orders_fulfilled: usize,
    pub company_offers_fulfilled: usize,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Marketplace {
    pub statistics: MarketplaceStatistics,
    next_offer_id: OfferHandle,
    next_order_id: OrderHandle,
}

impl Marketplace {
    pub fn new() -> Marketplace {
        Marketplace {
            statistics: MarketplaceStatistics::default(),
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
        if offer.company.is_some() {
            self.statistics.company_offers_placed += 1;
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
        if order.company.is_some() {
            self.statistics.company_orders_placed += 1;
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

    fn execute_orders(&mut self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
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
                                        // Offer will be consumed
                                        // Order will be partly finished
                                        // Consume offer
                                        order.amount -= offer.amount;
                                        // Check if the order was created by a real company
                                        match order.company {
                                            // Give resources to company
                                            Some(ordering_company) => {
                                                companies[ordering_company]
                                                    .stock
                                                    .add_to_stock(order.resource, offer.amount);
                                                // Give delta currency from max price back
                                                let price_delta = (order.max_price_per_unit
                                                    - offer.price_per_unit)
                                                    * offer.amount;
                                                companies[ordering_company]
                                                    .add_currency(price_delta);
                                                self.statistics.company_orders_partly_fulfilled +=
                                                    1;
                                            }
                                            None => {
                                                // Order was created by a consumer
                                                // No company to add resources to and remove currency from
                                            }
                                        }
                                        // Pay out offering company if it exists
                                        match offer.company {
                                            Some(offering_company) => {
                                                companies[offering_company].add_currency(
                                                    offer.price_per_unit * offer.amount,
                                                );
                                                self.statistics.company_offers_fulfilled += 1;
                                            }
                                            None => {
                                                // Offer was created by a producer
                                                // No company to add currency to
                                            }
                                        }

                                        // We consumed the hole amount of the offer and must therefore remove it from the market
                                        market_data.offers.remove(&offer_handle);
                                    } else {
                                        // Offer will be partly consumed
                                        // Order will be finished
                                        // Check if the order was created by a real company
                                        match order.company {
                                            Some(ordering_company) => {
                                                // Give resources to ordering company
                                                companies[ordering_company]
                                                    .stock
                                                    .add_to_stock(order.resource, order.amount);
                                                // Give delta currency from max price back
                                                let price_delta = (order.max_price_per_unit
                                                    - offer.price_per_unit)
                                                    * order.amount;
                                                companies[ordering_company]
                                                    .add_currency(price_delta);
                                                self.statistics.company_orders_fulfilled += 1;
                                            }
                                            None => {
                                                // Order was created by a consumer
                                                // No company to add resources to and remove currency from
                                            }
                                        }
                                        // Pay out offering company if it exists
                                        match offer.company {
                                            Some(offering_company) => {
                                                companies[offering_company].add_currency(
                                                    offer.price_per_unit * order.amount,
                                                );
                                                self.statistics.company_offers_partly_fulfilled +=
                                                    1;
                                            }
                                            None => {
                                                // Offer was created by a producer
                                                // No company to add currency to
                                            }
                                        }
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

    fn cleanup_complete_orders(&self, market_data: &mut MarketData) {
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

    fn cleanup_dead_orders(&self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
        let mut complete_orders: Vec<OrderHandle> = vec![];
        for (order_handle, order) in market_data.orders.iter_mut() {
            order.time_to_live -= 1;
            if order.time_to_live > 0 {
                continue;
            }
            // Pay back ordering company
            match order.company {
                Some(company) => {
                    companies[company].add_currency(order.max_price_per_unit * order.amount);
                }
                None => {}
            }
            complete_orders.push(*order_handle);
        }
        for order_handle in complete_orders {
            market_data.orders.remove(&order_handle);
        }
    }

    fn cleanup_dead_offers(&self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
        let mut complete_offers: Vec<OrderHandle> = vec![];
        for (offer_handle, offer) in market_data.offers.iter_mut() {
            offer.time_to_live -= 1;
            if offer.time_to_live > 0 {
                continue;
            }
            // Give back resources to offering company
            match offer.company {
                Some(company) => {
                    companies[company]
                        .stock
                        .add_to_stock(offer.resource, offer.amount);
                }
                None => {}
            }
            complete_offers.push(*offer_handle);
        }

        for order_handle in complete_offers {
            market_data.offers.remove(&order_handle);
        }
    }

    pub fn tick(&mut self, market_data: &mut MarketData, companies: &mut Vec<Company>) {
        self.execute_orders(market_data, companies);
        self.cleanup_complete_orders(market_data);
        self.cleanup_dead_orders(market_data, companies);
        self.cleanup_dead_offers(market_data, companies);
        self.update_order_index(market_data);
    }
}
