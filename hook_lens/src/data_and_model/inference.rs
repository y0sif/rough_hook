use burn::{module::Module, prelude::Backend, record::{CompactRecorder, Recorder}, tensor::{Device, Shape, Tensor, TensorData}};
use super::model::{self, DeepLearningModel};
use std::marker::PhantomData;
// use cnn models
use super::model::{Cnn, CnnRecord};
// use kan models
use super::model::{Kan, KanRecord};
use super::model::{Kan512 , Kan512Record};
use super::model::{Kan1024 , Kan1024Record};
use super::model::{kan_256_spline_order_12 , kan_256_spline_order_12Record};
use super::model::{kan_256_spline_order_6 , kan_256_spline_order_6Record};
use super::model::{kan_256_scale_base_2_scale_noise_2 , kan_256_scale_base_2_scale_noise_2Record};
use super::model::{kan_256_scale_base_4_scale_noise_4 , kan_256_scale_base_4_scale_noise_4Record};
use super::model::{kan_256_scale_base_6_scale_noise_6 , kan_256_scale_base_6_scale_noise_6Record};
use super::model::{kan_256_grid_size_10 , kan_256_grid_size_10Record};
use super::model::{kan_256_grid_size_20 , kan_256_grid_size_20Record};
use super::model::{kan_256_grid_size_30 , kan_256_grid_size_30Record};
//use cnn_kan models
use super::model::{kan_cnn_256 , kan_cnn_256Record};
use super::model::{kan_cnn_512 , kan_cnn_512Record};
use super::model::{kan_cnn_1024 , kan_cnn_1024Record};
use super::model::{kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2 , kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2Record};
use super::model::{kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2 , kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2Record};




#[derive(Module, Debug )]
pub enum ModelEnum<B:Backend>{
    cnn(Cnn<B>) ,
    kan_cnn_256(kan_cnn_256<B>)
}

impl<B:Backend> DeepLearningModel<B> for ModelEnum<B> 
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn new(num_classes: usize, device: &Device<B>)->Self
    {
        panic!("Must use the model function it self");
    }

    fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        match self {
            ModelEnum::cnn(model) => model.forward(x),
            ModelEnum::kan_cnn_256(model) => model.forward(x),
        }
    }
}


pub fn infer_model<B: Backend>(model: &ModelEnum<B> , device: B::Device ,  image: Vec<u8>) -> u8 
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

pub fn load_model_paramter<B: Backend>(id : i8 , artifact_dir: &str , device: B::Device)-> ModelEnum<B> 
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{

    let model_object = get_model_object(id, &device);
    

    let inner_model = match model_object {
        ModelEnum::cnn(cnn_model) =>{
            let record= CompactRecorder::new()
            .load(format!("{artifact_dir}/model").into(), &device)
            .expect("Trained model should exist");
            let model = cnn_model.load_record(record);
            ModelEnum::cnn(model)
            
        } , 
        ModelEnum::kan_cnn_256(kan_cnn_256) =>{
            let record= CompactRecorder::new()
            .load(format!("{artifact_dir}/model").into(), &device)
            .expect("Trained model should exist");
            let model = kan_cnn_256.load_record(record);
            ModelEnum::kan_cnn_256(model)
        }
    };

    return inner_model;
}




fn get_model_object<B:Backend>(id : i8 , device: &B::Device)->ModelEnum<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{

    match id {
        1=> ModelEnum::cnn(Cnn::new(13 , device)) ,
        13=>ModelEnum::kan_cnn_256(kan_cnn_256::new(13, device)),
        _=> panic!("not valid model Id")
    }
}
