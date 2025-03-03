use burn::{
    nn::{Linear, LinearConfig},
    prelude::*, tensor::activation::relu
};

use crate::data::ChessGameBatch;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>
}

#[derive(Config, Debug)]
pub struct ModelConfig;

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model{
            linear1: LinearConfig::new(241, 128).init(device),
            linear2: LinearConfig::new(128, 64).init(device),
            linear3: LinearConfig::new(64, 4).init(device)
        }
    }
}

impl<B: Backend> Model<B> {
    fn forward_pass(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(games);
        let x = relu(x);
        let x = self.linear2.forward(x);
        let x = relu(x);
        self.linear3.forward(x)
    }

    pub fn forward(&self, batch: ChessGameBatch<B>) -> Tensor<B, 2> {
        self.forward_pass(batch.flatten())
    }

    pub fn infer(&self, batch: ChessGameBatch<B>) -> Tensor<B, 2> {
        self.forward_pass(batch.flatten().detach())
    }
}
