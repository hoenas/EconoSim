use crate::market::offer::Offer;
use crate::player::Player;

use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Marketplace {
    pub offers: Vec<Offer>,
}

impl Marketplace {
    pub fn place_offer(&mut self, offer: Offer) -> OfferHandle {
        self.offers.push(offer);
        self.offers.len() - 1
    }

    pub fn get_offer_by_handle(&self, offer_handle: OfferHandle) -> Option<&Offer> {
        if offer_handle < self.offers.len() {
            Some(&self.offers[offer_handle])
        } else {
            None
        }
    }

    pub fn accept_offer(&mut self, offer: &Offer, player: &mut Player, amount: f64) {
        if amount <= offer.amount {
            let price = offer.price_per_unit * amount;
            if price <= player.currency {
                player.currency -= price;
                player.stock.add_to_stock(offer.resource, amount);
            }
        }
    }
}
