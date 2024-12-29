// use crate::{data::{ChessBoardBatcher, ChessDataset}, model::Model};

// use burn::{
//     data::dataloader::DataLoaderBuilder,
//     optim::{decay::WeightDecayConfig, AdamConfig},
//     prelude::*,
//     record::{CompactRecorder, NoStdTrainingRecorder},
//     tensor::backend::AutodiffBackend,
//     train::{
//         metric::{
//             store::{Aggregate, Direction, Split},
//             AccuracyMetric, CpuMemory, CpuTemperature, CpuUse, LossMetric,
//         },
//         LearnerBuilder, MetricEarlyStoppingStrategy, StoppingCondition,
//     },
// };

// static ARTIFACT_DIR: &str = "/tmp/hook_lens";

// #[derive(Config)]
// pub struct HookLensConfig {
//     #[config(default = 10)]
//     pub num_epochs: usize,

//     #[config(default = 64)]
//     pub batch_size: usize,

//     #[config(default = 4)]
//     pub num_workers: usize,

//     #[config(default = 42)]
//     pub seed: u64,

//     pub optimizer: AdamConfig,
// }

// fn create_artifact_dir(artifact_dir: &str) {
//     // Remove existing artifacts before to get an accurate learner summary
//     std::fs::remove_dir_all(artifact_dir).ok();
//     std::fs::create_dir_all(artifact_dir).ok();
// }

// pub fn run<B: AutodiffBackend>(device: B::Device) {
//     create_artifact_dir(ARTIFACT_DIR);
//     // Config
//     let config_optimizer = AdamConfig::new().with_weight_decay(Some(WeightDecayConfig::new(5e-5)));
//     let config = HookLensConfig::new(config_optimizer);
//     B::seed(config.seed);

//     // Data
//     let batcher_train = ChessBoardBatcher::<B>::new(device.clone());
//     let batcher_valid = ChessBoardBatcher::<B::InnerBackend>::new(device.clone());

//     let dataloader_train = DataLoaderBuilder::new(batcher_train)
//         .batch_size(config.batch_size)
//         .shuffle(config.seed)
//         .num_workers(config.num_workers)
//         .build(ChessDataset::train());
//     let dataloader_test = DataLoaderBuilder::new(batcher_valid)
//         .batch_size(config.batch_size)
//         .shuffle(config.seed)
//         .num_workers(config.num_workers)
//         .build(ChessDataset::test());

//     // Model
//     let learner = LearnerBuilder::new(ARTIFACT_DIR)
//         .metric_train_numeric(AccuracyMetric::new())
//         .metric_valid_numeric(AccuracyMetric::new())
//         .metric_train_numeric(CpuUse::new())
//         .metric_valid_numeric(CpuUse::new())
//         .metric_train_numeric(CpuMemory::new())
//         .metric_valid_numeric(CpuMemory::new())
//         .metric_train_numeric(CpuTemperature::new())
//         .metric_valid_numeric(CpuTemperature::new())
//         .metric_train_numeric(LossMetric::new())
//         .metric_valid_numeric(LossMetric::new())
//         .with_file_checkpointer(CompactRecorder::new())
//         .early_stopping(MetricEarlyStoppingStrategy::new::<LossMetric<B>>(
//             Aggregate::Mean,
//             Direction::Lowest,
//             Split::Valid,
//             StoppingCondition::NoImprovementSince { n_epochs: 1 },
//         ))
//         .devices(vec![device.clone()])
//         .num_epochs(config.num_epochs)
//         .summary()
//         .build(Model::new(&device), config.optimizer.init(), 1e-4);

//     let model_trained = learner.fit(dataloader_train, dataloader_test);

//     config
//         .save(format!("{ARTIFACT_DIR}/config.json").as_str())
//         .unwrap();

//     model_trained
//         .save_file(
//             format!("{ARTIFACT_DIR}/model"),
//             &NoStdTrainingRecorder::new(),
//         )
//         .expect("Failed to save trained model");
// }
// 
use std::time::Instant;

use crate::data::{ChessBoardBatch, ChessBoardBatcher, ChessDataset};

use burn::{
    data::dataloader::DataLoaderBuilder,
    nn::loss::CrossEntropyLossConfig,
    optim::SgdConfig,
    prelude::*,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{
        metric::{AccuracyMetric, LossMetric},
        ClassificationOutput, LearnerBuilder, TrainOutput, TrainStep, ValidStep,
    },
};

use resnet_burn::{weights, ResNet};

const NUM_CLASSES: u8 = 13;
const ARTIFACT_DIR: &str = "/tmp/hook_lens";

pub trait LabelClassification<B: Backend> {
    fn forward_classification(
        &self,
        images: Tensor<B, 4>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B>;
}

impl<B: Backend> LabelClassification<B> for ResNet<B> {
    fn forward_classification(
        &self,
        images: Tensor<B, 4>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(images);
        let loss = CrossEntropyLossConfig::new()
            .init(&output.device())
            .forward(output.clone(), targets.clone());

        ClassificationOutput::new(loss, output, targets)
    }
}

impl<B: AutodiffBackend> TrainStep<ChessBoardBatch<B>, ClassificationOutput<B>> for ResNet<B> {
    fn step(&self, batch: ChessBoardBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch.images, batch.targets);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<ChessBoardBatch<B>, ClassificationOutput<B>> for ResNet<B> {
    fn step(&self, batch: ChessBoardBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch.images, batch.targets)
    }
}

#[derive(Config)]
pub struct TrainingConfig {
    pub optimizer: SgdConfig,
    #[config(default = 400)]
    pub num_epochs: usize,
    #[config(default = 16)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 0.001)]
    pub learning_rate: f64,
}

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn train<B: AutodiffBackend>(config: TrainingConfig, device: B::Device) {
    create_artifact_dir(ARTIFACT_DIR);

    config
        .save(format!("{ARTIFACT_DIR}/config.json"))
        .expect("Config should be saved successfully");

    B::seed(config.seed);

    // Pre-trained ResNet-18 adapted for num_classes in this task
    let model = ResNet::resnet152_pretrained(weights::ResNet152::ImageNet1kV2, &device)
        .unwrap()
        .with_classes(NUM_CLASSES.into());


    let batcher_train = ChessBoardBatcher::<B>::new(device.clone());
    let batcher_valid = ChessBoardBatcher::<B::InnerBackend>::new(device.clone());
    

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(ChessDataset::train());
    
    
    let dataloader_test = DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.batch_size)
        .num_workers(config.num_workers)
        .build(ChessDataset::test());
    

    let learner = LearnerBuilder::new(ARTIFACT_DIR)
        .metric_train_numeric(AccuracyMetric::new())
        .metric_valid_numeric(AccuracyMetric::new())
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(
            model,
            config.optimizer.init(),
            config.learning_rate,
        );
    
    // Training
    let now = Instant::now();
    let model_trained = learner.fit(dataloader_train, dataloader_test);
    let elapsed = now.elapsed().as_secs();
    println!("Training completed in {}m{}s", (elapsed / 60), elapsed % 60);

    model_trained
        .save_file(format!("{ARTIFACT_DIR}/model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}