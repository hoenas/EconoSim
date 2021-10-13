use crate::processor::Processor;
use crate::stock::Stock;
use crate::worlddata::WorldData;
use serde::{Deserialize, Serialize};

pub type PlayerHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Player {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
    pub processors: Vec<Processor>,
}

impl Player {
    pub fn add_processor(&mut self, processor: Box<Processor>) {
        self.processors.push(*processor);
    }

    pub fn tick(&mut self) {
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock);
        }
    }
}
