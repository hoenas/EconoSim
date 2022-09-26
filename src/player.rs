use log::debug;

use crate::processor::Processor;
use crate::stock::Stock;
use crate::worlddata::WorldData;
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

    pub fn add_processor(&mut self, processor: Processor) {
        self.processors.push(processor);
    }

    pub fn tick(&mut self, world: &mut WorldData) {
        debug!("Player: {} tick", self.name);
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock, world);
        }
    }
}
