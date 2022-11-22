use serde::{Deserialize, Serialize};
use std::usize;

pub type ResourceHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
}
