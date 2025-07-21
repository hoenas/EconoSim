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

#[cfg(test)]
mod tests {

    #[test]
    fn tick() {
        use crate::{
            economy::producer::Producer,
            market::offer::{Offer, UnprocessedOffer},
        };

        let mut producer = Producer::new();
        let unprocessed_offer = UnprocessedOffer {
            amount: 10.0,
            price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        producer.production.push(unprocessed_offer);
        producer.offer_creation_ticks = 3;
        assert_eq!(producer.offers.len(), 0);
        producer.tick();
        assert_eq!(producer.offers.len(), 1);
        producer.tick();
        assert_eq!(producer.offers.len(), 1);
        producer.tick();
        assert_eq!(producer.offers.len(), 1);
        producer.tick();
        assert_eq!(producer.offers.len(), 2);
    }
}
