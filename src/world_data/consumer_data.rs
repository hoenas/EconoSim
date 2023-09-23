use crate::economy::consumer::Consumer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConsumerData {
    pub consumers: Vec<Consumer>,
}

impl ConsumerData {
    pub fn new() -> ConsumerData {
        ConsumerData { consumers: vec![] }
    }
}
