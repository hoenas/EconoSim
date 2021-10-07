use log::info;

use crate::resource::ResourceHandle;
pub struct Stock {
    pub resources: Vec<f64>,
}

impl Stock {
    fn calculate_new_stock_value(&self, resource: ResourceHandle, amount: f64) -> f64 {
        let resource_in_stock = self.resources[resource];
        resource_in_stock - amount
    }

    pub fn check_resource_in_stock(&self, resource: ResourceHandle, amount: f64) -> bool {
        self.calculate_new_stock_value(resource, amount) >= 0.0
    }

    pub fn check_resources_in_stock(
        &self,
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
            self.resources[resource] = value_after_transaction;
            true
        } else {
            false
        }
    }

    pub fn add_to_stock(&mut self, resource: ResourceHandle, amount: f64) {
        self.resources[resource] += amount;
    }

    pub fn print_stock(&self) {
        for (resource, amount) in self.resources.iter().enumerate() {
            info!("Resource {}: {}", resource, amount);
        }
    }
}
