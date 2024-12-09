use burn::{config::Config, data::dataloader::batcher::Batcher, module::Module, prelude::Backend, record::{CompactRecorder, Recorder}};

use crate::{data::{ChessPositionBatcher, ChessPositionItem}, training::TrainingConfig};


pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: ChessPositionItem) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = config.model.init::<B>(&device).load_record(record);

    let batcher = ChessPositionBatcher::new(device);
    let batch = batcher.batch(vec![item.clone()]);
    let output = model.infer(batch.fens) * 400.0;

    let predicted = output.flatten::<1>(0, 1).into_scalar();

    println!("Predicted {} Expected {}", predicted, item.evaluation);
}