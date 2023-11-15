use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to world to load agent from
    #[arg(short, long, default_value_t = String::from("data/trained_world.yml"))]
    in_file: String,
    /// Path world to copy agent to
    #[arg(short, long, default_value_t = String::from("data/trained_world.yml"))]
    out_file: String,
    /// Company handle top colpy agent from
    #[arg(short, long, default_value_t = 0 as usize)]
    company_handle: usize,
}

fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    let mut agent_world = Persistence::load_world_from(&cli_args.in_file);
    let serialized_agent = serde_yaml::to_string(
        &agent_world
            .company_data
            .get_company_by_handle(cli_args.company_handle)
            .unwrap()
            .agent,
    )
    .unwrap();
    let mut out_world = Persistence::load_world_from(&cli_args.out_file);
    for company in out_world.company_data.companies.iter_mut() {
        log::info!("Copying reference agent to {}", company.name);
        company.agent = serde_yaml::from_str(&serialized_agent).unwrap();
    }
    log::info!("Saving world to {}", cli_args.out_file);
    Persistence::write_world_to(&out_world, &cli_args.out_file);
}
