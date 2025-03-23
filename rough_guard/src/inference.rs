use super::model::DeepLearningModel;
use super::model::{Kan, KanRecord};
use crate::data::{ChessGameBatcher, ChessGameItem};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{activation::softmax, cast::ToElement},
};

#[derive(Module, Debug)]
pub enum ModelEnum<B: Backend> {
    Kan(Kan<B>),
}

impl<B: Backend> DeepLearningModel<B> for ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        match self {
            ModelEnum::Kan(model) => model.forward(games),
        }
    }
}

pub fn infer<B: Backend>(model: &ModelEnum<B>, device: B::Device, game: ChessGameItem)
where
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

    println!(
        "Predicted: {} ({}), Actual: {}",
        predicted,
        match predicted.to_i32() {
            0 => "Clean Game",
            1 => "White Cheating",
            2 => "Black Cheating",
            3 => "Both Cheating",
            _ => unreachable!(),
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
    println!("- None:    {:.2}%", probs[0] * 100.0);
    println!("- White:   {:.2}%", probs[1] * 100.0);
    println!("- Black:   {:.2}%", probs[2] * 100.0);
    println!("- Both: {:.2}%", probs[3] * 100.0);
}

pub fn load_model_paramter<B: Backend>(
    id: i8,
    artifact_dir: &str,
    device: B::Device,
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let dummy_weights = Tensor::<B, 1>::zeros([4], &device);
    let model_object = get_model_object(id, dummy_weights, &device);
    let inner_model = match model_object {
        ModelEnum::Kan(kan_model) => {
            let model = get_kan_model_from_record(kan_model, artifact_dir, device);
            ModelEnum::Kan(model)
        }
    };

    return inner_model;
}

fn get_model_object<B: Backend>(
    id: i8,
    class_weights: Tensor<B, 1>,
    device: &B::Device,
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    match id {
        1 => ModelEnum::Kan(Kan::new(
            vec![
                (vec![241, 256, 128], vec![6, 6, 0, 0]),
                (vec![128, 64, 4], vec![6, 6, 0, 0]),
            ],
            class_weights,
            device,
        )),
        _ => panic!("not valid model Id"),
    }
}

fn get_kan_model_from_record<B: Backend>(
    kan_model: Kan<B>,
    artifact_dir: &str,
    device: B::Device,
) -> Kan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    kan_model.load_record(record)
}

/* loadin model
let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model; run train first");

    let dummy_weights = Tensor::<B, 1>::zeros([4], &device);
    let model: Model<B> = config
        .model
        .init(&device, dummy_weights)
        .load_record(record);

 */
