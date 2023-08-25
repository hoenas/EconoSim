use crate::market::offer::UnprocessedOffer;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Producer {
    production: Vec<UnprocessedOffer>,
    pub offers: Vec<UnprocessedOffer>,
    offer_creation_ticks: usize,
    current_tick: usize,
}

impl Producer {
    fn new() -> Self {
        Self {
            production: vec![],
            offers: vec![],
            offer_creation_ticks: 1000,
            current_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        // TODO: implement complex production behaviour
        self.current_tick += 1;
        if self.current_tick == self.offer_creation_ticks {
            self.current_tick = 0;
            for offer in self.production.iter() {
                self.offers.push(offer.clone());
            }
        }
    }
}
