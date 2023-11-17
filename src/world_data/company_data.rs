use crate::economy::company::{Company, CompanyHandle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CompanyData {
    pub companies: Vec<Company>,
}

impl CompanyData {
    pub fn new() -> CompanyData {
        CompanyData {
            companies: Vec::new(),
        }
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
