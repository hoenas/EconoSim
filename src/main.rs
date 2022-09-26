mod market;
mod player;
mod processor;
mod recipe;
mod resource;
mod stock;
mod world;
mod worlddata;

use log::info;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::{thread, time::Duration};
use world::World;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");

    // Read world
    let infile = File::open("data/world.yml").unwrap();
    let mut world: World = serde_yaml::from_reader(infile).unwrap();

    // Sim loop
    let periode = Duration::from_millis(500);
    loop {
        info!("==========================================");
        world.tick();
        thread::sleep(periode);
    }
}
