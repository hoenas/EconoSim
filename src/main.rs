mod info;
mod market;
mod persistence;
mod player;
mod processor;
mod recipe;
mod resource;
mod stock;
mod world;
mod worlddata;

use info::Info;
use log::info;
use persistence::Persistence;
use simple_logger::SimpleLogger;
use std::{thread, time::Duration};

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");

    // Load world
    let mut world = Persistence::load_world();

    // Sim loop
    let periode = Duration::from_millis(500);
    loop {
        world.tick();
        Info::print(&mut world);
        thread::sleep(periode);
    }
}
