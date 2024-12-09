use burn::{
    nn::{Linear, LinearConfig},
    prelude::*,
};
use nn::{Sigmoid, Tanh};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>, 
    activation: Sigmoid
}

#[derive(Config, Debug)]
pub struct ModelConfig;

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model{
            linear1: LinearConfig::new(768, 8).init(device),
            linear2: LinearConfig::new(8, 8).init(device),
            linear3: LinearConfig::new(8, 1).init(device),
            activation: Sigmoid::new(),
        }
    }
}

impl <B:Backend> Model<B> {
pub fn forward(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(positions);
        let x = clipped_relu(x); 
        let x = self.linear2.forward(x);
        let x = clipped_relu(x); 
        let x = self.linear3.forward(x);
        self.activation.forward(x)
        
    }
}

fn clipped_relu<B: Backend>(tensor: Tensor<B, 2>) -> Tensor<B, 2> {
    tensor.clamp_max(1).clamp_min(0)
}