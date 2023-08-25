mod economy;
mod market;
mod persistence;
mod reinforcement_learning;
mod world;
mod world_data;

use format_num::NumberFormat;
use log::info;
use persistence::Persistence;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().env().init().unwrap();
    info!("=== SIM TEST ===");
    let num = NumberFormat::new();
    // Load world
    let mut trained_world = Persistence::load_prestine_world();
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
    for i in 0..1000 {
        info!("Episode {i}");
        let mut tmp_world = Persistence::load_prestine_world();
        for (i, company) in tmp_world.company_data.companies.iter_mut().enumerate() {
            company.trainer = trained_world.company_data.companies[i].trainer.clone();
        }
        trained_world = tmp_world;
        for k in 0..100000 {
            if k % 10000 == 0 {
                info!("Trainning progress: {k}");
            }
            trained_world.tick(true);
        }

        for k in 0..100000 {
            if k % 10000 == 0 {
                info!("Simulation progress: {k}");
            }
            trained_world.tick(false);
        }
        let mut tmp_world = Persistence::load_prestine_world();
        for (i, company) in tmp_world.company_data.companies.iter_mut().enumerate() {
            company.trainer = trained_world.company_data.companies[i].trainer.clone();
        }
        info!("Company value development:");
        for (i, company) in trained_world.company_data.companies.iter_mut().enumerate() {
            let delta = num.format(".4s", company.company_value - old_company_values[i]);
            let max_delta = num.format(".4s", company.company_value - max_company_values[i]);
            info!(
                "- {}:\t{}\t({delta})\t[{max_delta}]",
                company.name,
                num.format(".4s", company.company_value)
            );
            old_company_values[i] = company.company_value;
            if max_company_values[i] < company.company_value {
                max_company_values[i] = company.company_value;
            }
        }
        // trained_world.print_world_info();
    }
    Persistence::write_world(&trained_world);
}
