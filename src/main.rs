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
    let mut world = World::new();
    // Create resources
    let wood = world.data.add_resource(Resource {
        name: "Wood".to_string(),
    });
    let clay = world.data.add_resource(Resource {
        name: "Clay".to_string(),
    });
    let coal = world.data.add_resource(Resource {
        name: "Coal".to_string(),
    });

    // Create player
    world.add_player(Player::new("Player1"));

    // Create recipe
    let recipe = Recipe {
        name: "Coal".to_string(),
        ingredients: [(wood, 0.5), (clay, 0.7)].to_vec(),
        products: [(coal, 0.2)].to_vec(),
        production_speed: 0.5,
    };
    let recipe_handle = world.data.add_recipe(recipe);
    // Create processor
    let processor = Processor {
        name: "Coal Pile".to_string(),
        recipe: recipe_handle,
        production_speed: 1.2,
        productive: true,
    };
    let player = world.get_player_by_handle(0).unwrap();
    player.add_processor(processor);
    player.stock.add_to_stock(wood, 1000.0);
    player.stock.add_to_stock(clay, 1000.0);
    let outfile = File::create("data/world.yml").unwrap();
    serde_yaml::to_writer(outfile, &world.data).unwrap();

    // Sim loop
    let periode = Duration::from_millis(500);
    loop {
        info!("==========================================");
        world.tick();
        thread::sleep(periode);
    }
}
