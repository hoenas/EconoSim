use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum CompanyAction {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, usize),
    SellResource(usize, usize),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionSpace {
    pub actions: Vec<CompanyAction>,
}

impl ActionSpace {
    pub fn new(resource_count: usize, recipe_count: usize) -> ActionSpace {
        let mut actionspace: Vec<CompanyAction> = Vec::new();
        actionspace.push(CompanyAction::Nothing);
        for i in 0..recipe_count {
            actionspace.push(CompanyAction::BuyProcessor(i));
        }
        for i in 0..recipe_count {
            actionspace.push(CompanyAction::SellProcessor(i));
        }
        // TODO: Allow creation of offers / orders that are not according to best price policy
        for i in 1..resource_count {
            actionspace.push(CompanyAction::BuyResource(i, 5));
            actionspace.push(CompanyAction::SellResource(i, 5));
        }
        ActionSpace {
            actions: actionspace,
        }
    }
}
