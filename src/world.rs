use crate::market::marketplace::Marketplace;
use crate::market::offer::Offer;
use crate::market::order::Order;
use crate::world_data::company_data::CompanyData;
use crate::world_data::market_data::MarketData;
use crate::world_data::processor_data::ProcessorData;
use crate::world_data::recipe_data::RecipeData;
use crate::world_data::resource_data::ResourceData;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub company_data: CompanyData,
    pub processor_data: ProcessorData,
    pub recipe_data: RecipeData,
    pub resource_data: ResourceData,
    pub market_data: MarketData,
    pub market_place: Marketplace,
}

impl World {
    pub fn new() -> World {
        World {
            company_data: CompanyData::new(),
            processor_data: ProcessorData::new(),
            recipe_data: RecipeData::new(),
            resource_data: ResourceData::new(),
            market_data: MarketData::new(),
            market_place: Marketplace::new(),
        }
    }

    pub fn print_world_info(&self) {
        for company in self.company_data.companies.iter() {
            info!("Company: {}", company.name);
            info!("Currency: {}", company.currency);
            company.stock.print_stock();
            info!("");
        }
        info!("================================================================================");
        info!("Market offers:");
        for offer in self.market_data.offers.iter() {
            let company_name = self
                .company_data
                .get_company_name_by_handle(offer.1.company)
                .unwrap();
            let resource_name = self
                .resource_data
                .get_resource_name_by_handle(offer.1.resource)
                .unwrap();
            info!(
                " - Company {} offers {} units of {} @ {} credits/unit",
                company_name, offer.1.amount, resource_name, offer.1.price_per_unit
            );
        }
        info!("================================================================================");
    }

    pub fn tick(&mut self) {
        // TODO: Shuffle iterator in order to avoid bias
        for (company_handle, company) in self.company_data.companies.iter_mut().enumerate() {
            company.tick(&self.recipe_data);
            // Create offers
            for offer in company.offers.iter_mut() {
                self.market_place.place_offer(
                    Offer {
                        resource: offer.resource,
                        amount: offer.amount,
                        price_per_unit: offer.price_per_unit,
                        company: company_handle,
                    },
                    &mut self.market_data,
                );
            }
            company.offers.clear();
            // Create orders
            for order in company.orders.iter_mut() {
                self.market_place.place_order(
                    Order {
                        resource: order.resource,
                        amount: order.amount,
                        max_price_per_unit: order.max_price_per_unit,
                        company: company_handle,
                    },
                    &mut self.market_data,
                );
            }
            company.orders.clear();
        }
        self.market_place
            .tick(&mut self.market_data, &mut self.company_data.companies);
    }
}
