use burn::{config::Config, data::dataloader::DataLoaderBuilder, module::Module, nn::loss::CrossEntropyLossConfig, optim::AdamConfig, prelude::Backend, record::CompactRecorder, tensor::{backend::AutodiffBackend, Int, Tensor}, train::{metric::{store::{Aggregate, Direction, Split}, LossMetric}, ClassificationOutput, LearnerBuilder, MetricEarlyStoppingStrategy, StoppingCondition, TrainOutput, TrainStep, ValidStep}};
use crate::{data::{ChessGameBatch, ChessGameBatcher, ChessGameDataSet}, model::{Model, ModelConfig}};
impl <B: Backend> Model<B> {
    pub fn forward_classification(
        &self,
        features: ChessGameBatch<B>,
        label: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(features);
        let loss = CrossEntropyLossConfig::new().init(&output.device()).forward(output.clone(), label.clone().unsqueeze());
        
        ClassificationOutput::new(loss, output, label)    }
}

impl<B: AutodiffBackend> TrainStep<ChessGameBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: ChessGameBatch<B>) -> burn::train::TrainOutput<ClassificationOutput<B>> {
        let label = batch.label.clone();
        let item = self.forward_classification(batch, label);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl <B: Backend> ValidStep<ChessGameBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: ChessGameBatch<B>) -> ClassificationOutput<B> {
        let label = batch.label.clone();
        self.forward_classification(batch, label) 
    }
}

#[derive(Config)]
pub struct TrainingConfig{
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1.0e-4)]
    pub learning_rate: f64,
}

fn create_artifact_dir(artifact_dir: &str) {
    // Remove existing artifacts before to get an accurate learner summary
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn train<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: B::Device) {
    create_artifact_dir(artifact_dir);
    config
        .save(format!("{artifact_dir}/config.json"))
        .expect("Config should be saved successfully");

    B::seed(config.seed);

    let batcher_train = ChessGameBatcher::<B>::new(device.clone());
    let batcher_valid = ChessGameBatcher::<B::InnerBackend>::new(device.clone());
    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(ChessGameDataSet::train());
    
    let dataloader_test= DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.batch_size)
        .num_workers(config.num_workers)
        .build(ChessGameDataSet::test());

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .early_stopping(MetricEarlyStoppingStrategy::new::<LossMetric<B>>(
            Aggregate::Mean,
            Direction::Lowest,
            Split::Valid,
            StoppingCondition::NoImprovementSince { n_epochs: 1 },
        ))
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(
            config.model.init::<B>(&device),
            config.optimizer.init(),
            config.learning_rate,
    );

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    model_trained
        .save_file(format!("{artifact_dir}/model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}