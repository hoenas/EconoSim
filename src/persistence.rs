use crate::world::World;
use std::fs::File;
const PRESTINE_WORLD_FILENAME: &str = "data/init_world.yml";
const TRAINED_WORLD_FILENAME: &str = "data/world.yml";

pub struct Persistence {}

impl Persistence {
    pub fn load_prestine_world() -> World {
        let infile = File::open(PRESTINE_WORLD_FILENAME).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn load_trained_world() -> World {
        let infile = File::open(TRAINED_WORLD_FILENAME).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn write_world(world: &World) {
        let outfile = File::create(TRAINED_WORLD_FILENAME).unwrap();
        return serde_yaml::to_writer(outfile, world).unwrap();
    }
}
