use clap::{arg, command, Parser};
use econo_sim::economy::company::Company;
use econo_sim::economy::consumer::Consumer;
use econo_sim::economy::processor::Processor;
use econo_sim::economy::producer::Producer;
use econo_sim::economy::recipe::Recipe;
use econo_sim::economy::stock::Stock;
use econo_sim::economy::{company, resource};
use econo_sim::market::marketplace::Marketplace;
use econo_sim::market::offer::UnprocessedOffer;
use econo_sim::market::order::UnprocessedOrder;
use econo_sim::persistence::Persistence;
use econo_sim::reinforcement_learning::action::ActionSpace;
use econo_sim::reinforcement_learning::state::CompanyState;
use econo_sim::world::World;
use econo_sim::world_data::consumer_data::ConsumerData;
use econo_sim::world_data::market_data::MarketData;
use econo_sim::world_data::producer_data::ProducerData;
use econo_sim::world_data::recipe_data::RecipeData;
use econo_sim::world_data::resource_data::ResourceData;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct CompanyStartingConditionsStock {
    pub resources: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize)]
struct CompanyStartingConditions {
    stock: CompanyStartingConditionsStock,
    currency: f64,
    processors: Vec<Processor>,
}

struct RenderedCompanyStartingConditions {
    stock: Stock,
    currency: f64,
    processors: Vec<Processor>,
}

#[derive(Serialize, Deserialize)]
struct RecipeInput {
    pub name: String,
    pub ingredients: HashMap<String, f64>,
    pub products: HashMap<String, f64>,
    pub production_speed: f64,
}

#[derive(Serialize, Deserialize)]
struct RecipeDataInput {
    pub recipes: Vec<RecipeInput>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UnrenderedUnprocessedOffer {
    pub resource: String,
    pub amount: f64,
    pub price_per_unit: f64,
    pub time_to_live: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ProducerInput {
    production: Vec<UnrenderedUnprocessedOffer>,
    pub offers: Vec<UnrenderedUnprocessedOffer>,
    offer_creation_ticks: usize,
    current_tick: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ProducerDataInput {
    producers: Vec<ProducerInput>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UnrenderedUnprocessedOrder {
    pub resource: String,
    pub amount: f64,
    pub max_price_per_unit: f64,
    pub time_to_live: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ConsumerInput {
    consumption: Vec<UnrenderedUnprocessedOrder>,
    pub orders: Vec<UnrenderedUnprocessedOrder>,
    order_creation_ticks: usize,
    current_tick: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ConsumerDataInput {
    consumers: Vec<ConsumerInput>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of companies to generate
    #[arg(short, long, default_value_t = 1)]
    company_count: usize,
    /// Path to company starting conditions file
    #[arg(short, long, default_value_t =  String::from("data/company.yml"))]
    company_starting_conditions_file: String,
    /// Path to consumer file
    #[arg(short, long, default_value_t =  String::from("data/consumer.yml"))]
    consumer_file: String,
    /// Path to processor file
    #[arg(short, long, default_value_t =  String::from("data/processor.yml"))]
    processor_file: String,
    /// Path to producer file
    #[arg(short, long, default_value_t =  String::from("data/producer.yml"))]
    producer_file: String,
    /// Path to recipes file
    #[arg(short, long, default_value_t =  String::from("data/recipes.yml"))]
    recipes_file: String,
    /// Path to resources file
    #[arg(short, long, default_value_t =  String::from("data/resources.yml"))]
    resources_file: String,
    /// Path to save generated world to
    #[arg(short, long, default_value_t =  String::from("data/generated_world.yml"))]
    out_file: String,
}

fn render_recipe_data(recipes_file: String, resource_data: &ResourceData) -> RecipeData {
    let unrendered: RecipeDataInput = Persistence::load_from(&recipes_file);
    // Create recipe data
    let mut recipe_data = RecipeData::new();
    for recipe in unrendered.recipes.iter() {
        let mut tmp_recipe = Recipe::new(recipe.name.clone(), recipe.production_speed);
        // Render ingredients
        for (resource, amount) in recipe.ingredients.iter() {
            tmp_recipe.ingredients.insert(
                resource_data.get_resource_handle_by_name(resource).unwrap(),
                *amount,
            );
        }
        // Render products
        for (resource, amount) in recipe.products.iter() {
            tmp_recipe.products.insert(
                resource_data.get_resource_handle_by_name(resource).unwrap(),
                *amount,
            );
        }
        recipe_data.recipes.push(tmp_recipe);
    }
    recipe_data
}

fn render_producer_data(producers_file: String, resource_data: &ResourceData) -> ProducerData {
    let unrendered: ProducerDataInput = Persistence::load_from(&producers_file);
    // Create producer data
    let mut producer_data = ProducerData::new();
    // Sync data
    for producer in unrendered.producers.iter() {
        let mut tmp_producer = Producer::new();
        tmp_producer.current_tick = producer.current_tick;
        tmp_producer.offer_creation_ticks = producer.offer_creation_ticks;
        // Create offers
        for offer in producer.offers.iter() {
            tmp_producer.offers.push(UnprocessedOffer {
                resource: resource_data
                    .get_resource_handle_by_name(&offer.resource)
                    .unwrap(),
                amount: offer.amount,
                price_per_unit: offer.price_per_unit,
                time_to_live: offer.time_to_live,
            });
        }
        // Create production
        for production in producer.production.iter() {
            tmp_producer.production.push(UnprocessedOffer {
                resource: resource_data
                    .get_resource_handle_by_name(&production.resource)
                    .unwrap(),
                amount: production.amount,
                price_per_unit: production.price_per_unit,
                time_to_live: production.time_to_live,
            });
        }
        producer_data.producers.push(tmp_producer);
    }
    producer_data
}

fn render_consumer_data(consumers_file: String, resource_data: &ResourceData) -> ConsumerData {
    let unrendered: ConsumerDataInput = Persistence::load_from(&consumers_file);
    // Create consumer data
    let mut consumer_data = ConsumerData::new();
    // Sync data
    for consumer in unrendered.consumers.iter() {
        let mut tmp_consumer = Consumer::new();
        tmp_consumer.current_tick = consumer.current_tick;
        tmp_consumer.order_creation_ticks = consumer.order_creation_ticks;
        // Create offers
        for order in consumer.orders.iter() {
            tmp_consumer.orders.push(UnprocessedOrder {
                resource: resource_data
                    .get_resource_handle_by_name(&order.resource)
                    .unwrap(),
                amount: order.amount,
                max_price_per_unit: order.max_price_per_unit,
                time_to_live: order.time_to_live,
            });
        }
        // Create production
        for consumption in consumer.consumption.iter() {
            tmp_consumer.consumption.push(UnprocessedOrder {
                resource: resource_data
                    .get_resource_handle_by_name(&consumption.resource)
                    .unwrap(),
                amount: consumption.amount,
                max_price_per_unit: consumption.max_price_per_unit,
                time_to_live: consumption.time_to_live,
            });
        }
        consumer_data.consumers.push(tmp_consumer);
    }
    consumer_data
}

fn render_company_starting_conditions(
    company_starting_conditions_file: String,
    resource_data: &ResourceData,
) -> RenderedCompanyStartingConditions {
    let unrendered: CompanyStartingConditions =
        Persistence::load_from(&company_starting_conditions_file);
    // Create stock
    let mut stock = Stock::new();
    for (resource, amount) in unrendered.stock.resources.iter() {
        stock.add_to_stock(
            resource_data.get_resource_handle_by_name(resource).unwrap(),
            *amount,
        )
    }
    RenderedCompanyStartingConditions {
        stock: stock,
        currency: unrendered.currency,
        processors: unrendered.processors,
    }
}

fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    log::info!("=== WORLD BUILDER ===");
    log::info!("Building world with {} companies", cli_args.company_count);
    let mut world = World::new();
    // Load resource data
    world.resource_data = Persistence::load_from(&cli_args.resources_file);
    let resource_count = world.resource_data.resources.len();
    // Load consumers data
    world.consumer_data = render_consumer_data(cli_args.consumer_file, &world.resource_data);
    // Create market data
    world.market_data = MarketData::new(resource_count);
    // Load marketplace data
    world.market_place = Marketplace::new();
    // Load processor data
    world.processor_data = Persistence::load_from(&cli_args.processor_file);
    // Load producer data
    world.producer_data = render_producer_data(cli_args.producer_file, &world.resource_data);
    // Load recipe data
    world.recipe_data = render_recipe_data(cli_args.recipes_file, &world.resource_data);
    // Adjust resource count
    world.market_data.resource_count = resource_count;
    // Load company starting conditions
    let company_starting_conditions = render_company_starting_conditions(
        cli_args.company_starting_conditions_file,
        &world.resource_data,
    );
    // Create actionspace
    let actionspace = ActionSpace::new(resource_count, world.recipe_data.recipes.len());
    world.actionspace = actionspace;
    let actionspace_dimensions = world.actionspace.actions.len();
    // Define start state
    let mut start_state = CompanyState::new(resource_count);
    for (resource, amount) in company_starting_conditions.stock.resources.iter() {
        start_state.stock[*resource] = *amount as usize;
    }
    start_state.currency = company_starting_conditions.currency as usize;
    // Define state dimenstions
    let statespace_dimensions = start_state.as_f64_vec().len();
    log::info!("Resource count: {}", resource_count);
    log::info!("Actionspace dimensions: {}", actionspace_dimensions);
    log::info!("Statespace dimensions: {}", statespace_dimensions);
    // Create companies
    let mut companies: Vec<Company> = (0..cli_args.company_count)
        .map(|x| {
            Company::new(
                &format!("Company {}", x),
                x,
                resource_count,
                statespace_dimensions as i32,
                actionspace_dimensions as i32,
                0.9,
            )
        })
        .map(|mut x| {
            x.stock = company_starting_conditions.stock.clone();
            x.processors = company_starting_conditions.processors.clone();
            x.currency = company_starting_conditions.currency;
            x.old_state = start_state.clone();
            x
        })
        .collect();
    // Add all resources to stock
    for company in companies.iter_mut() {
        for resource in 0..world.resource_data.resources.len() {
            company.stock.add_to_stock(resource, 0.0);
        }
    }
    world.company_data.companies = companies;
    // Save world
    Persistence::write_world_to(&world, &cli_args.out_file);
}
