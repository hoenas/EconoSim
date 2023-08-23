mod economy;
mod market;
mod persistence;
mod reinforcement_learning;
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
        for i in 0..10000 {
            if i % 100 == 0 {
                info!("Progress: {}", i);
            }
            world.tick();
        }
        world.print_world_info();
        break;
        // thread::sleep(periode);
    }
    Persistence::write_world(&world);
}
