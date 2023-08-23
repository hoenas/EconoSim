use rurel::mdp::Agent;
use crate::reinforcement_learning::state::CompanyState;
use crate::reinforcement_learning::state::CompanyAction;
use crate::reinforcement_learning::state::ActionIdentifier;


struct CompanyAgent {
    state: CompanyState,
}


impl Agent<CompanyState> for CompanyAgent {
    fn current_state(&self) -> &CompanyState {
        &self.state
    }
    fn take_action(&mut self, action: &CompanyAction) {
        match action.action {
            ActionIdentifier::Nothing => {
                // do nothing
            }
            ActionIdentifier::BuyProcessor(processor_handle) => {
                
            }
            ActionIdentifier::SellProcessor(processor_handle) => {
                
            }
            ActionIdentifier::BuyResource(resource_handle, amount, price) => {
                
            }
            ActionIdentifier::SellResource(resource_handle, amount, price) => {
                
            }
        }
    }
}

impl CompanyAgent {

}