use super::model::DeepLearningModel;
use super::model::{ModifiedKan, ModifiedKanRecord, Mlp, MlpRecord};
use crate::data::{ChessGameBatcher, ChessGameItem};
use burn::{
    data::dataloader::batcher::Batcher,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{activation::softmax, cast::ToElement},
};

#[derive(Module, Debug)]
pub enum ModelEnum<B: Backend> {
    Mlp(Mlp<B>),
    ModifiedKan(ModifiedKan<B>),
}

impl<B: Backend> DeepLearningModel<B> for ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        match self {
            ModelEnum::Mlp(model) => model.forward(games),
            ModelEnum::ModifiedKan(model) => model.forward(games),
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
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
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
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
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
    };
    trained_model
}

fn get_model_archi<B: Backend>(
    id: i8,
    class_weights: Tensor<B, 1>,
    device: &B::Device,
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    match id {
        // mlp model with 12 linerar layer
        1 => ModelEnum::Mlp(Mlp::new(
            vec![
                (241, 64),
                (64, 128),
                (128, 256),
                (256, 512),
                (512, 1024),
                (1024, 512),
                (512, 256),
                (256, 128),
                (128, 64),
                (64, 4),
            ],
            class_weights,
            device,
        )),
        2 => ModelEnum::Mlp(Mlp::new(
            vec![
                (241, 64),
                (64, 128),
                (128, 256),
                (256, 512),
                (512, 1024),
                (1024, 2048),
                (2048, 1024),
                (1024, 512),
                (512, 256),
                (256, 128),
                (128, 64),
                (64, 4),
            ],
            class_weights,
            device,
        )),
        // kan model with 2 kan layers
        _ => ModelEnum::ModifiedKan(ModifiedKan::new(
            vec![
                (vec![241, 256, 128], vec![Some(6), Some(6), None, None]),
                (vec![128, 64, 4], vec![Some(6), Some(6), None, None]),
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
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
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
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    mlp_model.load_record(record)
}
