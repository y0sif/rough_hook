use burn::{
    nn::{Linear, LinearConfig},
    prelude::*, tensor::activation::relu,
    tensor::Tensor,
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    linear4: Linear<B>,
    linear5: Linear<B>,
    linear6: Linear<B>,
    linear7: Linear<B>,
    pub class_weights: Tensor<B, 1>
}

#[derive(Config, Debug)]
pub struct ModelConfig;

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device, class_weights: Tensor<B, 1>) -> Model<B> {
        Model{
            linear1: LinearConfig::new(241, 64).init(device),
            linear2: LinearConfig::new(64, 128).init(device),
            linear3: LinearConfig::new(128, 256).init(device),
            linear4: LinearConfig::new(256, 256).init(device),
            linear5: LinearConfig::new(256, 128).init(device),
            linear6: LinearConfig::new(128, 64).init(device),
            linear7: LinearConfig::new(64, 4).init(device),
            class_weights
        }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(games);
        let x = relu(x);
        let x = self.linear2.forward(x);
        let x = relu(x);
        let x = self.linear3.forward(x);
        let x = relu(x);
        let x = self.linear4.forward(x);
        let x = relu(x);
        let x = self.linear5.forward(x);
        let x = relu(x);
        let x = self.linear6.forward(x);
        let x = relu(x);
        self.linear7.forward(x)
    }

    pub fn infer(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        self.forward(games.detach())
    }
}
