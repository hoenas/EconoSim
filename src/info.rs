use log::info;

use crate::world::World;

pub struct Info {}

impl Info {
    pub fn print(world: &mut World) {
        info!("================================================================================");
        info!("WORLD STATUS ===================================================================");
        info!("Player Status:");
        for player in world.players.iter() {
            info!(" - Player: {}", player.name);
            player.stock.print_stock(&mut world.data);
        }
        info!("================================================================================");
        info!("Market offers:");
        for offer in world.data.market_place.offers.iter() {
            let player_name = world.get_player_name_by_handle(offer.player).unwrap();
            let resource_name = world
                .data
                .get_resource_name_by_handle(offer.resource)
                .unwrap();
            info!(
                " - Player {} offers {} units of {} @ {} credits/unit",
                player_name, offer.amount, resource_name, offer.price_per_unit
            );
        }
        info!("================================================================================");
    }
}