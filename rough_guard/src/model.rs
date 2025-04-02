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
pub struct ModifiedKan<B: Backend> {
    pub kan_layers: Vec<EfficientKan<B>>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> ModifiedKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    pub fn new(
        layers_info: Vec<(Vec<i32>, Vec<Option<i32>>)>,
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

impl<B: Backend> DeepLearningModel<B> for ModifiedKan<B>
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
    hyper_parameters: &Vec<Option<i32>>,
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

    if let Some(grid_size) = hyper_parameters[0] {
        kan_options = kan_options.with_grid_size(grid_size as u16);
    }

    if let Some(spline_order) = hyper_parameters[1] {
        kan_options = kan_options.with_spline_order(spline_order as u32);
    }

    if let Some(scale_base) = hyper_parameters[2] {
        kan_options = kan_options.with_scale_base(scale_base as f32);
    }

    if let Some(scale_noise) = hyper_parameters[3] {
        kan_options = kan_options.with_scale_noise(scale_noise as f32);
    }

    EfficientKan::new(&kan_options, device)
}
