use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use format_num::NumberFormat;
use simple_logger::SimpleLogger;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to load world from
    #[arg(short, long, default_value_t = String::from("data/generated_world.yml"))]
    in_file: String,
    /// Path to save trained world to
    #[arg(short, long, default_value_t = String::from("data/trained_world.yml"))]
    out_file: String,
}

fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    log::info!("=== SIM TEST ===");
    let num = NumberFormat::new();
    let epochs = 100000;
    // Load world
    let prestine_world = Persistence::load_world_from(&cli_args.in_file);
    let mut trained_world = Persistence::load_world_from(&cli_args.in_file);
    let mut old_company_values: Vec<f64> = trained_world
        .company_data
        .companies
        .iter()
        .map(|company| company.company_value)
        .collect();
    let mut max_company_values: Vec<f64> = trained_world
        .company_data
        .companies
        .iter()
        .map(|company| company.company_value)
        .collect();

    for epoch in 0..epochs {
        for company in trained_world.company_data.companies.iter() {
            let mut processor_counts: Vec<usize> = (0..trained_world.recipe_data.recipes.len())
                .map(|_| 0)
                .collect();
            for processor in &company.processors {
                processor_counts[processor.recipe] += 1;
            }
        }
        log::info!("Epoch {epoch}");
        // Reset starting conditions
        for company in trained_world.company_data.companies.iter_mut() {
            let reference_company = &prestine_world.company_data.companies[company.id];
            company.stock = reference_company.stock.clone();
            company.currency = reference_company.currency;
            company.company_value = reference_company.company_value;
            company.processors = reference_company.processors.clone();
        }
        let mut start = Instant::now();
        for k in 0..epoch + 1 {
            if k % 1000 == 0 {
                log::info!("Trainning progress: {k}");
            }
            trained_world.tick(true, k as f64 / (epoch + 1) as f64);
        }
        let mut fps = num.format(".4s", (epoch + 1) as f64 / start.elapsed().as_secs_f64());
        log::info!("Trained with {} ticks/s", fps);
        if epoch % 10 == 0 {
            log::info!("Simulating...");
            start = Instant::now();
            for k in 0..epoch + 1 {
                trained_world.tick(false, 0.0);
            }
            fps = num.format(".4s", (epoch + 1) as f64 / start.elapsed().as_secs_f64());
            log::info!("Simulated with {} ticks/s", fps);
            log::info!("Company value development:");
            for (i, company) in trained_world.company_data.companies.iter_mut().enumerate() {
                let delta = num.format(".4s", company.company_value - old_company_values[i]);
                let max_delta = num.format(".4s", company.company_value - max_company_values[i]);
                log::info!(
                    "- {}:\t{}\t({delta})\t[{max_delta}]",
                    company.name,
                    num.format(".4s", company.company_value)
                );
                old_company_values[i] = company.company_value;
                if max_company_values[i] < company.company_value {
                    max_company_values[i] = company.company_value;
                }
            }
        }
        Persistence::write_world_to(&trained_world, &cli_args.out_file);
    }
}
