use crate::economy::processor::Processor;
use crate::economy::recipe::RecipeHandle;
use crate::economy::resource::ResourceHandle;
use crate::economy::stock::Stock;
use crate::market::offer::UnprocessedOffer;
use crate::market::order::UnprocessedOrder;
use crate::world_data::recipe_data::RecipeData;
use crate::world_data::market_data::MarketData;
use log::debug;
use serde::{Deserialize, Serialize};

pub type CompanyHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
    pub processors: Vec<Processor>,
    pub orders: Vec<UnprocessedOrder>,
    pub offers: Vec<UnprocessedOffer>,
    pub company_value: f64,
    pub id: CompanyHandle,
}

impl Company {
    pub fn new(name: &str, company_handle: CompanyHandle) -> Self {
        Company {
            name: name.to_string(),
            stock: Stock::new(),
            currency: 0.0,
            processors: vec![],
            orders: vec![],
            offers: vec![],
            company_value: 0.0,
            id: company_handle,
        }
    }

    pub fn tick(&mut self, recipe_data: &RecipeData) {
        debug!("Company: {} economy tick", self.name);
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock, recipe_data);
        }
    }

    pub fn add_currency(&mut self, amount: f64) {
        self.currency += amount;
    }

    pub fn add_resource(&mut self, resource: ResourceHandle, amount: f64) {
        self.stock.add_to_stock(resource, amount);
    }

    // Methods to be used by an AI controller
    pub fn buy_processor(&mut self, recipe: RecipeHandle) {}

    pub fn sell_processor(&mut self, processor: Processor) {}

    pub fn place_order(&mut self, resource: ResourceHandle, amount: f64, max_price_per_unit: f64) {
        self.orders.push(UnprocessedOrder {
            resource,
            amount,
            max_price_per_unit,
        });
    }

    pub fn place_offer(&mut self, resource: ResourceHandle, amount: f64, price_per_unit: f64) {
        if self.stock.remove_from_stock_if_possible(resource, amount) {
            self.offers.push(UnprocessedOffer {
                resource,
                amount,
                price_per_unit,
            });
        }
    }

    pub fn update_company_value(&mut self, market_data: &MarketData, processor_value: f64) -> f64 {
        let mut new_company_value = self.currency;
        // Add value of all processors
        new_company_value += self.processors.len() as f64 * processor_value;
        // Add stockpile value
        for (resource, amount) in self.stock.resources.iter() {
            if market_data.price_index.contains_key(resource) {
                match market_data.price_index[resource] {
                    Some((_, price)) => {
                        new_company_value += *amount * price;
                    }
                    None => {
                        continue;
                    }
                };
            }
        }
        let old_company_value = self.company_value;
        // Add companies offers current value
        for order in market_data.offers.values() {
            if order.company != self.id {
                continue;
            }
            match market_data.price_index[&order.resource] {
                Some((_, price)) => {
                    new_company_value += order.amount * price;
                }
                None => {
                    break;
                }
            };
        }
        self.company_value = new_company_value;
        self.company_value - old_company_value
    }
}
