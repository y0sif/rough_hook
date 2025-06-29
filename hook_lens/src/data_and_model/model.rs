use std::usize;

use burn::{
    nn::{
        conv::{Conv2d, Conv2dConfig},
        pool::{MaxPool2d, MaxPool2dConfig},
        BatchNorm, BatchNormConfig, Dropout, DropoutConfig, Linear, LinearConfig, PaddingConfig2d,
        Relu,
    },
    prelude::*,
};

use burn_efficient_kan::{Kan as EfficientKan, KanOptions};
use lax::Lapack;

pub trait DeepLearningModel<B: Backend>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2>;
}

//################################################  CNN Template ####################################################//
#[derive(Module, Debug)]
pub struct Cnn<B: Backend> {
    activation: Relu,
    dropout: Dropout,
    pool: MaxPool2d,
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    conv3: Conv2d<B>,
    conv4: Conv2d<B>,
    conv5: Conv2d<B>,
    conv6: Conv2d<B>,
    batch1: BatchNorm<B, 2>,
    batch2: BatchNorm<B, 2>,
    batch3: BatchNorm<B, 2>,
    batch4: BatchNorm<B, 2>,
    batch5: BatchNorm<B, 2>,
    batch6: BatchNorm<B, 2>,
    fc1: Linear<B>,
    fc2: Linear<B>,
    fc3: Linear<B>,
}

impl<B: Backend> Cnn<B> {
    pub fn new(num_classes: usize, device: &Device<B>, layer_size: usize) -> Self {
        let conv1 = Conv2dConfig::new([3, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv2 = Conv2dConfig::new([32, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let conv3 = Conv2dConfig::new([32, 64], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv4 = Conv2dConfig::new([64, 64], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let conv5 = Conv2dConfig::new([64, 128], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv6 = Conv2dConfig::new([128, 128], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let pool = MaxPool2dConfig::new([2, 2]).with_strides([2, 2]).init();

        let fc1 = LinearConfig::new(2048, 512).init(device);
        let fc2 = LinearConfig::new(512, 64).init(device);
        let fc3 = LinearConfig::new(64, num_classes).init(device);

        let dropout = DropoutConfig::new(0.25).init();

        let batch1 = BatchNormConfig::new(32).init(device);
        let batch2 = BatchNormConfig::new(32).init(device);
        let batch3 = BatchNormConfig::new(64).init(device);
        let batch4 = BatchNormConfig::new(64).init(device);
        let batch5 = BatchNormConfig::new(128).init(device);
        let batch6 = BatchNormConfig::new(128).init(device);

        Self {
            activation: Relu::new(),
            dropout,
            pool,
            conv1,
            conv2,
            conv3,
            conv4,
            conv5,
            conv6,
            batch1,
            batch2,
            batch3,
            batch4,
            batch5,
            batch6,
            fc1,
            fc2,
            fc3,
        }
    }
}
impl<B: Backend> DeepLearningModel<B> for Cnn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = self.batch1.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv2.forward(x);
        let x = self.batch2.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv3.forward(x);
        let x = self.batch3.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv4.forward(x);
        let x = self.batch4.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv5.forward(x);
        let x = self.batch5.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv6.forward(x);
        let x = self.batch6.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = x.flatten(1, 3);

        let x = self.fc1.forward(x);
        let x = self.activation.forward(x);
        let x = self.dropout.forward(x);

        let x = self.fc2.forward(x);
        let x = self.activation.forward(x);
        let x = self.dropout.forward(x);

        self.fc3.forward(x)
    }
}

//################################################  KAN Template ####################################################//

#[derive(Module, Debug)]
pub struct Kan<B: Backend> {
    kan_layer: EfficientKan<B>,
}
impl<B: Backend> Kan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    pub fn new(
        num_classes: usize,
        device: &Device<B>,
        hidden_layer_size: usize,
        hyper_parameters: Vec<Option<i32>>,
    ) -> Self {
        let kan_layer =
            construct_kan_layer(hidden_layer_size, num_classes, &hyper_parameters, device);

        Self { kan_layer }
    }
}

impl<B: Backend> DeepLearningModel<B> for Kan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        let x = x.flatten(1, 3);

        self.kan_layer.forward(x)
    }
}

//################################################  KAN_CNN Template ####################################################//

#[derive(Module, Debug)]
pub struct KanCnn<B: Backend> {
    activation: Relu,
    dropout: Dropout,
    pool: MaxPool2d,
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    conv3: Conv2d<B>,
    conv4: Conv2d<B>,
    conv5: Conv2d<B>,
    conv6: Conv2d<B>,
    batch1: BatchNorm<B, 2>,
    batch2: BatchNorm<B, 2>,
    batch3: BatchNorm<B, 2>,
    batch4: BatchNorm<B, 2>,
    batch5: BatchNorm<B, 2>,
    batch6: BatchNorm<B, 2>,
    kan_layer: EfficientKan<B>,
}

impl<B: Backend> KanCnn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    pub fn new(
        num_classes: usize,
        device: &Device<B>,
        hidden_layer_size: usize,
        hyper_parameters: Vec<Option<i32>>,
    ) -> Self {
        let conv1: Conv2d<B> = Conv2dConfig::new([3, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv2 = Conv2dConfig::new([32, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let conv3 = Conv2dConfig::new([32, 64], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv4 = Conv2dConfig::new([64, 64], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let conv5 = Conv2dConfig::new([64, 128], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let conv6 = Conv2dConfig::new([128, 128], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);

        let pool = MaxPool2dConfig::new([2, 2]).with_strides([2, 2]).init();

        let dropout = DropoutConfig::new(0.25).init();

        let batch1 = BatchNormConfig::new(32).init(device);
        let batch2 = BatchNormConfig::new(32).init(device);
        let batch3 = BatchNormConfig::new(64).init(device);
        let batch4 = BatchNormConfig::new(64).init(device);
        let batch5 = BatchNormConfig::new(128).init(device);
        let batch6 = BatchNormConfig::new(128).init(device);

        let kan_layer =
            construct_kan_layer(hidden_layer_size, num_classes, &hyper_parameters, device);

        Self {
            activation: Relu::new(),
            dropout,
            pool,
            conv1,
            conv2,
            conv3,
            conv4,
            conv5,
            conv6,
            batch1,
            batch2,
            batch3,
            batch4,
            batch5,
            batch6,
            kan_layer,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for KanCnn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = self.batch1.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv2.forward(x);
        let x = self.batch2.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv3.forward(x);
        let x = self.batch3.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv4.forward(x);
        let x = self.batch4.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv5.forward(x);
        let x = self.batch5.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv6.forward(x);
        let x = self.batch6.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = x.flatten(1, 3);

        self.kan_layer.forward(x)
    }
}

fn construct_kan_layer<B: Backend>(
    hidden_layer_size: usize,
    num_classes: usize,
    hyper_parameters: &Vec<Option<i32>>,
    device: &Device<B>,
) -> EfficientKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + lax::Lapack,
{
    let mut kan_options = KanOptions::new([2048, hidden_layer_size as u32, num_classes as u32]);

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
