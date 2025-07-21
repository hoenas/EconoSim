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
        market_data.offers.get(&offer_handle)
    }

    pub fn get_order_by_handle(
        self,
        order_handle: OrderHandle,
        market_data: &mut MarketData,
    ) -> Option<&Order> {
        market_data.orders.get(&order_handle)
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
                                                    .add_resource_to_stock(
                                                        order.resource,
                                                        offer.amount,
                                                    );
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
                                                    .add_resource_to_stock(
                                                        order.resource,
                                                        order.amount,
                                                    );
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
                        .add_resource_to_stock(offer.resource, offer.amount);
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
        self.update_price_index(market_data);
    }
}

#[cfg(test)]
mod tests {
    use plotters::prelude::LogScalable;

    use super::Marketplace;
    use crate::economy::company::Company;
    use crate::market;
    use crate::market::offer::Offer;
    use crate::market::order::Order;
    use crate::world_data::market_data::MarketData;

    #[test]
    fn update_price_index() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        market_data.offers.insert(0, offer);
        marketplace.update_price_index(&mut market_data);

        assert_eq!(market_data.price_index.get(&0).unwrap().unwrap().0, 0);
        assert_eq!(market_data.price_index.get(&0).unwrap().unwrap().1, 100.0);
        assert!(market_data.price_index.get(&1).unwrap().is_none());
        assert!(market_data.price_index.get(&2).unwrap().is_none());
    }

    #[test]
    fn update_order_index() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let order = Order {
            company: Some(0),
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        market_data.orders.insert(0, order);
        marketplace.update_order_index(&mut market_data);

        assert_eq!(market_data.order_index.get(&0).unwrap().unwrap().0, 0);
        assert_eq!(market_data.order_index.get(&0).unwrap().unwrap().1, 100.0);
        assert!(market_data.order_index.get(&1).unwrap().is_none());
        assert!(market_data.order_index.get(&2).unwrap().is_none());
    }

    #[test]
    fn get_highest_order() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let order = Order {
            company: Some(0),
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut highest = order.clone();
        highest.max_price_per_unit = 99.0;
        market_data.orders.insert(0, order);
        market_data.orders.insert(1, highest);
        let returned_order = marketplace.get_highest_order(0, &mut market_data).unwrap();
        assert_eq!(returned_order.0, 0);
        assert_eq!(returned_order.1, 100.0);
    }

    #[test]
    fn get_cheapest_offer() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut cheaper_offer = offer.clone();
        cheaper_offer.price_per_unit = 99.0;
        market_data.offers.insert(0, offer);
        market_data.offers.insert(1, cheaper_offer);
        let returned_offer = marketplace
            .get_cheapest_offer(0, &market_data.offers)
            .unwrap();
        assert_eq!(returned_offer.0, 1);
        assert_eq!(returned_offer.1, 99.0);
    }

    #[test]
    fn place_offer() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let mut offer = Offer {
            company: Some(0),
            amount: -10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        // Adding a negative offer does not work
        assert!(marketplace
            .place_offer(offer.clone(), &mut market_data)
            .is_none());
        // Using a non existing resource does not work
        offer.amount = 10.0;
        offer.resource = 1;
        assert!(marketplace
            .place_offer(offer.clone(), &mut market_data)
            .is_none());
        // Adding a positive amount works
        offer.resource = 0;
        assert_eq!(
            marketplace
                .place_offer(offer.clone(), &mut market_data)
                .unwrap(),
            1
        );
        assert_eq!(marketplace.statistics.company_offers_placed, 1);
        // Adding an offer without a company does not increase statistics value
        // Offer handle is increased
        offer.company = None;
        assert_eq!(
            marketplace
                .place_offer(offer.clone(), &mut market_data)
                .unwrap(),
            2
        );
        assert_eq!(marketplace.statistics.company_offers_placed, 1);
    }

    #[test]
    fn place_order() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let mut order = Order {
            company: Some(0),
            amount: -10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        // Adding a negative offer does not work
        assert!(marketplace
            .place_order(order.clone(), &mut market_data)
            .is_none());
        // Using a non existing resource does not work
        order.amount = 10.0;
        order.resource = 1;
        assert!(marketplace
            .place_order(order.clone(), &mut market_data)
            .is_none());
        // Adding a positive amount works
        order.resource = 0;
        assert_eq!(
            marketplace
                .place_order(order.clone(), &mut market_data)
                .unwrap(),
            1
        );
        assert_eq!(marketplace.statistics.company_orders_placed, 1);
        // Adding an offer without a company does not increase statistics value
        // Offer handle is increased
        order.company = None;
        assert_eq!(
            marketplace
                .place_order(order.clone(), &mut market_data)
                .unwrap(),
            2
        );
        assert_eq!(marketplace.statistics.company_orders_placed, 1);
    }

    #[test]
    fn get_offer_by_handle() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };

        marketplace.place_offer(offer, &mut market_data);
        // Getting an existing offer works
        let existing_offer = marketplace.get_offer_by_handle(1, &mut market_data);
        assert!(existing_offer.is_some());
    }

    #[test]
    fn get_offer_by_handle_non_existing() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };

        marketplace.place_offer(offer, &mut market_data);
        // Getting an existing offer works
        let existing_offer = marketplace.get_offer_by_handle(2, &mut market_data);
        assert!(existing_offer.is_none());
    }

    #[test]
    fn get_order_by_handle() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let order = Order {
            company: Some(0),
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };

        marketplace.place_order(order, &mut market_data);
        // Getting an existing offer works
        let existing_order = marketplace.get_order_by_handle(1, &mut market_data);
        assert!(existing_order.is_some());
    }

    #[test]
    fn execute_orders() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(4);
        let mut companies: Vec<Company> = Vec::new();
        companies.push(Company::new("comp1", 0, 1, 0, 1, 1, 1.0));
        companies.push(Company::new("comp2", 1, 1, 0, 1, 1, 1.0));
        // Add offers
        let mut offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 50.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut offer_handle = 0;
        // Add offers of varying price range
        for i in 0..4 {
            offer.resource = i;
            for k in 1..4 {
                offer.price_per_unit = k.as_f64() * 50.0;
                market_data.offers.insert(offer_handle, offer.clone());
                offer_handle += 1;
            }
        }
        // Add orders
        // First order should be completely executed
        let mut order = Order {
            company: Some(1),
            amount: 5.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        market_data.orders.insert(0, order.clone());
        // Second order should be partly executed
        order.amount = 15.0;
        order.resource = 1;
        market_data.orders.insert(1, order.clone());
        // Third order should not be executed
        order.resource = 3;
        order.max_price_per_unit = 10.0;
        market_data.orders.insert(2, order.clone());
        marketplace.execute_orders(&mut market_data, &mut companies);
        marketplace.update_price_index(&mut market_data);
        marketplace.update_order_index(&mut market_data);
        // Check company 1 stats
        assert_eq!(companies[0].currency, 1250.0);
        assert!(companies[0].stock.resources.get(&0).is_none());
        assert!(companies[0].stock.resources.get(&1).is_none());
        assert!(companies[0].stock.resources.get(&2).is_none());
        // Check company 2 stats
        assert_eq!(companies[1].currency, 750.0);
        assert_eq!(*companies[1].stock.resources.get(&0).unwrap(), 5.0);
        assert_eq!(*companies[1].stock.resources.get(&1).unwrap(), 15.0);
        assert!(companies[1].stock.resources.get(&2).is_none());
        assert!(companies[1].stock.resources.get(&3).is_none());
        // Check open offers
        assert_eq!(market_data.price_index.get(&0).unwrap().unwrap().1, 50.0);
        assert_eq!(market_data.price_index.get(&1).unwrap().unwrap().1, 100.0);
        assert_eq!(market_data.price_index.get(&2).unwrap().unwrap().1, 50.0);
        assert_eq!(market_data.price_index.get(&3).unwrap().unwrap().1, 50.0);
        assert_eq!(market_data.offers.get(&0).unwrap().amount, 5.0);
        assert_eq!(market_data.offers.get(&1).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&2).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&4).unwrap().amount, 5.0);
        assert_eq!(market_data.offers.get(&5).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&6).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&7).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&8).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&9).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&10).unwrap().amount, 10.0);
        assert_eq!(market_data.offers.get(&11).unwrap().amount, 10.0);
        // Check open orders
        assert_eq!(market_data.order_index.get(&0).unwrap().unwrap().1, 100.0);
        assert_eq!(market_data.order_index.get(&1).unwrap().unwrap().1, 100.0);
        assert!(market_data.order_index.get(&2).unwrap().is_none());
        assert_eq!(market_data.order_index.get(&3).unwrap().unwrap().1, 10.0);
        assert_eq!(market_data.orders.get(&0).unwrap().amount, 0.0);
        assert_eq!(market_data.orders.get(&1).unwrap().amount, 0.0);
        assert_eq!(market_data.orders.get(&2).unwrap().amount, 15.0);
    }

    #[test]
    fn get_order_by_handle_non_existing() {
        let mut marketplace = Marketplace::new();
        let mut market_data = MarketData::new(1);
        let order = Order {
            company: Some(0),
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };

        marketplace.place_order(order, &mut market_data);
        // Getting an existing offer works
        let existing_order = marketplace.get_order_by_handle(2, &mut market_data);
        assert!(existing_order.is_none());
    }
    #[test]
    fn cleanup_complete_orders() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let order = Order {
            company: Some(0),
            amount: 100.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut other_order = order.clone();
        other_order.amount = 0.0;
        market_data.orders.insert(0, order);
        market_data.orders.insert(1, other_order);
        marketplace.cleanup_complete_orders(&mut market_data);
        assert_eq!(market_data.orders.len(), 1);
        assert!(market_data.orders.get(&0).is_some());
        assert!(market_data.orders.get(&1).is_none());
    }
    #[test]
    fn cleanup_dead_orders() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let mut companies: Vec<Company> = Vec::new();
        companies.push(Company::new("mycomp", 0, 1, 0, 1, 1, 1.0));
        let order = Order {
            company: Some(0),
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut other_order = order.clone();
        other_order.time_to_live = 1;
        market_data.orders.insert(0, order);
        market_data.orders.insert(1, other_order);
        marketplace.cleanup_dead_orders(&mut market_data, &mut companies);
        assert_eq!(market_data.orders.len(), 1);
        assert!(market_data.orders.get(&0).is_some());
        assert!(market_data.orders.get(&1).is_none());
    }
    #[test]
    fn cleanup_dead_offers() {
        let marketplace = Marketplace::new();
        let mut market_data = MarketData::new(3);
        let mut companies: Vec<Company> = Vec::new();
        companies.push(Company::new("mycomp", 0, 1, 0, 1, 1, 1.0));
        let offer = Offer {
            company: Some(0),
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        let mut other_offer = offer.clone();
        other_offer.time_to_live = 1;
        market_data.offers.insert(0, offer);
        market_data.offers.insert(1, other_offer);
        marketplace.cleanup_dead_offers(&mut market_data, &mut companies);
        assert_eq!(market_data.offers.len(), 1);
        assert!(market_data.offers.get(&0).is_some());
        assert!(market_data.offers.get(&1).is_none());
    }
}
