use crate::market::order::UnprocessedOrder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
            order_creation_ticks: 100,
            current_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        // TODO: implement complex need behaviour
        if self.current_tick % self.order_creation_ticks == 0 {
            for order in self.consumption.iter() {
                self.current_tick = 0;
                self.orders.push(order.clone());
            }
        }
        self.current_tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::economy::consumer;

    #[test]
    fn tick() {
        use crate::{economy::consumer::Consumer, market::order::UnprocessedOrder};

        let mut consumer = Consumer::new();
        let unprocessed_order = UnprocessedOrder {
            amount: 10.0,
            max_price_per_unit: 100.0,
            resource: 0,
            time_to_live: 100,
        };
        consumer.consumption.push(unprocessed_order);
        consumer.order_creation_ticks = 3;
        assert_eq!(consumer.orders.len(), 0);
        consumer.tick();
        assert_eq!(consumer.orders.len(), 1);
        consumer.tick();
        assert_eq!(consumer.orders.len(), 1);
        consumer.tick();
        assert_eq!(consumer.orders.len(), 1);
        consumer.tick();
        assert_eq!(consumer.orders.len(), 2);
    }
}
