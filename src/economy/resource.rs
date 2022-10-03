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

impl Resource {
    pub fn read_resources_from_file(path: String) -> Result<Vec<Resource>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the YAML contents
        let resources = serde_yaml::from_reader(reader)?;

        // Return the Vector of Resources
        Ok(resources)
    }
}
