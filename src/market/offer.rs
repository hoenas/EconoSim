use crate::resource::ResourceHandle;
use crate::player::PlayerHandle;

pub type OfferHandle = usize;

pub struct Offer {
    pub resource: ResourceHandle,
    pub amount: f64,
    pub price_per_unit: f64,
    pub player: PlayerHandle,
}
