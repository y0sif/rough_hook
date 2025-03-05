use burn::prelude::*;
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};
use nn::Sigmoid;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    kan: EfficientKan<B>,
    activation: Sigmoid
}

#[derive(Config, Debug)]
pub struct ModelConfig;

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> 
    where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
    {
        Model{
            kan: EfficientKan::new(&KanOptions::new([768, 256, 1]), device),
            activation: Sigmoid::new(),
        }
    }
}

impl <B:Backend> Model<B> {
    pub fn forward(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.kan.forward(positions);
        self.activation.forward(x)
    }
    
    pub fn infer(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {
        self.kan.forward(positions)
    }

}
