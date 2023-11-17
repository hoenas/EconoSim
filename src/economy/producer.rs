use crate::market::offer::UnprocessedOffer;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Producer {
    pub production: Vec<UnprocessedOffer>,
    pub offers: Vec<UnprocessedOffer>,
    pub offer_creation_ticks: usize,
    pub current_tick: usize,
}

impl Producer {
    pub fn new() -> Self {
        Self {
            production: vec![],
            offers: vec![],
            offer_creation_ticks: 100,
            current_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        // TODO: implement complex production behaviour
        if self.current_tick % self.offer_creation_ticks == 0 {
            self.current_tick = 0;
            for offer in self.production.iter() {
                self.offers.push(offer.clone());
            }
        }
        self.current_tick += 1;
    }
}
