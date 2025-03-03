use crate::{data::
    {ChessGameBatcher, ChessGameItem}, model::Model, training::TrainingConfig};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::activation::softmax
};

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: ChessGameItem) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model; run train first");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist; run train first");

    let model: Model<B> = config.model.init(&device).load_record(record);

    let label = item.label;
    let batcher = ChessGameBatcher::new(device);
    let batch = batcher.batch(vec![item]);

    // Forward pass
    let output = model.forward(batch);

    // Apply softmax to get probabilities
    let probabilities = softmax(output, 1);
    let predicted = probabilities.argmax(1).flatten::<1>(0, 1).into_scalar();

    println!("Predicted {} Expected {}", predicted, label);
}
