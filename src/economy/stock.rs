use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};

use crate::economy::resource::ResourceHandle;
use crate::world_data::resource_data::ResourceData;

#[derive(Serialize, Deserialize, Clone)]

pub struct Stock {
    pub resources: HashMap<ResourceHandle, f64>,
}

impl Stock {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    fn get_resource_amount_in_stock(&mut self, resource: ResourceHandle) -> f64 {
        match self.resources.get(&resource) {
            Some(value) => *value,
            None => {
                self.resources.insert(resource, 0.0);
                0.0
            }
        }
    }

    fn calculate_new_stock_value(&mut self, resource: ResourceHandle, amount: f64) -> f64 {
        let resource_in_stock = self.get_resource_amount_in_stock(resource);
        resource_in_stock - amount
    }

    pub fn check_resource_in_stock(&mut self, resource: ResourceHandle, amount: f64) -> bool {
        self.calculate_new_stock_value(resource, amount) >= 0.0 && amount >= 0.0
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

    pub fn remove_resource_from_stock_if_possible(
        &mut self,
        resource: ResourceHandle,
        amount: f64,
    ) -> bool {
        let value_after_transaction = self.calculate_new_stock_value(resource, amount);
        if value_after_transaction >= 0.0 {
            self.resources.insert(resource, value_after_transaction);
            true
        } else {
            false
        }
    }

    pub fn remove_resources_from_stock_if_possible(
        &mut self,
        resource_transactions: &Vec<(ResourceHandle, f64)>,
    ) -> bool {
        if self.check_resources_in_stock(resource_transactions) {
            for (resource, amount) in resource_transactions.iter() {
                self.remove_resource_from_stock_if_possible(*resource, *amount);
            }
            true
        } else {
            false
        }
    }

    pub fn add_resource_to_stock(&mut self, resource: ResourceHandle, amount: f64) {
        if amount < 0.0 {
            panic!("Cannot add amount smaller than zero!")
        }
        let new_value = self.get_resource_amount_in_stock(resource) + amount;
        self.resources.insert(resource, new_value);
    }

    pub fn print_stock(&self, resource_data: &ResourceData) {
        for (&resource_handle, amount) in self.resources.iter() {
            match resource_data.get_resource_name_by_handle(resource_handle) {
                Some(resource_name) => {
                    info!("Resource {}: {}", resource_name, amount);
                }
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Stock;
    use crate::economy::resource::ResourceHandle;

    #[test]
    fn get_resource_amount_in_stock_resource() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        // Existing
        assert_eq!(stock.get_resource_amount_in_stock(0), 10.0);
        // Not existing
        assert_eq!(stock.get_resource_amount_in_stock(1), 0.0);
    }

    #[test]
    fn calculate_new_stock_value_positive() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        // Positive
        assert_eq!(stock.calculate_new_stock_value(0, 5.0), 5.0);
        // Negative
        assert_eq!(stock.calculate_new_stock_value(0, 15.0), -5.0);
        // Zero
        assert_eq!(stock.calculate_new_stock_value(0, 10.0), 0.0);
    }

    #[test]
    fn check_resource_in_stock() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        // Available
        assert!(stock.check_resource_in_stock(0, 5.0));
        assert!(stock.check_resource_in_stock(0, 10.0));
        // Not available
        assert!(!stock.check_resource_in_stock(0, 15.0));
    }

    #[test]
    fn check_resources_in_stock() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        stock.resources.insert(1, 10.0);
        // Available
        let param_available: Vec<(ResourceHandle, f64)> = vec![(0, 10.0), (1, 10.0)];
        assert!(stock.check_resources_in_stock(&param_available));
        // Not available
        let param_not_available: Vec<(ResourceHandle, f64)> = vec![(0, 10.0), (1, 15.0)];
        assert!(stock.check_resources_in_stock(&param_not_available));
    }

    #[test]
    fn calculate_new_stock_value_zero() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        assert_eq!(stock.calculate_new_stock_value(0, 10.0), 0.0);
    }

    #[test]
    fn add_resource_to_stock() {
        let mut stock = Stock::new();
        stock.resources.insert(0, 10.0);
        stock.resources.insert(1, 10.0);
        // Add nothing
        stock.add_resource_to_stock(0, 0.0);
        assert_eq!(*stock.resources.get(&0).unwrap(), 10.0);
        assert_eq!(*stock.resources.get(&1).unwrap(), 10.0);
        // Add something
        stock.add_resource_to_stock(0, 5.0);
        assert_eq!(*stock.resources.get(&0).unwrap(), 15.0);
        assert_eq!(*stock.resources.get(&1).unwrap(), 10.0);
    }

    #[test]
    #[should_panic]
    fn add_resource_to_stock_subtract_something() {
        let mut stock = Stock::new();
        // Add resources
        stock.resources.insert(0, 10.0);
        stock.resources.insert(1, 10.0);
        // Actual test
        stock.add_resource_to_stock(1, -10.0);
    }
}
