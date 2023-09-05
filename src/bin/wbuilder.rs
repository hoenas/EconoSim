use clap::{arg, command, Parser};
use econo_sim::{economy::company::Company, persistence::Persistence, world::World};
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of companies to generate
    #[arg(short, long, default_value_t = 1)]
    company_count: usize,
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
