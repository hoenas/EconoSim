use clap::{arg, command, Parser};
use econo_sim::persistence::Persistence;
use simple_logger::SimpleLogger;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to load world from
    #[arg(short, long, default_value_t = String::from("data/trained_world.yml"))]
    world_file: String,
    /// Ticks to stop simulating after (0 is no limit)
    #[arg(short, long, default_value_t = 0)]
    max_ticks: usize,
    /// Ticks to print info after
    #[arg(short, long, default_value_t = 500)]
    info_ticks: usize,
    /// Frame rate limit (0 is no limit)
    #[arg(short, long, default_value_t = 0)]
    fps_limit: usize,
}

fn main() {
    let cli_args = Args::parse();
    let info_ticks = std::cmp::max(1, cli_args.info_ticks);
    SimpleLogger::new().init().unwrap();
    log::info!("=== SIMULATION ===");
    // Caclulate time per tick limit
    let mut tick_duration = Duration::from_secs(0);
    if cli_args.fps_limit > 0 {
        tick_duration = Duration::from_millis((1000.0 / cli_args.fps_limit as f64) as u64);
        log::info!(
            "Limiting simulation tickrate to {} ticks/s ({}ms per tick)",
            cli_args.fps_limit,
            tick_duration.as_millis()
        );
    }
    if cli_args.max_ticks > 0 {
        log::info!("Limiting simulation to {} ticks", cli_args.max_ticks);
    }
    let mut trained_world = Persistence::load_world_from(&cli_args.world_file);
    let mut ticks: usize = 0;
    loop {
        let start = Instant::now();

        trained_world.tick(false, 0.0);
        ticks += 1;
        if ticks % info_ticks == 0 {
            trained_world.print_world_info();
        }
        if start.elapsed() < tick_duration {
            std::thread::sleep(tick_duration - start.elapsed());
        }
        if cli_args.max_ticks != 0 && ticks >= cli_args.max_ticks {
            break;
        }
    }
}
