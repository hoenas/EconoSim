use serde::{Deserialize, Serialize};

use crate::economy::company::{Company, CompanyHandle};
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
        for company in self.companies.iter_mut() {
            company.tick(&self.data);
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
