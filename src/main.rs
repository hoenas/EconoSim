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
    let mut world_data: WorldData = Default::default();
    // Create resources
    world_data.add_resource(Box::new(Resource {
        name: "Wood".to_string(),
    }));
    world_data.add_resource(Box::new(Resource {
        name: "Clay".to_string(),
    }));
    world_data.add_resource(Box::new(Resource {
        name: "Coal".to_string(),
    }));

    // Create player
    world_data.add_player(Box::new(Player {
        name: "Player1".to_string(),
        ..Default::default()
    }));

    // Create recipe
    let mut recipe = Recipe {
        name: "Coal".to_string(),
        ingredients: [(0, 0.5), (1, 0.7)].to_vec(),
        products: [(2, 0.2)].to_vec(),
        production_speed: 0.5,
    };

    // Create processor
    let mut processor = Box::new(Processor {
        name: "Coal Pile".to_string(),
        recipe: recipe,
        production_speed: 1.2,
        productive: true,
    });
    let mut player = world_data.get_player_by_handle(0).unwrap();
    player.add_processor(processor);
    let outfile = File::create("data/world.yml").unwrap();
    serde_yaml::to_writer(outfile, &world_data).unwrap();

    // Create world
    let mut world = World { data: world_data };

    // Sim loop
    let periode = Duration::from_millis(1000);
    loop {
        info!("==========================================");
        world.tick();
        thread::sleep(periode);
    }
}
