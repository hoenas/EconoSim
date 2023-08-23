use rurel::mdp::State;

// Constants

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
    trade_price: i64
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ActionIdentifier {
    Nothing,
    BuyProcessor(usize),
    SellProcessor(usize),
    BuyResource(usize, i64, i64),
    SellResource(usize, i64, i64),
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CompanyAction {
    pub action: ActionIdentifier,
}

impl State for CompanyState {
    type A = CompanyAction;
    fn reward(&self) -> f64 {
        // TODO: Add reward function
        return 0.0
    }
    fn actions(&self) -> Vec<CompanyAction> {
        let mut actionspace: Vec<CompanyAction> = Vec::new();
        actionspace.push(CompanyAction{action: ActionIdentifier::Nothing});
        for i in 0..100 {
            actionspace.push(CompanyAction{action: ActionIdentifier::BuyProcessor(i)});
            actionspace.push(CompanyAction{action: ActionIdentifier::SellProcessor(i)});
        }
        for i in 0..100 {
            for k in 0..100 {
                for j in 0..100 {
                actionspace.push(CompanyAction{action: ActionIdentifier::BuyResource(i, k, j)});
                actionspace.push(CompanyAction{action: ActionIdentifier::SellResource(i, k, j)});
                }
        }}
        actionspace
    }
}
