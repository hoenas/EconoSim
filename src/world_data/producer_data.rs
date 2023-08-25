use crate::economy::producer::Producer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProducerData {
    pub producers: Vec<Producer>,
}

impl ProducerData {
    pub fn new() -> ProducerData {
        ProducerData { producers: vec![] }
    }
}
