use crate::reinforcement_learning::feed_forward::FeedForward;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec;

use burn::backend::Wgpu as B;

// Sources:
// https://artemoppermann.com/de/deep-q-learning/
// https://pytorch.org/tutorials/intermediate/reinforcement_q_learning.html
// https://huggingface.co/blog/deep-rl-dqn
// https://storage.googleapis.com/deepmind-media/dqn/DQNNaturePaper.pdf
// a': Possible actions

#[derive(Serialize, Deserialize, Clone)]
pub struct Experience {
    pub old_state: Vec<f64>,
    pub action: usize,
    pub new_state: Vec<f64>,
    pub reward: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DeepRLAgent {
    action_dimensions: usize,
    discount: f64,
    last_action: usize,
}

impl DeepRLAgent {
    pub fn new(state_dimensions: i32, action_dimensions: i32, discount: f64) -> DeepRLAgent {
        let mut agent = DeepRLAgent {
            action_dimensions: action_dimensions as usize,
            discount: discount,
            last_action: 0,
        };
        agent.update_q();
        agent
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
        // TODO: Reimplement with Burn
        return 0;
    }

    fn update_q(&mut self) {
        // TODO: Implement
    }

    pub fn train(&mut self, old_state: Vec<f64>, reward: f64, new_state: Vec<f64>, ticks: usize) {
        // Example implementation
        // https://github.com/valohai/qlearning-simple/blob/master/deep_gambler.py
        // Example implementation with experience replay
        // https://github.com/sudharsan13296/Hands-On-Reinforcement-Learning-With-Python/blob/master/08.%20Atari%20Games%20with%20DQN/8.8%20Building%20an%20Agent%20to%20Play%20Atari%20Games.ipynb
        // TODO: Reimplement with Burn
    }
}

impl Clone for DeepRLAgent {
    fn clone(&self) -> Self {
        DeepRLAgent {
            action_dimensions: self.action_dimensions.clone(),
            discount: self.discount.clone(),
            last_action: self.last_action.clone(),
        }
    }
}
