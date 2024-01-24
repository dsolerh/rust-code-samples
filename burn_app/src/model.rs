use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig, ReLU},
    tensor::{backend::Backend, Tensor},
};
use serde;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    hidden: Linear<B>,
    activation: ReLU,
    output: Linear<B>,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
}

impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self) -> Model<B> {
        Model {
            hidden: LinearConfig::new(self.input_size, self.hidden_size).init(),
            activation: ReLU::new(),
            output: LinearConfig::new(self.hidden_size, self.output_size).init(),
        }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        // Create a channel at the second dimension.
        let x = self.hidden.forward(input);
        let x = self.activation.forward(x);
        self.output.forward(x)
    }
}
