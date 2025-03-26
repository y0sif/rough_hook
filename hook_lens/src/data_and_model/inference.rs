use super::model::{self, DeepLearningModel};
use burn::{
    module::Module,
    prelude::Backend,
    record::{CompactRecorder, Recorder},
    tensor::{Device, Shape, Tensor, TensorData},
};
use opencv::barcode::NONE;

use super::model::{Cnn, CnnRecord, Kan, KanCnn, KanCnnRecord, KanRecord};

#[derive(Module, Debug)]
pub enum ModelEnum<B: Backend> {
    // any new model should be added here first
    Cnn(Cnn<B>),
    // in the kan models and kan_cnn models has  addational parametre --> grid_size, spline_order, scale_base, scale_noise : if you put 0 it will be ignored
    Kan_256(Kan<B>),
    Kan_512(Kan<B>),

    Kan_cnn_256(KanCnn<B>),
    kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(KanCnn<B>),
    kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(KanCnn<B>),
    kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(KanCnn<B>),
    kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_44epoch(KanCnn<B>),
    kan_cnn_256_hook_lens_grid_size_15_spline_order_16_scale_base_3_scale_noise_2(KanCnn<B>),
    kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_112epoch(
        KanCnn<B>,
    ),
    kna_cnn_256_hook_lens_grid_size_20_spline_order_12_scale_base_4_scale_noise_4(KanCnn<B>),
    preTrained_cnn_added_to_kan32(KanCnn<B>)
}

impl<B: Backend> DeepLearningModel<B> for ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
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
            ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_44epoch(
                model,
            ) => model.forward(x),
            ModelEnum::kan_cnn_256_hook_lens_grid_size_15_spline_order_16_scale_base_3_scale_noise_2(
                model,
            ) => model.forward(x),
            ModelEnum::kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_112epoch(
                model
            )=>model.forward(x),
            ModelEnum::kna_cnn_256_hook_lens_grid_size_20_spline_order_12_scale_base_4_scale_noise_4(
                model
            )=>model.forward(x),
            ModelEnum::preTrained_cnn_added_to_kan32(model)=>model.forward(x)

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

        ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_44epoch(
            kan_cnn,
        ) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_44epoch(
                model,
            )
        }

        ModelEnum::kan_cnn_256_hook_lens_grid_size_15_spline_order_16_scale_base_3_scale_noise_2(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_256_hook_lens_grid_size_15_spline_order_16_scale_base_3_scale_noise_2(model)
        }

        ModelEnum::kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_112epoch(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_112epoch(model)
        }

        ModelEnum::kna_cnn_256_hook_lens_grid_size_20_spline_order_12_scale_base_4_scale_noise_4(kan_cnn) => {
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::kna_cnn_256_hook_lens_grid_size_20_spline_order_12_scale_base_4_scale_noise_4(model)
        }
        ModelEnum::preTrained_cnn_added_to_kan32(kan_cnn)=>{
            let model = get_kan_cnn_model_from_record(kan_cnn, artifact_dir, device);
            ModelEnum::preTrained_cnn_added_to_kan32(model)
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
        1 => ModelEnum::Cnn(Cnn::new(13, device, 256)),

        10 => ModelEnum::Kan_256(Kan::new(13, device, 256, vec![None , None ,None ,None])),
        11 => ModelEnum::Kan_512(Kan::new(13, device, 512, vec![None , None ,None ,None])),

        20 => ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            KanCnn::new(13, device, 256, vec![Some(15) ,Some(12) ,Some(4),Some(2)]), 
        ),
        21 => ModelEnum::kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            KanCnn::new(13, device, 512, vec![Some(15) ,Some(12) ,Some(4),Some(2)]),
        ),
        22 => ModelEnum::kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2(
            KanCnn::new(13, device, 1024, vec![Some(15) ,Some(12) ,Some(4),Some(2)]),
        ),
        23 => {
            ModelEnum::kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_44epoch(
                KanCnn::new(13, device, 256, vec![Some(15) ,Some(12) ,Some(4),Some(2)]),
            )
        }
        24 => ModelEnum::kan_cnn_256_hook_lens_grid_size_15_spline_order_16_scale_base_3_scale_noise_2(
            KanCnn::new(13, device, 256, vec![Some(15) ,Some(16) ,Some(3),Some(2)]),
        ),
        25 => ModelEnum::kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2_112epoch(
            KanCnn::new(13, device, 1024, vec![Some(15) ,Some(12) ,Some(4),Some(2)]),
        ),
        26 => ModelEnum::kna_cnn_256_hook_lens_grid_size_20_spline_order_12_scale_base_4_scale_noise_4(
            KanCnn::new(13, device, 256, vec![Some(20) ,Some(12) ,Some(4),Some(2)]),
        ),
        27 => ModelEnum::preTrained_cnn_added_to_kan32(
            KanCnn::new(13, device, 32, vec![None , None , None , None]),
        ),
        

        _ => panic!("not valid model Id"),
    }
}

fn get_kan_cnn_model_from_record<B: Backend>(
    kan_cnn_model: KanCnn<B>,
    artifact_dir: &str,
    device: B::Device,
) -> KanCnn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    kan_cnn_model.load_record(record)
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
