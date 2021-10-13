mod market;
mod player;
mod processor;
mod recipe;
mod resource;
mod stock;
mod world;

use log::info;
use player::Player;
use resource::Resource;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::{thread, time::Duration};
use world::World;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");

    // Create world
    let mut world: World = Default::default();
    // Create resources
    world.add_resource(Box::new(Resource {
        name: "Wood".to_string(),
    }));
    world.add_resource(Box::new(Resource {
        name: "Clay".to_string(),
    }));
    world.add_resource(Box::new(Resource {
        name: "Coal".to_string(),
    }));

    // Create player
    world.add_player(Box::new(Player {
        name: "Player1".to_string(),
        ..Default::default()
    }));

    let outfile = File::create("data/world.yml").unwrap();
    serde_yaml::to_writer(outfile, &world).unwrap();

    // Sim loop
    let periode = Duration::from_millis(1000);
    loop {
        info!("==========================================");
        world.tick();
        thread::sleep(periode);
    }
}
