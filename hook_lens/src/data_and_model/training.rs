use std::time::Instant;

use crate::{
    data_and_model::data::{ChessBoardBatch, ChessBoardBatcher, ChessDataset},
    data_and_model::model::Kan,
};
use burn::{
    data::dataloader::DataLoaderBuilder,
    nn::loss::CrossEntropyLossConfig,
    optim::AdamConfig,
    prelude::*,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{
        metric::{AccuracyMetric, LossMetric},
        ClassificationOutput, LearnerBuilder, TrainOutput, TrainStep, ValidStep,
    },
};

const NUM_CLASSES: u8 = 13;
const ARTIFACT_DIR: &str = "/tmp/hook_lens";

impl<B: Backend> Kan<B> {
    pub fn forward_classification(
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

impl<B: AutodiffBackend> TrainStep<ChessBoardBatch<B>, ClassificationOutput<B>> for Kan<B> {
    fn step(&self, batch: ChessBoardBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch.images, batch.targets);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<ChessBoardBatch<B>, ClassificationOutput<B>> for Kan<B> {
    fn step(&self, batch: ChessBoardBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch.images, batch.targets)
    }
}

#[derive(Config)]
pub struct TrainingConfig {
    pub optimizer: AdamConfig,
    #[config(default = 400)]
    pub num_epochs: usize,
    #[config(default = 8)]
    pub batch_size: usize,
    #[config(default = 8)]
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

pub fn train<B: AutodiffBackend>(config: TrainingConfig, device: B::Device) 
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    create_artifact_dir(ARTIFACT_DIR);

    config
        .save(format!("{ARTIFACT_DIR}/config.json"))
        .expect("Config should be saved successfully");

    B::seed(config.seed);


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
            Kan::new(NUM_CLASSES.into(), &device),
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