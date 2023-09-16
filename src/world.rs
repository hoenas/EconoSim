use crate::market::marketplace::Marketplace;
use crate::market::offer::Offer;
use crate::market::order::Order;
use crate::reinforcement_learning::action::ActionSpace;
use crate::world_data::company_data::CompanyData;
use crate::world_data::consumer_data::ConsumerData;
use crate::world_data::market_data::MarketData;
use crate::world_data::processor_data::ProcessorData;
use crate::world_data::producer_data::ProducerData;
use crate::world_data::recipe_data::RecipeData;
use crate::world_data::resource_data::ResourceData;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub company_data: CompanyData,
    pub processor_data: ProcessorData,
    pub recipe_data: RecipeData,
    pub resource_data: ResourceData,
    pub producer_data: ProducerData,
    pub consumer_data: ConsumerData,
    pub market_data: MarketData,
    pub market_place: Marketplace,
    pub actionspace: ActionSpace,
}

impl World {
    pub fn new() -> World {
        World {
            company_data: CompanyData::new(),
            processor_data: ProcessorData::new(),
            recipe_data: RecipeData::new(),
            resource_data: ResourceData::new(),
            producer_data: ProducerData::new(),
            consumer_data: ConsumerData::new(),
            market_data: MarketData::new(0),
            market_place: Marketplace::new(),
            actionspace: ActionSpace::new(0, 0),
        }
    }

    pub fn print_world_info(&self) {
        for company in self.company_data.companies.iter() {
            info!("Company: {}", company.name);
            info!("Currency: {}", company.currency);
            info!("Value: {}", company.company_value);
            info!("Processors:");
            for processor in company.processors.iter() {
                info!(" - {}", processor.name);
            }
            company.stock.print_stock(&self.resource_data);
            info!("");
        }
        info!("================================================================================");
        info!("Market offers:");
        for offer in self.market_data.offers.iter() {
            let mut company_name: &str = "Producer";
            match offer.1.company {
                Some(company_handle) => {
                    company_name = self
                        .company_data
                        .get_company_name_by_handle(company_handle)
                        .unwrap();
                }
                None => {}
            }

            let resource_name = self
                .resource_data
                .get_resource_name_by_handle(offer.1.resource)
                .unwrap();
            info!(
                " - Company {} offers {} units of {} @ {} credits/unit",
                company_name, offer.1.amount, resource_name, offer.1.price_per_unit
            );
        }
        info!("================================================================================");
        info!("Market orders:");
        for order in self.market_data.orders.iter() {
            let mut company_name: &str = "Consumer";
            match order.1.company {
                Some(company_handle) => {
                    company_name = self
                        .company_data
                        .get_company_name_by_handle(company_handle)
                        .unwrap();
                }
                None => {}
            }
            let resource_name = self
                .resource_data
                .get_resource_name_by_handle(order.1.resource)
                .unwrap();
            info!(
                " - Company {} requests {} units of {} @ {} credits/unit max",
                company_name, order.1.amount, resource_name, order.1.max_price_per_unit
            );
        }
        info!("================================================================================");
    }

    fn update_producers(&mut self) {
        for producer in self.producer_data.producers.iter_mut() {
            producer.tick();
            for offer in producer.offers.iter_mut() {
                self.market_place.place_offer(
                    Offer {
                        resource: offer.resource,
                        amount: offer.amount,
                        price_per_unit: offer.price_per_unit,
                        company: None,
                        time_to_live: offer.time_to_live,
                    },
                    &mut self.market_data,
                );
            }
            producer.offers.clear();
        }
    }

    fn update_consumers(&mut self) {
        for consumer in self.consumer_data.consumers.iter_mut() {
            consumer.tick();
            for order in consumer.orders.iter_mut() {
                self.market_place.place_order(
                    Order {
                        resource: order.resource,
                        amount: order.amount,
                        max_price_per_unit: order.max_price_per_unit,
                        company: None,
                        time_to_live: order.time_to_live,
                    },
                    &mut self.market_data,
                );
            }
            consumer.orders.clear();
        }
    }

    fn update_companies(&mut self, train: bool, exploration_factor: f64) {
        // TODO: Shuffle iterator in order to avoid bias
        for (company_handle, company) in self.company_data.companies.iter_mut().enumerate() {
            company.tick(
                &self.recipe_data,
                &self.market_data,
                self.processor_data.processor_price,
                &self.actionspace,
                train,
                exploration_factor,
            );
            // Create offers
            for offer in company.offers.iter_mut() {
                if !company
                    .stock
                    .remove_from_stock_if_possible(offer.resource, offer.amount)
                {
                    continue;
                }
                self.market_place.place_offer(
                    Offer {
                        resource: offer.resource,
                        amount: offer.amount,
                        price_per_unit: offer.price_per_unit,
                        company: Some(company_handle),
                        time_to_live: offer.time_to_live,
                    },
                    &mut self.market_data,
                );
            }
            company.offers.clear();
            // Create orders
            for order in company.orders.iter_mut() {
                let order_price = order.max_price_per_unit * order.amount;
                if company.currency < order_price {
                    continue;
                }
                company.currency -= order_price;
                self.market_place.place_order(
                    Order {
                        resource: order.resource,
                        amount: order.amount,
                        max_price_per_unit: order.max_price_per_unit,
                        company: Some(company_handle),
                        time_to_live: order.time_to_live,
                    },
                    &mut self.market_data,
                );
            }
            company.orders.clear();
        }
    }

    pub fn tick(&mut self, train: bool, exploration_factor: f64) {
        // Update producers
        self.update_producers();
        // Update consumers
        self.update_consumers();
        // Update companies
        self.update_companies(train, exploration_factor);
        // Update market
        self.market_place
            .tick(&mut self.market_data, &mut self.company_data.companies);
    }
}
