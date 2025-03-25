use burn::{config::Config, data::dataloader::DataLoaderBuilder, module::Module, nn::loss::{MseLoss, Reduction}, optim::AdamConfig, prelude::Backend, record::CompactRecorder, tensor::{backend::AutodiffBackend, Tensor}, train::{metric::{store::{Aggregate, Direction, Split}, LossMetric}, LearnerBuilder, MetricEarlyStoppingStrategy, RegressionOutput, StoppingCondition, TrainOutput, TrainStep, ValidStep}};
use crate::{data::{ChessPositionBatch, ChessPositionBatcher, ChessPositionDataSet}, model::{Model, ModelConfig}};
impl <B: Backend> Model<B> {
    pub fn forward_regression(
        &self,
        fens: Tensor<B, 2>,
        evaluations: Tensor<B, 1>,
    ) -> RegressionOutput<B> {
        let output = self.forward(fens);
        let loss = MseLoss::new().forward(output.clone(), evaluations.clone().unsqueeze(), Reduction::Mean);
        
        RegressionOutput::new(loss, output.unsqueeze(), evaluations.unsqueeze())
    }
}

impl<B: AutodiffBackend> TrainStep<ChessPositionBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: ChessPositionBatch<B>) -> burn::train::TrainOutput<RegressionOutput<B>> {
        let item = self.forward_regression(batch.fens, batch.evaluations);
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl <B: Backend> ValidStep<ChessPositionBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: ChessPositionBatch<B>) -> RegressionOutput<B> {
        self.forward_regression(batch.fens, batch.evaluations) 
    }
}

#[derive(Config)]
pub struct TrainingConfig{
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 50)]
    pub num_epochs: usize,
    #[config(default = 256)]
    pub batch_size: usize,
    #[config(default = 12)]
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

    let batcher_train = ChessPositionBatcher::<B>::new(device.clone());
    let batcher_valid = ChessPositionBatcher::<B::InnerBackend>::new(device.clone());
    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(ChessPositionDataSet::train());
    
    let dataloader_test= DataLoaderBuilder::new(batcher_valid)
        .batch_size(config.batch_size)
        .num_workers(config.num_workers)
        .build(ChessPositionDataSet::test());

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
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