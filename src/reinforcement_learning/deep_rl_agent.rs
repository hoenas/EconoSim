use crate::reinforcement_learning::feed_forward::FeedForward;
use crate::reinforcement_learning::feed_forward::FeedForwardConfig;
use burn::module::Module;
use burn::prelude::*;
use burn::record::FullPrecisionSettings;
use burn::record::PrettyJsonFileRecorder;
use serde::{Deserialize, Serialize};

use crate::reinforcement_learning::backend::Backend as B;
use burn::backend::wgpu::WgpuDevice::DefaultDevice as device;
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
    serialization_path: String,
    #[serde(skip)]
    network: FeedForward<B>,
}

impl DeepRLAgent {
    pub fn new(
        state_dimensions: usize,
        action_dimensions: usize,
        discount: f64,
        serialization_path: String,
    ) -> DeepRLAgent {
        if let device = Default::default() {
            todo!()
        };
        let mut agent = DeepRLAgent {
            action_dimensions: action_dimensions as usize,
            discount: discount,
            last_action: 0,
            serialization_path: serialization_path,
            network: FeedForwardConfig::new(
                state_dimensions,
                action_dimensions,
                2 * action_dimensions,
            )
            .init::<B>(&device),
        };
        agent.update_q();
        agent
    }

    pub fn load_model(&mut self) {
        let device = Default::default();
        let recorder: PrettyJsonFileRecorder<FullPrecisionSettings> =
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new();
        let model = FeedForward::<B>::init(&device);
        self.network = model
            .load_file(self.serialization_path, &recorder, &device)
            .expect("Should be able to load the model");
    }

    pub fn save_model(&mut self) {
        let recorder = PrettyJsonFileRecorder::<FullPrecisionSettings>::new();
        self.network
            .clone()
            .save_file(self.serialization_path.clone(), &recorder)
            .expect("Should be able to save the model");
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
        todo!("Implement");
        return 0;
    }

    fn update_q(&mut self) {
        todo!("Implement");
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
