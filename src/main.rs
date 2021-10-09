mod player;
mod processor;
mod recipe;
mod resource;
mod stock;

use resource::{Resource};
use simple_logger::SimpleLogger;
use std::{thread, time::Duration};

use log::info;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");
    // Create resources
    let mut resources = Vec::new();
    resources.push(Resource {
        name: "Wood".to_string()
    });
    resources.push(Resource {
        name: "Clay".to_string()
    });
    resources.push(Resource {
        name: "Coal".to_string()
    });
    // Create stock
    let mut my_resources = Vec::new();
    my_resources.push(1.0);
    my_resources.push(1.0);
    my_resources.push(0.0);
    let mut my_stock = stock::Stock {
        resources: my_resources,
    };
    // Create recipe
    let mut my_ingredients = Vec::new();
    my_ingredients.push((0, 0.05));
    my_ingredients.push((1, 0.02));
    let mut my_products = Vec::new();
    my_products.push((2, 0.025));
    let my_recipe = recipe::Recipe {
        ingredients: my_ingredients,
        production_speed: 0.1,
        products: my_products,
    };
    // Create processor
    let my_processor = processor::Processor {
        name: "Coal Pile".to_string(),
        production_speed: 0.1,
        recipe: my_recipe,
    };
    // Sim loop
    let periode = Duration::from_millis(1000);
    my_stock.print_stock();
    loop {
        info!("==========================================");
        my_processor.tick(&mut my_stock);
        my_stock.print_stock();
        thread::sleep(periode);
    }
}
