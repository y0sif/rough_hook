use super::model::DeepLearningModel;
use super::model::{Mlp, MlpRecord, ModifiedKan, ModifiedKanRecord};
use crate::data::{ChessGameBatcher, ChessGameItem};
use crate::model::Mlp_no_bn;
use burn::tensor::backend::AutodiffBackend;
use burn::train::{ClassificationOutput, TrainStep, ValidStep};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{activation::softmax, cast::ToElement},
};

// Import FeaturesBatch and ClassificationOutput from their module
use crate::data::FeaturesBatch;

#[derive(Module, Debug)]
pub enum ModelEnum<B: Backend> {
    Mlp(Mlp<B>),
    Mlp_no_bn(Mlp_no_bn<B>),
    ModifiedKan(ModifiedKan<B>),
}

impl<B: Backend> DeepLearningModel<B> for ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        match self {
            ModelEnum::Mlp(model) => model.forward(games),
            ModelEnum::ModifiedKan(model) => model.forward(games),
            ModelEnum::Mlp_no_bn(model) => model.forward(games),
        }
    }
}

pub fn infer<B: Backend>(
    model: &ModelEnum<B>,
    device: B::Device,
    game: &ChessGameItem,
) -> (u8, Vec<f32>)
where
    B::IntElem: TryInto<u8> + std::fmt::Debug,
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    let label = game.label;
    let batcher = ChessGameBatcher::new(device);
    let batch = batcher.batch(vec![game.clone()]);

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
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    let dummy_weights = Tensor::<B, 1>::zeros([4], &device);
    let model_archi = get_model_archi(id, dummy_weights, &device);

    let trained_model = match model_archi {
        ModelEnum::Mlp(mlp_model_archi) => {
            let model = load_mlp_model_weigths(mlp_model_archi, artifact_dir, device);
            ModelEnum::Mlp(model)
        }
        ModelEnum::ModifiedKan(kan_model_archi) => {
            let model = load_kan_model_weigths(kan_model_archi, artifact_dir, device);
            ModelEnum::ModifiedKan(model)
        }
        ModelEnum::Mlp_no_bn(mlp_no_bn_model_archi) => {
            let model = load_mlp_no_bn_model_weigths(mlp_no_bn_model_archi, artifact_dir, device);
            ModelEnum::Mlp_no_bn(model)
        }
    };
    trained_model
}

fn get_model_archi<B: Backend>(
    id: i8,
    class_weights: Tensor<B, 1>,
    device: &B::Device,
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    match id {
        // mlp model with 12 linerar layer
        1 => ModelEnum::Mlp_no_bn(Mlp_no_bn::new(
            vec![
                (7, 16),
                (16, 16),
                (16, 16),
                (16, 8),
                (8, 8),
                (8, 8),
                (8, 4),
                (4, 4),
                (4, 4),
                (4, 2),
            ],
            class_weights,
            0.0,
            device,
        )),
        2 => ModelEnum::Mlp_no_bn(Mlp_no_bn::new(
            vec![
                (7, 16),
                (16, 16),
                (16, 16),
                (16, 8),
                (8, 8),
                (8, 8),
                (8, 4),
                (4, 4),
                (4, 4),
                (4, 2),
            ],
            class_weights,
            0.3,
            device,
        )),
        3 => ModelEnum::Mlp_no_bn(Mlp_no_bn::new(
            vec![
                (7, 7),
                (7, 7),
                (7, 7),
                (7, 6),
                (6, 6),
                (6, 6),
                (6, 5),
                (5, 5),
                (5, 5),
                (5, 4),
                (4, 4),
                (4, 4),
                (4, 3),
                (3, 3),
                (3, 3),
                (3, 2),
                (2, 2),
                (2, 2),
            ],
            class_weights,
            0.0,
            device,
        )),
        4 => ModelEnum::Mlp_no_bn(Mlp_no_bn::new(
            vec![
                (7, 7),
                (7, 7),
                (7, 7),
                (7, 6),
                (6, 6),
                (6, 6),
                (6, 5),
                (5, 5),
                (5, 5),
                (5, 4),
                (4, 4),
                (4, 4),
                (4, 3),
                (3, 3),
                (3, 3),
                (3, 2),
                (2, 2),
                (2, 2),
            ],
            class_weights,
            0.3,
            device,
        )),
        5 => ModelEnum::Mlp(Mlp::new(
            vec![
                (7, 7),
                (7, 7),
                (7, 7),
                (7, 6),
                (6, 6),
                (6, 6),
                (6, 5),
                (5, 5),
                (5, 5),
                (5, 4),
                (4, 4),
                (4, 4),
                (4, 3),
                (3, 3),
                (3, 3),
                (3, 2),
                (2, 2),
                (2, 2),
            ],
            class_weights,
            0.15,
            device,
        )),
        6 => ModelEnum::ModifiedKan(ModifiedKan::new(
            vec![
                ([7, 6, 5], [Some(20), Some(3), None, None]),
                ([5, 4, 3], [Some(10), Some(2), None, None]),
                ([3, 2, 2], [Some(5), Some(1), None, None]),
            ],
            class_weights,
            device,
        )),
        _ => panic!("not valid model Id"),
    }
}

fn load_kan_model_weigths<B: Backend>(
    kan_model: ModifiedKan<B>,
    artifact_dir: &str,
    device: B::Device,
) -> ModifiedKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    kan_model.load_record(record)
}

fn load_mlp_model_weigths<B: Backend>(
    mlp_model: Mlp<B>,
    artifact_dir: &str,
    device: B::Device,
) -> Mlp<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    mlp_model.load_record(record)
}
fn load_mlp_no_bn_model_weigths<B: Backend>(
    mlp_model: Mlp_no_bn<B>,
    artifact_dir: &str,
    device: B::Device,
) -> Mlp_no_bn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack + lax::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    mlp_model.load_record(record)
}
