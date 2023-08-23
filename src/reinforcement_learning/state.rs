use rurel::mdp::State;
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

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum CompanyAction {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, usize, usize),
    SellResource(usize, usize, usize),
}

impl State for CompanyState {
    type A = CompanyAction;
    fn reward(&self) -> f64 {
        // TODO: Add reward function
        return 0.0;
    }
    fn actions(&self) -> Vec<CompanyAction> {
        let mut actionspace: Vec<CompanyAction> = Vec::new();
        actionspace.push(CompanyAction::Nothing);
        for i in 0..2 {
            actionspace.push(CompanyAction::BuyProcessor(i));
        }
        for i in 0..10 {
            actionspace.push(CompanyAction::SellProcessor(i));
        }
        for i in 0..5 {
            for k in 0..100 {
                for j in 0..100 {
                    actionspace.push(CompanyAction::BuyResource(i, k, j));
                    actionspace.push(CompanyAction::SellResource(i, k, j));
                }
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
}
