use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessorData {
    pub processor_price: f64,
}

impl ProcessorData {
    pub fn new() -> ProcessorData {
        ProcessorData {
            processor_price: 1000.0,
        }
    }
}
