use std::vec;

use neuroflow::activators::Type::Relu;
use neuroflow::FeedForward;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub q_network: FeedForward,
    pub target_network: FeedForward,
    action_dimensions: usize,
    discount: f64,
    last_action: usize,
    experience_replay_batch_size: usize,
    experience_buffer_length: usize,
    pub experience_buffer: Vec<Experience>,
    target_network_update: usize,
    pub target_network_update_tick: usize,
}

impl DeepRLAgent {
    pub fn new(
        state_dimensions: i32,
        action_dimensions: i32,
        discount: f64,
        experience_replay_batch_size: usize,
        experience_buffer_length: usize,
        q_update_ticks: usize,
    ) -> DeepRLAgent {
        let mut q_network =
            FeedForward::new(&[state_dimensions, 2 * action_dimensions, action_dimensions]);
        q_network.learning_rate(0.00025);
        q_network.momentum(0.95);
        q_network.activation(Relu);

        let mut target_network =
            FeedForward::new(&[state_dimensions, 2 * action_dimensions, action_dimensions]);
        target_network.learning_rate(0.00025);
        target_network.momentum(0.95);
        target_network.activation(Relu);

        let mut agent = DeepRLAgent {
            q_network: q_network,
            target_network: target_network,
            action_dimensions: action_dimensions as usize,
            discount: discount,
            last_action: 0,
            experience_replay_batch_size,
            experience_buffer_length: experience_buffer_length,
            experience_buffer: vec![],
            target_network_update: q_update_ticks,
            target_network_update_tick: 0,
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
        let selection = rand::thread_rng().gen_range(0.0..1.0);
        if exploration_factor > selection {
            self.last_action = rand::random::<usize>() % self.action_dimensions;
        } else {
            let (index_of_max, _) = DeepRLAgent::get_max(&self.q_network.calc(&state));
            self.last_action = index_of_max;
        }
        self.last_action
    }

    fn update_q(&mut self) {
        let serialized = serde_yaml::to_string(&self.q_network).unwrap();
        self.target_network = serde_yaml::from_str(&serialized).unwrap();
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
        if self.experience_buffer.len() >= self.experience_replay_batch_size {
            // Train
            for experience in self
                .experience_buffer
                .choose_multiple(&mut rand::thread_rng(), self.experience_replay_batch_size)
            {
                // Ask the model for the Q values of the old state (inference)
                let mut policy_q_values = self.q_network.calc(&experience.old_state).to_vec();
                let target_q_values = self.target_network.calc(&experience.new_state).to_vec();
                let max_action_reward = DeepRLAgent::get_max(&target_q_values).1;
                // Update Q values for training
                // https://github.com/rajibhossen/dqn-examples/blob/master/mountain-car-dqn.py#L160C62-L160C69
                let mut target_q_value = experience.reward + self.discount * max_action_reward;
                // Limit to [-1,1]
                target_q_value = target_q_value.max(-1.0);
                target_q_value = target_q_value.min(1.0);
                policy_q_values[experience.action] = target_q_value;
                // Train
                self.q_network.fit(&experience.old_state, &policy_q_values);
            }
        }
        self.target_network_update_tick += 1;
        if (self.target_network_update_tick + 1) % self.target_network_update == 0 {
            self.target_network_update_tick = 0;
            self.update_q();
        }
    }
}

impl Clone for DeepRLAgent {
    fn clone(&self) -> Self {
        let serialized_q_network = serde_yaml::to_string(&self.q_network).unwrap();
        let q_network = serde_yaml::from_str(&serialized_q_network).unwrap();
        let serialized_target_network = serde_yaml::to_string(&self.q_network).unwrap();
        let target_network = serde_yaml::from_str(&serialized_target_network).unwrap();
        DeepRLAgent {
            q_network: q_network,
            target_network: target_network,
            action_dimensions: self.action_dimensions.clone(),
            discount: self.discount.clone(),
            last_action: self.last_action.clone(),
            experience_replay_batch_size: self.experience_replay_batch_size.clone(),
            experience_buffer_length: self.experience_buffer_length.clone(),
            experience_buffer: self.experience_buffer.clone(),
            target_network_update: self.target_network_update.clone(),
            target_network_update_tick: self.target_network_update_tick.clone(),
        }
    }
}
