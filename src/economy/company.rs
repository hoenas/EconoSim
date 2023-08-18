use log::debug;

use crate::economy::processor::Processor;
use crate::economy::recipe::RecipeHandle;
use crate::economy::resource::ResourceHandle;
use crate::economy::stock::Stock;
use crate::market::offer::UnprocessedOffer;
use crate::market::order::UnprocessedOrder;
use crate::worlddata::WorldData;
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
}

impl Company {
    pub fn new(name: &str) -> Self {
        Company {
            name: name.to_string(),
            stock: Stock::new(),
            currency: 0.0,
            processors: vec![],
            orders: vec![],
            offers: vec![],
        }
    }

    pub fn tick(&mut self, world: &WorldData) {
        debug!("Company: {} economy tick", self.name);
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock, world);
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
            resource: resource,
            amount: amount,
            max_price_per_unit: max_price_per_unit,
        });
    }

    pub fn place_offer(&mut self, resource: ResourceHandle, amount: f64, price_per_unit: f64) {
        if self.stock.remove_from_stock_if_possible(resource, amount) {
            self.offers.push(UnprocessedOffer {
                resource: resource,
                amount: amount,
                price_per_unit: price_per_unit,
            });
        }
    }
}
