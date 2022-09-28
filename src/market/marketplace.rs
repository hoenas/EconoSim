use crate::player::Player;
use crate::{market::offer::Offer, resource::ResourceHandle};

use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Marketplace {
    pub offers: Vec<Offer>,
}

impl Marketplace {
    pub fn new() -> Marketplace {
        Marketplace { offers: Vec::new() }
    }

    pub fn get_cheapest_offer(&self, resource: ResourceHandle) -> Option<&Offer> {
        let mut cheapest_offer: Option<&Offer> = None;
        for offer in self.offers.iter() {
            if offer.resource == resource {
                if cheapest_offer.is_none() {
                    cheapest_offer = Some(offer);
                } else if cheapest_offer.unwrap().price_per_unit > offer.price_per_unit {
                    cheapest_offer = Some(offer);
                }
            }
        }
        cheapest_offer
    }

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
