mod market;
mod player;
mod processor;
mod recipe;
mod resource;
mod stock;
mod world;
mod worlddata;

use log::info;
use player::Player;
use processor::Processor;
use recipe::Recipe;
use resource::Resource;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::{thread, time::Duration};
use world::World;
use worlddata::WorldData;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");

    // Create world
    let infile = File::open("data/world.yml").unwrap();
    let mut world: World = serde_yaml::from_reader(infile).unwrap();

    let outfile = File::create("data/world2.yml").unwrap();
    serde_yaml::to_writer(outfile, &world).unwrap();

    // Sim loop
    let periode = Duration::from_millis(500);
    loop {
        info!("==========================================");
        world.tick();
        thread::sleep(periode);
    }
}
