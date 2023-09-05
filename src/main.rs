use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use econo_sim::visualization::{render_diagrams, CompanyTrainingData, CompanyTrainingDatapoint};
use format_num::NumberFormat;
use simple_logger::SimpleLogger;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to load world from
    #[arg(short, long, default_value_t = String::from("data/init_world.yml"))]
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
    let epoche_length = 100000;
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

    for epoch in 0..1000 {
        // Record trainig data
        let mut training_data: Vec<CompanyTrainingData> = trained_world
            .company_data
            .companies
            .iter()
            .map(|x| CompanyTrainingData {
                name: x.name.clone(),
                data: Vec::new(),
            })
            .collect();

        for company in trained_world.company_data.companies.iter() {
            let mut processor_counts: Vec<usize> = (0..trained_world.recipe_data.recipes.len())
                .map(|_| 0)
                .collect();
            for processor in &company.processors {
                processor_counts[processor.recipe] += 1;
            }
            training_data[epoch].data.push(CompanyTrainingDatapoint {
                company_value: company.company_value,
                currency: company.currency,
                processor_counts: processor_counts,
                stock: company.old_state.stock.clone(),
            })
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
        for k in 0..epoche_length {
            if k % 10000 == 0 {
                log::info!("Trainning progress: {k}");
            }
            trained_world.tick(true, 1.0 / ((k + 1) as f64));
        }
        let mut fps = num.format(".4s", epoche_length as f64 / start.elapsed().as_secs_f64());
        log::info!("Trained with {} ticks/s", fps);
        start = Instant::now();
        for k in 0..epoche_length {
            if k % 10000 == 0 {
                log::info!("Simulation progress: {k}");
            }
            trained_world.tick(false, 0.0);
        }
        fps = num.format(".4s", epoche_length as f64 / start.elapsed().as_secs_f64());
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
        let duration = start.elapsed();
        for company in trained_world.company_data.companies.iter() {
            let mut processor_counts: Vec<usize> = (0..trained_world.recipe_data.recipes.len())
                .map(|_| 0)
                .collect();
            for processor in &company.processors {
                processor_counts[processor.recipe] += 1;
            }
            training_data[epoch].data.push(CompanyTrainingDatapoint {
                company_value: company.company_value,
                currency: company.currency,
                processor_counts: processor_counts,
                stock: company.old_state.stock.clone(),
            })
        }
        // trained_world.print_world_info();
        Persistence::write_world_to(&trained_world, &cli_args.out_file);
        log::info!("Storing diagram");
        render_diagrams(training_data, epoch);
    }
}
