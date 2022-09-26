use crate::world::World;
use std::fs::File;
const FILENAME: &str = "data/world.yml";

pub struct Persistence {}

impl Persistence {
    pub fn load_world() -> World {
        let infile = File::open(FILENAME).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn write_world(world: &World) {
        let outfile = File::create(FILENAME).unwrap();
        return serde_yaml::to_writer(outfile, world).unwrap();
    }
}
