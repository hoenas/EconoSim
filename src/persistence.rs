use crate::world::World;
use std::fs::File;
const PRESTINE_WORLD_FILENAME: &str = "data/init_world.yml";
const TRAINED_WORLD_FILENAME: &str = "data/world.yml";

pub struct Persistence {}

impl Persistence {
    pub fn load_from<T>(filename: &str) -> T
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        log::info!("Loading {} from {}", std::any::type_name::<T>(), filename);
        let infile = File::open(filename).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn load_world_from(filename: &str) -> World {
        let infile = File::open(filename).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn load_prestine_world() -> World {
        return Persistence::load_world_from(PRESTINE_WORLD_FILENAME);
    }

    pub fn load_trained_world() -> World {
        let infile = File::open(TRAINED_WORLD_FILENAME).unwrap();
        return serde_yaml::from_reader(infile).unwrap();
    }

    pub fn write_world(world: &World) {
        return Persistence::write_world_to(world, TRAINED_WORLD_FILENAME);
    }

    pub fn write_world_to(world: &World, filename: &str) {
        let outfile = File::create(filename).unwrap();
        return serde_yaml::to_writer(outfile, world).unwrap();
    }
}
