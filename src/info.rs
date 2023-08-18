use log::info;

use crate::world::World;

pub struct Info {}

impl Info {
    pub fn print(world: &mut World) {
        info!("================================================================================");
        info!("WORLD STATUS ===================================================================");
        info!("Company Status:");
        for player in world.companies.iter() {
            info!("Company: {}", player.name);
            info!("Currency: {}", player.currency);
            player.stock.print_stock(&mut world.data);
            info!("");
        }
        info!("================================================================================");
        info!("Market offers:");
        for offer in world.data.market_place.offers.iter() {
            let player_name = world.get_company_name_by_handle(offer.1.company).unwrap();
            let resource_name = world
                .data
                .get_resource_name_by_handle(offer.1.resource)
                .unwrap();
            info!(
                " - Company {} offers {} units of {} @ {} credits/unit",
                player_name, offer.1.amount, resource_name, offer.1.price_per_unit
            );
        }
        info!("================================================================================");
    }
}
