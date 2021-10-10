use crate::stock::Stock;
use serde::{Deserialize, Serialize};

pub type PlayerHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub stock: Stock,
    pub currency: f64,
}
