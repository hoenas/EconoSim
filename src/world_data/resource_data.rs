use crate::economy::company::{Company, CompanyHandle};
use crate::economy::recipe::{Recipe, RecipeHandle};
use crate::economy::resource::{Resource, ResourceHandle};
use crate::market::marketplace::Marketplace;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResourceData {
    pub resources: Vec<Resource>,
}

impl ResourceData {
    pub fn new() -> ResourceData {
        ResourceData {
            resources: Vec::new(),
        }
    }

    pub fn get_resource_by_handle(
        &mut self,
        resource_handle: ResourceHandle,
    ) -> Option<&mut Resource> {
        if resource_handle < self.resources.len() {
            Some(&mut self.resources[resource_handle])
        } else {
            None
        }
    }

    pub fn get_resource_name_by_handle(&self, resource_handle: ResourceHandle) -> Option<&str> {
        if resource_handle < self.resources.len() {
            Some(&self.resources[resource_handle].name)
        } else {
            None
        }
    }
}
