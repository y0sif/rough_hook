use burn::{config::Config, data::dataloader::DataLoaderBuilder, module::Module, nn::loss::CrossEntropyLossConfig, optim::AdamConfig, prelude::Backend, record::CompactRecorder, tensor::{backend::AutodiffBackend, Int, Tensor}, train::{metric::{store::{Aggregate, Direction, Split}, LossMetric, AccuracyMetric}, ClassificationOutput, LearnerBuilder, MetricEarlyStoppingStrategy, StoppingCondition, TrainOutput, TrainStep, ValidStep}};
use crate::{data::{ChessGameBatcher, ChessGameDataSet, FeaturesBatch}, model::{Model, ModelConfig}};
use burn_dataset::Dataset;
 
 fn compute_class_weights<B: Backend>(dataset: &crate::data::ChessGameDataSet, device: &B::Device) -> Tensor<B, 1> {
     let num_classes = 4;
     let mut counts = vec![0f32; num_classes];
     let total = dataset.len() as f32;
     
     for i in 0..dataset.len() {
         if let Some(item) = dataset.get(i) {
             let label = item.label as usize;
             counts[label] += 1.0;
         }
     }
     
     let weights: Vec<f32> = counts
         .iter()
         .map(|&count| total / ((num_classes as f32) * count))
         .collect();
     
     Tensor::<B, 1>::from_data(weights.as_slice(), device)
 }

impl <B: Backend> Model<B> {
    pub fn forward_classification(
        &self,
        features: Tensor<B,2>,
        label: Tensor<B, 1, Int>,
        class_weights: Tensor<B, 1>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(features);
        let loss = CrossEntropyLossConfig::new()
                                 .with_weights(Some(class_weights.clone().into_data().convert::<f32>().to_vec().unwrap()))
                                 .init(&output.device())
                                 .forward(output.clone(), label.clone().unsqueeze());
        
        ClassificationOutput::new(loss, output, label)    }
}

impl<B: AutodiffBackend> TrainStep<FeaturesBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: FeaturesBatch<B>) -> burn::train::TrainOutput<ClassificationOutput<B>> {
        let label = batch.label.clone();
        let item = self.forward_classification(batch.features, label, self.class_weights.clone());
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl <B: Backend> ValidStep<FeaturesBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: FeaturesBatch<B>) -> ClassificationOutput<B> {
        let label = batch.label.clone();
        self.forward_classification(batch.features, label, self.class_weights.clone()) 
    }
}

#[derive(Config)]
pub struct TrainingConfig{
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 100)]
    pub num_epochs: usize,
    #[config(default = 16)]
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

    // Compute class weights from the training dataset.
    let train_dataset = ChessGameDataSet::train();
    let class_weights = compute_class_weights::<B>(&train_dataset, &device);

    //let vec: Vec<f32> = class_weights.clone().into_data().convert::<f32>().to_vec().unwrap();
    //println!("{:?}", vec);

    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(AccuracyMetric::new())
        .metric_valid_numeric(AccuracyMetric::new())
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .early_stopping(MetricEarlyStoppingStrategy::new::<LossMetric<B>>(
            Aggregate::Mean,
            Direction::Lowest,
            Split::Valid,
            StoppingCondition::NoImprovementSince { n_epochs: 3 },
        ))
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(
            config.model.init::<B>(&device, class_weights),
            config.optimizer.init(),
            config.learning_rate,
    );

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    model_trained
        .save_file(format!("{artifact_dir}/model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}
