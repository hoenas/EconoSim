use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum CompanyAction {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, usize, usize),
    SellResource(usize, usize, usize),
}

#[derive(Serialize, Deserialize)]
pub struct ActionSpace {
    pub actions: Vec<CompanyAction>,
}

impl ActionSpace {
    pub fn new(resource_count: usize) -> ActionSpace {
        let mut actionspace: Vec<CompanyAction> = Vec::new();
        actionspace.push(CompanyAction::Nothing);
        for i in 0..2 {
            actionspace.push(CompanyAction::BuyProcessor(i));
        }
        for i in 0..10 {
            actionspace.push(CompanyAction::SellProcessor(i));
        }
        for i in 1..resource_count {
            for k in 0..10 {
                let k_value = (2 as usize).pow(k);
                actionspace.push(CompanyAction::BuyResource(i, 5, k_value));
                actionspace.push(CompanyAction::SellResource(i, 5, k_value));
            }
        }
        ActionSpace {
            actions: actionspace,
        }
    }
}
