use std::vec;

use neuroflow::activators::Type::Relu;
use neuroflow::FeedForward;
use rand::{prelude::*, seq::index};
use rurel::strategy::learn::q;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Experience {
    pub old_state: Vec<f64>,
    pub action: usize,
    pub new_state: Vec<f64>,
    pub reward: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DeepRLAgent {
    pub neural_network: FeedForward,
    action_dimensions: usize,
    discount: f64,
    last_action: usize,
    experience_replay_ticks: usize,
    experience_buffer_length: usize,
    experience_buffer: Vec<Experience>,
}

impl DeepRLAgent {
    pub fn new(
        state_dimensions: i32,
        action_dimensions: i32,
        discount: f64,
        experience_replay_steps: usize,
        experience_buffer_length: usize,
    ) -> DeepRLAgent {
        let mut neural_network =
            FeedForward::new(&[state_dimensions, 2 * action_dimensions, action_dimensions]);
        neural_network.learning_rate(0.1);
        neural_network.activation(Relu);
        DeepRLAgent {
            neural_network: neural_network,
            action_dimensions: action_dimensions as usize,
            discount: discount,
            last_action: 0,
            experience_replay_ticks: experience_replay_steps,
            experience_buffer_length: experience_buffer_length,
            experience_buffer: vec![],
        }
    }

    fn get_max(values: &[f64]) -> (usize, f64) {
        let mut max = values[0];
        let mut index_of_max = 0;
        for (i, value) in values.iter().enumerate() {
            if *value > max {
                max = *value;
                index_of_max = i;
            }
        }
        (index_of_max, max)
    }

    pub fn get_next_state_action(&mut self, state: Vec<f64>, exploration_factor: f64) -> usize {
        if exploration_factor > rand::random::<f64>() {
            rand::random::<usize>() % self.action_dimensions
        } else {
            let (index_of_max, _) = DeepRLAgent::get_max(&self.neural_network.calc(&state));
            self.last_action = index_of_max;
            self.last_action
        }
    }

    pub fn get_output(network: &mut FeedForward, state: &[f64]) -> Vec<f64> {
        network.calc(&state).iter().map(|x| *x).collect()
    }

    pub fn train(&mut self, old_state: Vec<f64>, reward: f64, new_state: Vec<f64>, ticks: usize) {
        // Example implementation
        // https://github.com/valohai/qlearning-simple/blob/master/deep_gambler.py
        // Example implementation with experience replay
        // https://github.com/sudharsan13296/Hands-On-Reinforcement-Learning-With-Python/blob/master/08.%20Atari%20Games%20with%20DQN/8.8%20Building%20an%20Agent%20to%20Play%20Atari%20Games.ipynb

        // Add experience to buffer
        self.experience_buffer.push(Experience {
            old_state: old_state,
            action: self.last_action,
            new_state: new_state,
            reward: reward,
        });
        if self.experience_buffer.len() >= self.experience_buffer_length {
            self.experience_buffer.remove(0);
        }

        // Actual training
        if ticks % self.experience_replay_ticks == 0 {
            // Train the network with every experience in the buffer
            for experience in self
                .experience_buffer
                .iter()
                .choose_multiple(&mut rand::thread_rng(), self.experience_buffer_length)
            {
                // Ask the model for the Q values of the old state (inference)
                let mut old_state_q_values =
                    DeepRLAgent::get_output(&mut self.neural_network, &experience.old_state);
                // Ask the model for the Q values of the new state (inference)
                let new_state_q_values =
                    DeepRLAgent::get_output(&mut self.neural_network, &experience.new_state);
                // Real Q value for the action we took. This is what we will train towards.
                let (_, max) = DeepRLAgent::get_max(&new_state_q_values);
                old_state_q_values[experience.action] = reward + self.discount * max;

                // Train
                self.neural_network
                    .fit(&experience.old_state, old_state_q_values.as_mut_slice());
            }
        }
    }
}
