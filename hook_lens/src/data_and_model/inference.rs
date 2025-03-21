use super::model::DeepLearningModel;
use burn::{
    module::Module,
    prelude::Backend,
    record::{CompactRecorder, Recorder},
    tensor::{Device, Shape, Tensor, TensorData},
};

use super::model::{kan, kanRecord, kan_cnn, kan_cnnRecord, Cnn, CnnRecord};

#[derive(Module, Debug)]
pub enum ModelEnum<B: Backend> {
    // any new model should be added here first
    Cnn(Cnn<B>),
    // in the kan models and kan_cnn models has  addational parametre --> grid_size, spline_order, scale_base, scale_noise : if you put 0 it will be ignored
    Kan_256(kan<B, 256, 0, 0, 0, 0>),
    Kan_512(kan<B, 512, 0, 0, 0, 0>),

    Kan_cnn_256(kan_cnn<B, 256, 0, 0, 0, 0>),
    kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
        kan_cnn<B, 256, 15, 12, 4, 2>,
    ),
    kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
        kan_cnn<B, 512, 15, 12, 4, 2>,
    ),
    kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
        kan_cnn<B, 1024, 15, 12, 4, 2>,
    ),
}

impl<B: Backend> DeepLearningModel<B> for ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn new(num_classes: usize, device: &Device<B>) -> Self {
        panic!("Must use the model function it self");
    }

    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        match self {
            ModelEnum::Cnn(model) => model.forward(x),

            ModelEnum::Kan_256(model) => model.forward(x),
            ModelEnum::Kan_512(model) => model.forward(x),

            ModelEnum::Kan_cnn_256(model) => model.forward(x),
            ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
                model,
            ) => model.forward(x),
            ModelEnum::kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
                model,
            ) => model.forward(x),
            ModelEnum::kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
                model,
            ) => model.forward(x),
        }
    }
}

pub fn infer_model<B: Backend>(model: &ModelEnum<B>, device: B::Device, image: Vec<u8>) -> u8
where
    B::IntElem: TryInto<u8> + std::fmt::Debug,
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let img = TensorData::new(image, Shape::new([32, 32, 3]));
    let img = Tensor::<B, 3>::from_data(img.convert::<B::FloatElem>(), &device)
        .swap_dims(2, 1) // [H, C, W]
        .swap_dims(1, 0); // [C, H, W]
    let img = img / 255.0;
    let img = img.unsqueeze();
    let output = model.forward(img);
    let predicted = output.argmax(1).flatten::<1>(0, 1).into_scalar();
    let result: Result<u8, _> = predicted.try_into();

    match result {
        Ok(value) => value,
        Err(_) => panic!("Failed to convert prediction to u8"),
    }
}

pub fn load_model_paramter<B: Backend>(
    id: i8,
    artifact_dir: &str,
    device: B::Device,
) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let model_object = get_model_object(id, &device);

    let inner_model = match model_object {
        ModelEnum::Cnn(cnn_model) => {
            let record = CompactRecorder::new()
                .load(format!("{artifact_dir}/model").into(), &device)
                .expect("Trained model should exist");
            let model = cnn_model.load_record(record);
            ModelEnum::Cnn(model)
        }
        ModelEnum::Kan_256(kan_model) => {
            let model = get_kan_model_from_record(kan_model, artifact_dir, device);
            ModelEnum::Kan_256(model)
        }
        ModelEnum::Kan_512(kan_model) => {
            let model = get_kan_model_from_record(kan_model, artifact_dir, device);
            ModelEnum::Kan_512(model)
        }
        ModelEnum::Kan_cnn_256(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::Kan_cnn_256(model)
        }
        ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(model)
        }
        ModelEnum::kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(model)
        }
        ModelEnum::kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            kan_cnn,
        ) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(model)
        }
    };

    return inner_model;
}

//from : 1---> 9 cnn models
//from : 10---> 19 kan models
//from : 20---> 29 cnn_kan models
fn get_model_object<B: Backend>(id: i8, device: &B::Device) -> ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    match id {
        1 => ModelEnum::Cnn(Cnn::new(13, device)),

        10 => ModelEnum::Kan_256(kan::new(13, device)),
        11 => ModelEnum::Kan_512(kan::new(13, device)),

        20 => ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            kan_cnn::new(13, device),
        ),
        21 => ModelEnum::kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            kan_cnn::new(13, device),
        ),
        22 => ModelEnum::kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            kan_cnn::new(13, device),
        ),
        _ => panic!("not valid model Id"),
    }
}

fn get_kan_cnn_model_from_record<
    B: Backend,
    const hidden_layer_size: usize,
    const grid_size: u16,
    const spline_order: u32,
    const scale_base: i32,
    const scale_noise: i32,
>(
    kan_cnn_model: kan_cnn<B, hidden_layer_size, grid_size, spline_order, scale_base, scale_noise>,
    artifact_dir: &str,
    device: B::Device,
) -> kan_cnn<B, hidden_layer_size, grid_size, spline_order, scale_base, scale_noise>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    kan_cnn_model.load_record(record)
}

fn get_kan_model_from_record<
    B: Backend,
    const hidden_layer_size: usize,
    const grid_size: u16,
    const spline_order: u32,
    const scale_base: i32,
    const scale_noise: i32,
>(
    kan_model: kan<B, hidden_layer_size, grid_size, spline_order, scale_base, scale_noise>,
    artifact_dir: &str,
    device: B::Device,
) -> kan<B, hidden_layer_size, grid_size, spline_order, scale_base, scale_noise>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    kan_model.load_record(record)
}
