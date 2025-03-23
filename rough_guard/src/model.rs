use burn::{
    nn::{Linear, LinearConfig},
    prelude::*,
    tensor::{activation::relu, Tensor, T},
};
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};

pub trait DeepLearningModel<B: Backend>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2>;
}
//////////////////////////////////////////////// MLP template ///////////////////////////////////////////////////////
#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    pub linear_layers: Vec<Linear<B>>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> Mlp<B> {
    pub fn new(
        layers_info: Vec<(usize, usize)>,
        class_weights: Tensor<B, 1>,
        device: &Device<B>,
    ) -> Self {
        let mut linear_layers: Vec<Linear<B>> = Vec::new();

        for layer_info in layers_info.iter() {
            let layer = LinearConfig::new(layer_info.0, layer_info.1).init(device);
            linear_layers.push(layer);
        }

        Self {
            linear_layers,
            class_weights,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for Mlp<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = games;
        for layer in &self.linear_layers {
            x = layer.forward(x);
        }
        return x;
    }
}

//////////////////////////////////////////////// Kan Template ///////////////////////////////////////////////////////
#[derive(Module, Debug)]
pub struct Kan<B: Backend> {
    pub kan_layers: Vec<EfficientKan<B>>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> Kan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    pub fn new(
        layers_info: Vec<(Vec<i32>, Vec<i32>)>,
        class_weights: Tensor<B, 1>,
        device: &Device<B>,
    ) -> Self {
        // layers info : each vector in it represent one Kan layer

        let mut kan_layers: Vec<EfficientKan<B>> = Vec::new();

        for layer_info in layers_info.iter() {
            let kan_layer = construct_kan_layer(&layer_info.0, &layer_info.1, device);
            kan_layers.push(kan_layer);
        }
        Self {
            kan_layers,
            class_weights,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for Kan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = games;
        for kan_layer in &self.kan_layers {
            x = kan_layer.forward(x)
        }
        return x;
    }
}

fn construct_kan_layer<B: Backend>(
    options_values: &Vec<i32>,
    hyper_parameters: &Vec<i32>,
    device: &Device<B>,
) -> EfficientKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let mut kan_options = KanOptions::new([
        options_values[0] as u32,
        options_values[1] as u32,
        options_values[2] as u32,
    ]);
    //hyper_parameters[0] --> grid size
    if hyper_parameters[0] != 0 {
        kan_options = kan_options.with_grid_size(hyper_parameters[0] as u16);
    }
    //hyper_parameters[1] ---> spline order
    if hyper_parameters[1] != 0 {
        kan_options = kan_options.with_spline_order(hyper_parameters[1] as u32);
    }
    //hyper_parameters[2] ---> scale_base
    if hyper_parameters[2] != 0 {
        kan_options = kan_options.with_scale_base(hyper_parameters[2] as f32);
    }
    //hyper_parameters[3] --> scale noise
    if hyper_parameters[3] != 0 {
        kan_options = kan_options.with_scale_noise(hyper_parameters[3] as f32);
    }

    EfficientKan::new(&kan_options, device)
}
