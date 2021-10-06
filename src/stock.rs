use std::collections::HashMap;
use crate::resource::ResourceHandle;

pub struct Stock {
    resources: HashMap<ResourceHandle, f64>,
}

impl Stock {
    pub fn remove_from_stock_if_possible(&mut self, resource: ResourceHandle, amount: f64) -> bool {
        let resource_in_stock = self.resources[&resource];
        let value_after_transaction = resource_in_stock - amount;
        if value_after_transaction >= 0.0 {
            self.resources.insert(resource,value_after_transaction);
            true
        } else {
            false
        }
    }

    pub fn add_to_stock(&mut self, resource: ResourceHandle, amount: f64) {
        let resource_in_stock = self.resources[&resource];
        let value_after_transaction = resource_in_stock + amount;
        self.resources.insert(resource,value_after_transaction);
    }
}
