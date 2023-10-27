use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use format_num::NumberFormat;
use simple_logger::SimpleLogger;
use std::cmp::max;
use std::io::stdout;
use std::io::Write;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to load world from
    #[arg(short, long, default_value_t = String::from("data/generated_world.yml"))]
    in_file: String,
    /// Path to save trained world to
    #[arg(short, long, default_value_t = String::from("data/trained_world.yml"))]
    /// Path to save trained performance data to
    out_file: String,
    #[arg(short, long, default_value_t = String::from("data/training_performance.csv"))]
    training_out_file: String,
    /// Epochs to train
    #[arg(short, long, default_value_t = 10000)]
    epochs: usize,
    /// Ticks simulate with training
    #[arg(long, default_value_t = 10000)]
    training_ticks: usize,
    /// Epochs where the world is saved to the output file
    #[arg(long, default_value_t = 10)]
    save_epochs: usize,
    /// Epochs where a test simulation without learning is done
    #[arg(long, default_value_t = 10)]
    sim_epochs: usize,
    /// Ticks simulate without training
    #[arg(long, default_value_t = 10000)]
    sim_ticks: usize,
}

fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    log::info!("=== TRAINING ===");
    let num = NumberFormat::new();
    let epochs = max(1, cli_args.epochs);
    let training_ticks = max(1, cli_args.training_ticks);
    let save_epochs = max(1, cli_args.save_epochs);
    let sim_epochs = max(1, cli_args.sim_epochs);
    let sim_ticks = max(1, cli_args.sim_ticks);
    let mut world_saved = false;
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
        .map(|_| 0.0)
        .collect();

    // Track training progress
    let writer_file = std::fs::File::create(&cli_args.training_out_file).unwrap();
    let mut csv_writer = csv::Writer::from_writer(writer_file);
    let mut company_names = trained_world
        .company_data
        .companies
        .iter()
        .map(|company| company.name.clone())
        .collect::<Vec<String>>();
    company_names.insert(0, String::from("Epoch"));
    csv_writer.write_record(&company_names).unwrap();

    for epoch in 0..epochs {
        let training_percentage = ((epoch as f64 / epochs as f64) * 100.0).floor();
        log::info!("Epoch {epoch} ({training_percentage}%)");
        world_saved = false;
        let mut start = Instant::now();
        for k in 0..training_ticks {
            if k % max(training_ticks / 100, 1) == 0 {
                print!(".");
                stdout().flush().unwrap();
            }
            trained_world.tick(true, k as f64 / training_ticks as f64);
        }
        println!();
        let mut fps = num.format(".4s", training_ticks as f64 / start.elapsed().as_secs_f64());
        log::info!("Trained with {} ticks/s", fps);
        if epoch % sim_epochs == 0 {
            log::info!("Simulating...");
            start = Instant::now();
            for k in 0..sim_ticks {
                if k % max(sim_ticks / 100, 1) == 0 {
                    print!(".");
                    stdout().flush().unwrap();
                }
                trained_world.tick(false, 0.0);
            }
            println!();
            fps = num.format(".4s", sim_ticks as f64 / start.elapsed().as_secs_f64());
            log::info!("Simulated with {} ticks/s", fps);
            log::info!("Company value development:");
            for (i, company) in trained_world.company_data.companies.iter_mut().enumerate() {
                let delta = num.format(
                    ".4s",
                    (company.company_value - old_company_values[i]) as f64 / sim_ticks as f64,
                );
                let max_delta = num.format(
                    ".4s",
                    (company.company_value - max_company_values[i]) as f64 / sim_ticks as f64,
                );
                log::info!(
                    "- {}:\t{}\t({delta})\t[{max_delta}]",
                    company.name,
                    num.format(".4s", company.company_value / sim_ticks as f64)
                );
                old_company_values[i] = company.company_value;
                if max_company_values[i] < company.company_value {
                    max_company_values[i] = company.company_value;
                }
            }
            log::info!("Writing training performance data...");
            let mut dataset: Vec<i64> = trained_world
                .company_data
                .companies
                .iter()
                .map(|x| x.company_value.round() as i64)
                .collect();
            dataset.insert(0, epoch as i64);
            csv_writer.serialize(dataset).unwrap();
            csv_writer.flush().unwrap();
        }
        // Reset starting conditions
        for company in trained_world.company_data.companies.iter_mut() {
            let reference_company = &prestine_world.company_data.companies[company.id];
            company.stock = reference_company.stock.clone();
            company.currency = reference_company.currency;
            company.company_value = reference_company.company_value;
            company.old_company_value = reference_company.company_value;
            company.processors = reference_company.processors.clone();
            company.old_state = reference_company.old_state.clone();
            company.offers = vec![];
            company.orders = vec![];
        }
        if epoch % save_epochs == 0 {
            log::info!("Saving world...");
            Persistence::write_world_to(&trained_world, &cli_args.out_file);
            world_saved = true;
        }
    }
    if !world_saved {
        log::info!("Saving world...");
        Persistence::write_world_to(&trained_world, &cli_args.out_file);
    }
}
