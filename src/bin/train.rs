use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use econo_sim::world::World;
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
    #[arg(long, default_value_t = 100)]
    training_ticks: usize,
    /// Epochs where the world is saved to the output file
    #[arg(long, default_value_t = 100)]
    save_epochs: usize,
    /// Exploration factor (if 0.0, an adaptive exploration factor will be used)
    #[arg(long, default_value_t = 0.0)]
    exploration_factor: f64,
    /// Minimum exploration factor to be used (only used if adaptive exploration factor is used)
    #[arg(long, default_value_t = 0.05)]
    min_exploration_factor: f64,
}

fn reset_world(world: &mut World, reference_world: &World) -> World {
    // Reset starting conditions
    let mut new_world = reference_world.clone();
    for i in 0..reference_world.company_data.companies.len() {
        let cloned_agent = world.company_data.companies[i].agent.clone();
        new_world.company_data.companies[i].agent.q_network = cloned_agent.q_network;
        new_world.company_data.companies[i].agent.target_network = cloned_agent.target_network;
        new_world.company_data.companies[i].agent.experience_buffer =
            cloned_agent.experience_buffer.clone();
        new_world.company_data.companies[i].agent.target_network_update_tick = cloned_agent.target_network_update_tick;
    }
    return new_world;
}

fn save_world(world: &World, out_file: &String) {
    log::info!("Saving world...");
    // Empty experience buffer before saving
    let mut cloned_world = world.clone();
    for company in cloned_world.company_data.companies.iter_mut() {
        company.agent.experience_buffer.clear();
    }
    Persistence::write_world_to(&cloned_world, out_file);
}

fn main() {
    let cli_args = Args::parse();
    SimpleLogger::new().init().unwrap();
    log::info!("=== TRAINING ===");
    let num = NumberFormat::new();
    let epochs = max(1, cli_args.epochs);
    let training_ticks = max(1, cli_args.training_ticks);
    let save_epochs = max(1, cli_args.save_epochs);
    let mut world_saved = false;
    // Load world
    let prestine_world = Persistence::load_world_from(&cli_args.in_file);
    let mut trained_world = Persistence::load_world_from(&cli_args.in_file);

    // Track training progress
    let writer_file = std::fs::File::create(&cli_args.training_out_file).unwrap();
    let mut csv_writer = csv::Writer::from_writer(writer_file);
    let mut title_column: Vec<String> = vec![];
    title_column.push(format!("Orders placed"));
    title_column.push(format!("Partly fulfilled orders"));
    title_column.push(format!("Fulfilled orders"));
    title_column.push(format!("Offers placed"));
    title_column.push(format!("Partly fulfilled offers"));
    title_column.push(format!("Fulfilled offers"));
    for company in trained_world.company_data.companies.iter() {
        title_column.push(format!("{} value", company.name));
        title_column.push(format!("{} processor count", company.name));
        title_column.push(format!("{} productive processor ticks", company.name));
    }
    title_column.insert(0, String::from("Epoch"));
    csv_writer.write_record(&title_column).unwrap();

    let mut exploration_factor = cli_args.exploration_factor;

    for epoch in 0..epochs {
        let training_percentage = ((epoch as f64 / epochs as f64) * 100.0).floor();
        world_saved = false;
        let start = Instant::now();

        if cli_args.exploration_factor == 0.0 {
            exploration_factor = 1.0 - (epoch as f64 / epochs as f64);
            exploration_factor = exploration_factor.max(cli_args.min_exploration_factor);
        }
        log::info!(
            "Epoch {epoch} ({training_percentage}%) | Exploration factor : {exploration_factor}"
        );
        for k in 0..training_ticks {
            if k % max(training_ticks / 100, 1) == 0 {
                print!(".");
                stdout().flush().unwrap();
            }
            trained_world.tick(true, exploration_factor, k);
        }
        println!();
        let fps = num.format(".4s", training_ticks as f64 / start.elapsed().as_secs_f64());
        log::info!("Trained with {} ticks/s", fps);

        log::info!("Writing training performance data...");
        let mut training_data: Vec<i64> = vec![];
        training_data.push(epoch as i64);
        training_data.push(trained_world.market_place.statistics.company_orders_placed as i64);
        training_data.push(
            trained_world
                .market_place
                .statistics
                .company_orders_partly_fulfilled as i64,
        );
        training_data.push(
            trained_world
                .market_place
                .statistics
                .company_orders_fulfilled as i64,
        );
        training_data.push(trained_world.market_place.statistics.company_offers_placed as i64);
        training_data.push(
            trained_world
                .market_place
                .statistics
                .company_offers_partly_fulfilled as i64,
        );
        training_data.push(
            trained_world
                .market_place
                .statistics
                .company_offers_fulfilled as i64,
        );
        for company in trained_world.company_data.companies.iter() {
            training_data.push(company.company_value.round() as i64);
            training_data.push(company.processors.len() as i64);
            training_data.push(company.productive_processor_ticks as i64);
        }
        csv_writer.serialize(training_data).unwrap();
        csv_writer.flush().unwrap();

        // Reset starting conditions
        trained_world = reset_world(&mut trained_world, &prestine_world);
        if epoch % save_epochs == 0 {
            save_world(&trained_world, &cli_args.out_file);
            world_saved = true;
        }
    }
    if !world_saved {
        save_world(&trained_world, &cli_args.out_file);
    }
}
