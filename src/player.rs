use log::debug;

use crate::economy::recipe::RecipeHandle;
use crate::economy::resource::ResourceHandle;
use crate::economy::stock::Stock;
use crate::worlddata::WorldData;
use crate::economy::processor::Processor;
use serde::{Deserialize, Serialize};

pub type PlayerHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
    pub processors: Vec<Processor>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            stock: Stock::new(),
            currency: 0.0,
            processors: vec![],
        }
    }

    pub fn tick(&mut self, world: &mut WorldData) {
        debug!("Player: {} tick", self.name);
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock, world);
        }
    }

    pub fn buy_processor(&mut self, recipe: RecipeHandle) {}

    pub fn sell_processor(&mut self, processor: Processor) {}

    pub fn buy_resource(&mut self, resource: ResourceHandle, amount: f64) {}

    pub fn sell_resource(&mut self, resource: ResourceHandle, amount: f64) {}
}
