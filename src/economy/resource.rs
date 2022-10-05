use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::usize;

pub type ResourceHandle = usize;

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
}
