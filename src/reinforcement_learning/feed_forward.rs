use burn::{
    nn::{Linear, LinearConfig, Relu},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Module, Debug)]
pub struct FeedForward<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

#[derive(Config, Debug)]
pub struct FeedForwardConfig {
    inputs: usize,
    outputs: usize,
    hidden_size: usize,
    #[config(default = "0.5")]
    dropout: f64,
}

impl FeedForwardConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self, device: &B::Device) -> FeedForward<B> {
        FeedForward {
            activation: Relu::new(),
            linear1: LinearConfig::new(self.inputs, self.hidden_size).init(device),
            linear2: LinearConfig::new(self.hidden_size, self.outputs).init(device),
        }
    }
}
