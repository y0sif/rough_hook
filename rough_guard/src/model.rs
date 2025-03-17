use burn::{
    nn::{Linear, LinearConfig},
    prelude::*, tensor::activation::relu,
    tensor::Tensor,
};
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    pub kan_layer: EfficientKan<B>,
    pub class_weights: Tensor<B, 1>
}

#[derive(Config, Debug)]
pub struct ModelConfig;

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device, class_weights: Tensor<B, 1>) -> Model<B> 
    where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
    {
        Model{
            kan_layer: EfficientKan::new(&KanOptions::new([
                241,
                128,
                4
            ]).with_grid_size(2).with_spline_order(2), device),
            class_weights
        }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.kan_layer.forward(games);
        x
    }

    pub fn infer(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        self.forward(games.detach())
    }
}
