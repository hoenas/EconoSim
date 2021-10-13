use crate::processor::Processor;
use crate::stock::Stock;
use crate::world::World;
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
    pub fn tick(&mut self) {
        for processor in self.processors.iter() {
            processor.tick(&mut self.stock);
        }
    }
}
