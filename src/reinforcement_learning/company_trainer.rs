use crate::reinforcement_learning::state::CompanyState;
use rurel::mdp::Agent;
use rurel::strategy::explore::ExplorationStrategy;
use rurel::strategy::learn::LearningStrategy;
use rurel::strategy::terminate::TerminationStrategy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::state::CompanyAction;

/// An `AgentTrainer` can be trained for using a certain [Agent](mdp/trait.Agent.html). After
/// training, the `AgentTrainer` contains learned knowledge about the process, and can be queried
/// for this. For example, you can ask the `AgentTrainer` the expected values of all possible
/// actions in a given state.
#[derive(Serialize, Deserialize, Clone)]
pub struct CompanyTrainer {
    q: HashMap<CompanyState, HashMap<CompanyAction, f64>>,
    state: CompanyState,
}

impl CompanyTrainer {
    pub fn new(state: CompanyState) -> CompanyTrainer {
        CompanyTrainer {
            q: HashMap::new(),
            state: state,
        }
    }

    /// Fetches the learned values for the given state, by `Action`, or `None` if no value was
    /// learned.
    pub fn expected_values(&self, state: &CompanyState) -> Option<&HashMap<CompanyAction, f64>> {
        // XXX: make associated const with empty map and remove Option?
        self.q.get(state)
    }

    /// Fetches the learned value for the given `Action` in the given `State`, or `None` if no
    /// value was learned.
    pub fn expected_value(&self, state: &CompanyState, action: &CompanyAction) -> Option<f64> {
        self.q.get(state).and_then(|m| m.get(action).copied())
    }

    /// Returns a clone of the entire learned state to be saved or used elsewhere.
    pub fn export_learned_values(&self) -> HashMap<CompanyState, HashMap<CompanyAction, f64>> {
        self.q.clone()
    }

    // Returns a reference to the learned state.
    pub fn learned_values(&self) -> &HashMap<CompanyState, HashMap<CompanyAction, f64>> {
        &self.q
    }

    /// Imports a state, completely replacing any learned progress
    pub fn import_state(&mut self, q: HashMap<CompanyState, HashMap<CompanyAction, f64>>) {
        self.q = q;
    }

    /// Returns the best action for the given `State`, or `None` if no values were learned.
    pub fn best_action(&self, state: &CompanyState) -> Option<CompanyAction> {
        self.expected_values(state)
            .and_then(|m| {
                m.iter()
                    .max_by(|&(_, v1), &(_, v2)| v1.partial_cmp(v2).unwrap())
            })
            .map(|t| t.0.clone())
    }

    pub fn exploration_step(
        &mut self,
        state: CompanyState,
        agent: &mut dyn Agent<CompanyState>,
        exploration_strategy: &dyn ExplorationStrategy<CompanyState>,
    ) -> CompanyAction {
        self.state = state;
        exploration_strategy.pick_action(agent)
    }

    pub fn training_step(
        &mut self,
        reward: f64,
        agent: &mut dyn Agent<CompanyState>,
        learning_strategy: &dyn LearningStrategy<CompanyState>,
        termination_strategy: &mut dyn TerminationStrategy<CompanyState>,
        action: CompanyAction,
    ) -> bool {
        // current action value
        let r_t_next = reward;
        let v = {
            let old_value = self.q.get(&self.state).and_then(|m| m.get(&action));
            learning_strategy.value(&self.q.get(&self.state), &old_value, r_t_next)
        };

        self.q
            .entry(self.state.clone())
            .or_insert_with(HashMap::new)
            .insert(action, v);

        termination_strategy.should_stop(&self.state)
    }
}
