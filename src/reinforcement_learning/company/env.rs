use crate::economy::company::Company;
use rsrl::domains::{Domain, Observation, Reward};
use rsrl::spaces::real::Interval;
use rsrl::spaces::ProductSpace;

impl Domain for Company {
    type StateSpace = Interval;
    type ActionSpace = ProductSpace<Interval>;

    fn emit(&self) -> Observation<f64> {
        // Stockpile
        // let mut observation: Vec<f64> = Vec::new();
        // for resource_amount in self.stock.resources.values() {
        //     observation.push(resource_amount);
        // }
        // // Currentcy
        // observation.push(self.currency);
        // // TODO: production rates
        // // Price and order index
        // price_index: Vec<f64>,
        // order_index: Vec<f64>,
        // // Trade related state
        // trade_resource: f64,
        // trade_price: f64
        // Observation::Full(observation)
        if self.currency >= 1000000.0 {
            Observation::Terminal(self.currency)
        } else {
            Observation::Full(self.currency)
        }
    }

    fn step(&mut self, action: &Vec<f64>) -> (Observation<f64>, Reward) {
        // self.update_state(*action);

        // let to = self.emit();
        // let reward = if to.is_terminal() {
        //     REWARD_GOAL
        // } else {
        //     REWARD_STEP
        // };

        // (to, reward)
        (Observation::Full(self.currency), self.company_value_delta)
    }

    fn state_space(&self) -> Self::StateSpace {
        return Interval::bounded(0.0, 1000000.0);
    }

    fn action_space(&self) -> Self::ActionSpace {
        ProductSpace::empty()
            + Interval::bounded(0.0, 5.0)
            + Interval::bounded(0.0, 100.0)
            + Interval::bounded(0.0, 100.0)
            + Interval::bounded(0.0, 100.0)
    }
}
