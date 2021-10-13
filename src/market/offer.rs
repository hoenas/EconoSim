use crate::player::PlayerHandle;
use crate::resource::ResourceHandle;

use serde::{Deserialize, Serialize};

pub type OfferHandle = usize;

#[derive(Serialize, Deserialize, Default)]
pub struct Offer {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub price_per_unit: f64,
    pub player: PlayerHandle,
}
