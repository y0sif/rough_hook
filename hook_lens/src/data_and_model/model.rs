use burn::{
    nn::{
        conv::{Conv2d, Conv2dConfig},
        pool::{MaxPool2d, MaxPool2dConfig},
        Dropout, DropoutConfig, Linear, LinearConfig, PaddingConfig2d, Relu,
        BatchNorm, BatchNormConfig,
    },
    prelude::*,
};

use burn_efficient_kan::*;

#[derive(Module, Debug)]
pub struct Cnn<B: Backend> {
    activation: Relu,
    dropout: Dropout,
    pool: MaxPool2d,
    pub conv1: Conv2d<B>,
    pub conv2: Conv2d<B>,
    pub conv3: Conv2d<B>,
    pub conv4: Conv2d<B>,
    pub conv5: Conv2d<B>,
    pub conv6: Conv2d<B>,
    pub batch1: BatchNorm<B, 2>,
    pub batch2: BatchNorm<B, 2>,
    pub batch3: BatchNorm<B, 2>,
    pub batch4: BatchNorm<B, 2>,
    pub batch5: BatchNorm<B, 2>,
    pub batch6: BatchNorm<B, 2>,
    fc1: Linear<B>,
    fc2: Linear<B>,
    fc3: Linear<B>,
}

#[derive(Module, Debug)]
pub struct CustomCnn<B: Backend> {
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

impl<B: Backend> Cnn<B> {
    pub fn new(num_classes: usize, device: &Device<B>) -> Self {
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
            fc3
        }
    }

    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
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

impl<B: Backend> CustomCnn<B> {
    pub fn new(
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
        num_classes: usize,
        device: &Device<B>) -> Self {
        let pool = MaxPool2dConfig::new([2, 2]).with_strides([2, 2]).init();

        let kan_layer = EfficientKan::new(&KanOptions::new([
            2048,
            256,
            num_classes as u32
            ]), device);

        let dropout = DropoutConfig::new(0.25).init();

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
            kan_layer
        }
    }

    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
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