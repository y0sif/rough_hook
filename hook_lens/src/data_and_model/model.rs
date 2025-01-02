// use crate::data::ChessBoardBatch;
// use burn::{
//     nn::{loss::CrossEntropyLossConfig, BatchNorm, PaddingConfig2d},
//     prelude::*,
//     tensor::backend::AutodiffBackend,
//     train::{ClassificationOutput, TrainOutput, TrainStep, ValidStep},
// };

// #[derive(Module, Debug)]
// pub struct Model<B: Backend> {
//     conv1: ConvBlock<B>,
//     conv2: ConvBlock<B>,
//     conv3: ConvBlock<B>,
//     dropout: nn::Dropout,
//     fc1: nn::Linear<B>,
//     fc2: nn::Linear<B>,
//     activation: nn::Gelu,
// }

// impl<B: Backend> Default for Model<B> {
//     fn default() -> Self {
//         let device = B::Device::default();
//         Self::new(&device)
//     }
// }

// const NUM_CLASSES: usize = 13;

// impl<B: Backend> Model<B> {
//     pub fn new(device: &B::Device) -> Self {
//         let conv1 = ConvBlock::new([1, 8], [3, 3], device); // out: [Batch,8,26,26]
//         let conv2 = ConvBlock::new([8, 16], [3, 3], device); // out: [Batch,16,24x24]
//         let conv3 = ConvBlock::new([16, 24], [3, 3], device); // out: [Batch,24,22x22]
//         let hidden_size = 24 * 22 * 22;
//         let fc1 = nn::LinearConfig::new(hidden_size, 32)
//             .with_bias(false)
//             .init(device);
//         let fc2 = nn::LinearConfig::new(32, NUM_CLASSES)
//             .with_bias(false)
//             .init(device);

//         let dropout = nn::DropoutConfig::new(0.5).init();

//         Self {
//             conv1,
//             conv2,
//             conv3,
//             dropout,
//             fc1,
//             fc2,
//             activation: nn::Gelu::new(),
//         }
//     }

//     pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
//         let x = self.conv1.forward(x);
//         let x = self.conv2.forward(x);
//         let x = self.conv3.forward(x);

//         let [batch_size, channels, height, width] = x.dims();
//         let x = x.reshape([batch_size, channels * height * width]);

//         let x = self.dropout.forward(x);
//         let x = self.fc1.forward(x);
//         let x = self.activation.forward(x);

//         self.fc2.forward(x)
//     }

//     pub fn forward_classification(&self, item: ChessBoardBatch<B>) -> ClassificationOutput<B> {
//         let targets = item.targets;
//         let output = self.forward(item.images);
//         let loss = CrossEntropyLossConfig::new()
//             .init(&output.device())
//             .forward(output.clone(), targets.clone());

//         ClassificationOutput {
//             loss,
//             output,
//             targets,
//         }
//     }
// }

// #[derive(Module, Debug)]
// pub struct ConvBlock<B: Backend> {
//     conv: nn::conv::Conv2d<B>,
//     norm: BatchNorm<B, 2>,
//     activation: nn::Gelu,
// }

// impl<B: Backend> ConvBlock<B> {
//     pub fn new(channels: [usize; 2], kernel_size: [usize; 2], device: &B::Device) -> Self {
//         let conv = nn::conv::Conv2dConfig::new(channels, kernel_size)
//             .with_padding(PaddingConfig2d::Valid)
//             .init(device);
//         let norm = nn::BatchNormConfig::new(channels[1]).init(device);

//         Self {
//             conv,
//             norm,
//             activation: nn::Gelu::new(),
//         }
//     }

//     pub fn forward(&self, input: Tensor<B, 4>) -> Tensor<B, 4> {
//         let x = self.conv.forward(input);
//         let x = self.norm.forward(x);

//         self.activation.forward(x)
//     }
// }

// impl<B: AutodiffBackend> TrainStep<ChessBoardBatch<B>, ClassificationOutput<B>> for Model<B> {
//     fn step(&self, item: ChessBoardBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
//         let item = self.forward_classification(item);

//         TrainOutput::new(self, item.loss.backward(), item)
//     }
// }

// impl<B: Backend> ValidStep<ChessBoardBatch<B>, ClassificationOutput<B>> for Model<B> {
//     fn step(&self, item: ChessBoardBatch<B>) -> ClassificationOutput<B> {
//         self.forward_classification(item)
//     }
// }
//
use burn::{
    nn::{
        conv::{Conv2d, Conv2dConfig},
        pool::{MaxPool2d, MaxPool2dConfig},
        Dropout, DropoutConfig, Linear, LinearConfig, PaddingConfig2d, Relu,
    },
    prelude::*,
};

/// Basic convolutional neural network with VGG-style blocks.
//
//       VGG block
// ┌────────────────────┐
// │      3x3 conv      │
// │          ↓         │
// │     activation     │
// │          ↓         │
// │      3x3 conv      │
// │          ↓         │
// │     activation     │
// │          ↓         │
// │       maxpool      │
// └────────────────────┘
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
    fc1: Linear<B>,
    fc2: Linear<B>,
    fc3: Linear<B>,
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

        let fc1 = LinearConfig::new(1152, 512).init(device);
        let fc2 = LinearConfig::new(512, 64).init(device);
        let fc3 = LinearConfig::new(64, num_classes).init(device);

        let dropout = DropoutConfig::new(0.3).init();

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
            fc1,
            fc2,
            fc3
        }
    }

    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        let x = self.conv1.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv2.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv3.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv4.forward(x);
        let x = self.activation.forward(x);
        let x = self.pool.forward(x);
        let x = self.dropout.forward(x);

        let x = self.conv5.forward(x);
        let x = self.activation.forward(x);
        let x = self.conv6.forward(x);
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