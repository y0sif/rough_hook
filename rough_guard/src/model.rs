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
    linear8: Linear<B>,
    linear9: Linear<B>,
    linear10: Linear<B>,
    linear11: Linear<B>,
    linear12: Linear<B>,
    linear13: Linear<B>,
    linear14: Linear<B>,
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
            linear4: LinearConfig::new(256, 512).init(device),
            linear5: LinearConfig::new(512, 1024).init(device),
            linear6: LinearConfig::new(1024, 2048).init(device),
            linear7: LinearConfig::new(2048, 4096).init(device),
            linear8: LinearConfig::new(4096, 2048).init(device),
            linear9: LinearConfig::new(2048, 1024).init(device),
            linear10: LinearConfig::new(1024, 512).init(device),
            linear11: LinearConfig::new(512, 256).init(device),
            linear12: LinearConfig::new(256, 128).init(device),
            linear13: LinearConfig::new(128, 64).init(device),
            linear14: LinearConfig::new(64, 4).init(device),
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
        let x = self.linear7.forward(x);
        let x = relu(x);
        let x = self.linear8.forward(x);
        let x = relu(x);
        let x = self.linear9.forward(x);
        let x = relu(x);
        let x = self.linear10.forward(x);
        let x = relu(x);
        let x = self.linear11.forward(x);
        let x = relu(x);
        let x = self.linear12.forward(x);
        let x = relu(x);
        let x = self.linear13.forward(x);
        let x = relu(x);
        self.linear14.forward(x)
    }

    pub fn infer(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        self.forward(games.detach())
    }
}