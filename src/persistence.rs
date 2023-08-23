use crate::world::World;
use std::fs::File;
const START_FILENAME: &str = "data/init_world.yml";
const END_FILENAME: &str = "data/world.yml";

pub struct Persistence {}

impl Persistence {
    pub fn load_world() -> World {
        let infile = File::open(START_FILENAME).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn write_world(world: &World) {
        let outfile = File::create(END_FILENAME).unwrap();
        return serde_yaml::to_writer(outfile, world).unwrap();
    }
}
