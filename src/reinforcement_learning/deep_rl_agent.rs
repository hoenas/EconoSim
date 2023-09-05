use neuroflow::activators::Type::Relu;
use neuroflow::FeedForward;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeepRLAgent {
    pub neural_network: FeedForward,
    action_dimensions: usize,
    discount: f64,
    last_action: usize,
}

impl DeepRLAgent {
    pub fn new(state_dimensions: i32, action_dimensions: i32, discount: f64) -> DeepRLAgent {
        let mut neural_network = FeedForward::new(&[state_dimensions, 10, 10, action_dimensions]);
        neural_network.learning_rate(0.1);
        neural_network.activation(Relu);
        DeepRLAgent {
            neural_network: neural_network,
            action_dimensions: action_dimensions as usize,
            discount: discount,
            last_action: 0,
        }
    }

    pub fn get_next_state_action(&mut self, state: Vec<f64>, exploration_factor: f64) -> usize {
        let mut rng = rand::thread_rng();
        if exploration_factor > rng.gen() {
            rng.next_u64() as usize % self.action_dimensions
        } else {
            let index_of_max: Option<usize> = self
                .neural_network
                .calc(&state)
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index);
            self.last_action = index_of_max.unwrap();
            self.last_action
        }
    }

    pub fn get_output(network: &mut FeedForward, state: &[f64]) -> Vec<f64> {
        network.calc(&state).iter().map(|x| *x).collect()
    }

    pub fn train(&mut self, old_state: Vec<f64>, reward: f64, new_state: Vec<f64>) {
        // Example implementation
        // https://github.com/valohai/qlearning-simple/blob/master/deep_gambler.py
        // Ask the model for the Q values of the old state (inference)
        let mut old_state_q_values = DeepRLAgent::get_output(&mut self.neural_network, &old_state);

        // Ask the model for the Q values of the new state (inference)
        let new_state_q_values = DeepRLAgent::get_output(&mut self.neural_network, &new_state);

        // Real Q value for the action we took. This is what we will train towards.
        let max_index = self.get_next_state_action(new_state, 0.0);
        old_state_q_values[self.last_action] =
            reward + self.discount * new_state_q_values[max_index];

        // Train
        self.neural_network
            .fit(&old_state, old_state_q_values.as_mut_slice());
    }
}
