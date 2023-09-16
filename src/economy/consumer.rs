use crate::market::order::UnprocessedOrder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Consumer {
    pub consumption: Vec<UnprocessedOrder>,
    pub orders: Vec<UnprocessedOrder>,
    pub order_creation_ticks: usize,
    pub current_tick: usize,
}

impl Consumer {
    pub fn new() -> Self {
        Self {
            consumption: vec![],
            orders: vec![],
            order_creation_ticks: 1000,
            current_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        // TODO: implement complex need behaviour
        self.current_tick += 1;
        if self.current_tick == self.order_creation_ticks {
            for order in self.consumption.iter() {
                self.current_tick = 0;
                self.orders.push(order.clone());
            }
        }
    }
}
