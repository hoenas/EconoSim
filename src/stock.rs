use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

use crate::{resource::ResourceHandle, worlddata::WorldData};

#[derive(Serialize, Deserialize)]

pub struct Stock {
    pub resources: HashMap<ResourceHandle, f64>,
}

impl Stock {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    fn get_resource_value(&mut self, resource: ResourceHandle) -> f64 {
        match self.resources.get(&resource) {
            Some(value) => *value,
            None => {
                self.resources.insert(resource, 0.0);
                0.0
            }
        }
    }

    fn calculate_new_stock_value(&mut self, resource: ResourceHandle, amount: f64) -> f64 {
        let resource_in_stock = self.get_resource_value(resource);
        resource_in_stock - amount
    }

    pub fn check_resource_in_stock(&mut self, resource: ResourceHandle, amount: f64) -> bool {
        self.calculate_new_stock_value(resource, amount) >= 0.0
    }

    pub fn check_resources_in_stock(
        &mut self,
        resource_transactions: &Vec<(ResourceHandle, f64)>,
    ) -> bool {
        let mut in_stock: bool = true;
        for (resource, amount) in resource_transactions.iter() {
            in_stock &= self.check_resource_in_stock(*resource, *amount)
        }
        in_stock
    }

    pub fn make_transaction(&mut self, resource_transactions: &Vec<(ResourceHandle, f64)>) -> bool {
        if self.check_resources_in_stock(resource_transactions) {
            for (resource, amount) in resource_transactions.iter() {
                self.remove_from_stock_if_possible(*resource, *amount);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_from_stock_if_possible(&mut self, resource: ResourceHandle, amount: f64) -> bool {
        let value_after_transaction = self.calculate_new_stock_value(resource, amount);
        if value_after_transaction >= 0.0 {
            self.resources.insert(resource, value_after_transaction);
            true
        } else {
            false
        }
    }

    pub fn add_to_stock(&mut self, resource: ResourceHandle, amount: f64) {
        let new_value = self.get_resource_value(resource) + amount;
        self.resources.insert(resource, new_value);
    }

    pub fn print_stock(&self, world: &mut WorldData) {
        for (resource_handle, amount) in self.resources.values().enumerate() {
            let resource = world.get_resource_by_handle(resource_handle).unwrap();
            info!("Resource {}: {}", resource.name, amount);
        }
    }
}
