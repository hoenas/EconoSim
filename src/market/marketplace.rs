use std::collections::HashMap;

use crate::economy::resource::ResourceHandle;
use crate::market::offer::Offer;
use crate::player::Player;

use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Marketplace {
    pub offers: Vec<Offer>,
    pub price_index: HashMap<ResourceHandle, Option<f64>>,
    pub resource_count: ResourceHandle,
}

impl Marketplace {
    pub fn new() -> Marketplace {
        Marketplace {
            offers: Vec::new(),
            price_index: HashMap::new(),
            resource_count: 0,
        }
    }

    pub fn update_price_index(&mut self) {
        for handle in 0..self.resource_count {
            let offer = self.get_cheapest_offer(handle);
            if offer.is_none() {
                self.price_index.insert(handle, None);
            } else {
                self.price_index
                    .insert(handle, Some(offer.unwrap().price_per_unit));
            }
        }
    }

    pub fn get_cheapest_offer(&self, resource: ResourceHandle) -> Option<&Offer> {
        let mut cheapest_offer: Option<&Offer> = None;
        for offer_handle in 0..self.offers.len() - 1 {
            let offer = self.get_offer_by_handle(offer_handle).unwrap();
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
        self.update_price_index();
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
            self.update_price_index();
        }
    }
}
