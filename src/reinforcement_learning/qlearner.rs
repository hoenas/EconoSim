use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CompanyState {
    // Stockpile
    stock: Vec<i64>,
    // Currentcy
    currency: i64,
    // TODO: production rates
    // Price and order index
    price_index: Vec<i64>,
    order_index: Vec<i64>,
    // Trade related state
    trade_resource: usize,
    trade_price: i64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum CompanyAction {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, i64, i64),
    SellResource(usize, i64, i64),
}

pub struct QLearner {
    Q: HashMap<CompanyState, HashMap<CompanyAction, f64>>,
    last_action: CompanyAction,
    last_state: CompanyState,
}

impl QLearner {
    pub fn new(state: CompanyState) -> Self {
        QLearner {
            Q: HashMap::new(),
            last_action: CompanyAction::Nothing,
            last_state: state,
        }
    }

    pub fn get_action(state: CompanyState) -> CompanyAction {
        // self.last_action = ...
        CompanyAction::Nothing
    }

    pub fn update_q(&mut self, new_state: CompanyState, reward: f64) {
        // ...
        self.last_state = new_state;
    }
}
