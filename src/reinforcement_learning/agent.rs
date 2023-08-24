use std::vec;

use crate::reinforcement_learning::state::CompanyAction;
use crate::reinforcement_learning::state::CompanyState;
use rurel::mdp::Agent;
use serde::{Deserialize, Serialize};

static DUMMY_STATE: CompanyState = CompanyState {
    stock: vec![],
    currency: 0,
    price_index: vec![],
    order_index: vec![],
};

#[derive(Serialize, Deserialize)]
pub struct CompanyAgent {}

impl Agent<CompanyState> for CompanyAgent {
    fn current_state(&self) -> &CompanyState {
        &DUMMY_STATE
    }
    fn take_action(&mut self, action: &CompanyAction) {
        match action {
            CompanyAction::Nothing => {
                // do nothing
            }
            CompanyAction::BuyProcessor(processor_handle) => {}
            CompanyAction::SellProcessor(processor_handle) => {}
            CompanyAction::BuyResource(resource_handle, amount, price) => {}
            CompanyAction::SellResource(resource_handle, amount, price) => {}
        }
    }
}
