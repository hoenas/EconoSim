use serde::{Deserialize, Serialize};

use crate::economy::company::{Company, CompanyHandle};
use crate::market::offer::Offer;
use crate::market::order::Order;
use crate::worlddata::WorldData;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub companies: Vec<Company>,
    pub data: WorldData,
}

impl World {
    pub fn new() -> World {
        World {
            companies: Vec::new(),
            data: WorldData::new(),
        }
    }

    pub fn tick(&mut self) {
        for (company_handle, company) in self.companies.iter_mut().enumerate() {
            company.tick(&self.data);
            // Create offers
            for offer in company.offers.iter_mut() {
                self.data.market_place.place_offer(Offer {
                    resource: offer.resource,
                    amount: offer.amount,
                    price_per_unit: offer.price_per_unit,
                    company: company_handle,
                });
            }
            company.offers.clear();
            // Create orders
            for order in company.orders.iter_mut() {
                self.data.market_place.place_order(Order {
                    resource: order.resource,
                    amount: order.amount,
                    max_price_per_unit: order.max_price_per_unit,
                    company: company_handle,
                });
            }
            company.orders.clear();
        }
        self.data.market_place.perform_paybacks(&mut self.companies);
    }

    pub fn add_company(&mut self, company: Company) -> CompanyHandle {
        self.companies.push(company);
        self.companies.len() - 1
    }

    pub fn get_company_by_handle(&mut self, company_handle: CompanyHandle) -> Option<&mut Company> {
        if company_handle < self.companies.len() {
            Some(&mut self.companies[company_handle])
        } else {
            None
        }
    }

    pub fn get_company_name_by_handle(&self, company_handle: CompanyHandle) -> Option<&str> {
        if company_handle < self.companies.len() {
            Some(&self.companies[company_handle].name)
        } else {
            None
        }
    }
}
