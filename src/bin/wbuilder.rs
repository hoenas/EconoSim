use clap::{arg, command, Parser};
use econo_sim::{economy::company::Company, persistence::Persistence, world::World};
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of companies to generate
    #[arg(short, long, default_value_t = 1)]
    company_count: usize,
    /// Path to actionspace file
    #[arg(short, long, default_value_t =  String::from("data/actionspace.yml"))]
    actionspace_file: String,
    /// Path to consumer file
    #[arg(short, long, default_value_t =  String::from("data/consumer.yml"))]
    consumer_file: String,
    /// Path to market file
    #[arg(short, long, default_value_t =  String::from("data/market.yml"))]
    market_file: String,
    /// Path to marketplace file
    #[arg(short, long, default_value_t =  String::from("data/marketplace.yml"))]
    marketplace_file: String,
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
fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    log::info!("=== WORLD BUILDER ===");
    log::info!("Building world with {} companies", cli_args.company_count);
    let mut world = Persistence::load_prestine_world();
    // Load consumers data
    world.consumer_data = Persistence::load_from(&cli_args.consumer_file);
    // Load market data
    world.market_data = Persistence::load_from(&cli_args.market_file);
    // Load marketplace data
    world.market_place = Persistence::load_from(&cli_args.marketplace_file);
    // Load processor data
    world.processor_data = Persistence::load_from(&cli_args.processor_file);
    // Load producer data
    world.producer_data = Persistence::load_from(&cli_args.producer_file);
    // Load recipe data
    world.recipe_data = Persistence::load_from(&cli_args.recipes_file);
    // Load resource data
    world.resource_data = Persistence::load_from(&cli_args.resources_file);
    let resource_count = world.resource_data.resources.len();
    let actionspace_dimensions = world.actionspace.actions.len();
    let statespace_dimensions = world.company_data.companies[0].old_state.as_f64_vec().len();
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
        .collect();
    let reference_company = &world.company_data.companies[0];
    for company in companies.iter_mut() {
        company.stock.resources = reference_company.stock.resources.clone();
        company.currency = reference_company.currency;
        company.old_state = reference_company.old_state.clone();
    }
    world.company_data.companies = companies;
    // Save world
    Persistence::write_world_to(&world, &cli_args.out_file);
}
