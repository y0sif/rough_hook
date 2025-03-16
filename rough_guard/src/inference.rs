use crate::{data::
    {ChessGameBatcher, ChessGameItem}, model::Model, training::TrainingConfig};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{activation::softmax, cast::ToElement}
};

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: ChessGameItem) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model; run train first");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist; run train first");

    let dummy_weights = Tensor::<B, 1>::zeros([4], &device);
    let model: Model<B> = config.model.init(&device, dummy_weights).load_record(record);

    let label = item.label;
    let batcher = ChessGameBatcher::new(device);
    let batch = batcher.batch(vec![item]);

    // Forward pass
    let output = model.forward(batch.features);

    // Apply softmax to get probabilities
    let probabilities = softmax(output, 1);
    let predicted = probabilities.clone().argmax(1).flatten::<1>(0, 1).into_scalar();

    println!("Predicted: {} ({}), Actual: {}", 
        predicted,
        match predicted.to_i32() {
            0 => "Clean Game",
            1 => "White Cheating",
            2 => "Black Cheating",
            3 => "Both Cheating",
            _ => unreachable!()
        },
        label
    );
    
    // Print confidence distribution
    let probs: Vec<f32> = probabilities
        .squeeze::<1>(0)
        .into_data()
        .convert::<f32>()
        .to_vec()
        .unwrap();
    
    println!("Confidence:");
    println!("- No Cheat:    {:.2}%", probs[0] * 100.0);
    println!("- White Cheat:   {:.2}%", probs[1] * 100.0);
    println!("- Black Cheat:   {:.2}%", probs[2] * 100.0);
    println!("- Both Cheat: {:.2}%", probs[3] * 100.0);
}
