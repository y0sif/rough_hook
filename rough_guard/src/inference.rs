use crate::data::{ChessGameBatcher, ChessGameItem};
use crate::model::{self, Model, ModelConfig};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{activation::softmax, cast::ToElement},
};
use serde::de;


pub fn infer<B: Backend>(
    model: &Model<B>,
    device: B::Device,
    game: &ChessGameItem,
) -> (u8, Vec<f32>)
where
    B::IntElem: TryInto<u8> + std::fmt::Debug,
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let label = game.label;
    let batcher = ChessGameBatcher::new(device);
    let batch = batcher.batch(vec![game]);

    // Forward pass
    let output = model.forward(batch.features);

    // Apply softmax to get probabilities
    let probabilities = softmax(output, 1);
    let predicted = probabilities
        .clone()
        .argmax(1)
        .flatten::<1>(0, 1)
        .into_scalar();

    let result: Result<u8, _> = predicted.try_into();

    let predicted_res = match result {
        Ok(value) => value,
        Err(_) => panic!("Failed to convert prediction to u8"),
    };

    // Print confidence distribution
    let probs: Vec<f32> = probabilities
        .squeeze::<1>(0)
        .into_data()
        .convert::<f32>()
        .to_vec()
        .unwrap();

    (predicted_res, probs)
}

pub fn load_model_paramter<B: Backend>(
    id: i8,
    artifact_dir: &str,
    device: B::Device,
) -> Model<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let dummy_weights = Tensor::<B, 1>::zeros([4], &device);
    let model = ModelConfig::new();

    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    let trained_model = model.init(&device, dummy_weights).load_record(record); 
    trained_model
}