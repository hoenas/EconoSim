use tch::nn::Linear;
use tch::nn::Module;
use tch::Tensor;

#[derive(Debug)]
struct DeepQNetwork {
    layer1: Linear,
    layer2: Linear,
    layer3: Linear,
}

impl DeepQNetwork {}

impl Module for DeepQNetwork {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.apply(&self.layer1)
            .relu()
            .apply(&self.layer2)
            .relu()
            .apply(&self.layer3)
    }
}
