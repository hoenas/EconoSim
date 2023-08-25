use crate::reinforcement_learning::action::CompanyAction;
use serde::{Deserialize, Serialize};
// Constants

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct CompanyState {
    // Stockpile
    pub stock: Vec<usize>,
    // Currentcy
    pub currency: usize,
    // TODO: production rates
    // Price and order index
    pub price_index: Vec<usize>,
    pub order_index: Vec<usize>,
}

impl CompanyState {
    fn actions(&self) -> Vec<CompanyAction> {
        let mut actionspace: Vec<CompanyAction> = Vec::new();
        actionspace.push(CompanyAction::Nothing);
        for i in 0..2 {
            actionspace.push(CompanyAction::BuyProcessor(i));
        }
        for i in 0..10 {
            actionspace.push(CompanyAction::SellProcessor(i));
        }
        for i in 1..5 {
            for k in 0..10 {
                let k_value = (2 as usize).pow(k);
                actionspace.push(CompanyAction::BuyResource(i, 5, k_value));
                actionspace.push(CompanyAction::SellResource(i, 5, k_value));
            }
        }
        actionspace
    }
}

impl CompanyState {
    pub fn new(resource_count: usize) -> CompanyState {
        CompanyState {
            stock: vec![0; 100],
            currency: 0,
            price_index: (0..resource_count).collect(),
            order_index: (0..resource_count).collect(),
        }
    }

    pub fn as_f64_vec(&self) -> Vec<f64> {
        let mut return_value: Vec<f64> = vec![];
        let mut stock_vec: Vec<f64> = self.stock.iter().map(|x| *x as f64).collect();
        return_value.append(&mut stock_vec);
        return_value.push(self.currency as f64);
        let mut price_index_vec: Vec<f64> = self.price_index.iter().map(|x| *x as f64).collect();
        return_value.append(&mut price_index_vec);
        let mut order_index_vec: Vec<f64> = self.order_index.iter().map(|x| *x as f64).collect();
        return_value.append(&mut order_index_vec);
        return_value
    }
}
