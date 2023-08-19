mod economy;
mod market;
mod persistence;
mod world;
mod world_data;

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
    world.print_world_info();
    loop {
        world.tick();
        world.print_world_info();
        thread::sleep(periode);
    }
}
