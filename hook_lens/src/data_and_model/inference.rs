use burn::{module::Module, prelude::Backend, record::{CompactRecorder, Recorder}, tensor::{Shape, Tensor, TensorData}};

use super::model::{Cnn, CnnRecord};
use super::model::{Kan, KanRecord};
use super::model::{Cnn_Kan , Cnn_KanRecord};


pub fn infer<B: Backend> (artifact_dir: &str, id : i8 ,  device: B::Device , image: Vec<u8>)->u8
where
        B::IntElem: TryInto<u8> + std::fmt::Debug,
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    match id {
        1 => infer_cnn::<B>(artifact_dir , device , image),
        2 => infer_kan::<B>(artifact_dir , device , image),
        3 => infer_Cnn_Kan::<B>(artifact_dir , device , image),
        _ => panic!("Model with id {} not found", id),
    }
}

fn infer_cnn<B:Backend>(artifact_dir: &str , device: B::Device , image: Vec<u8>)->u8 
where
        B::IntElem: TryInto<u8> + std::fmt::Debug,{
    let record: CnnRecord<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");
    let model = Cnn::new(13, &device);
    
    let model = model.load_record(record);
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

pub fn infer_kan<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: KanRecord<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = Kan::new(13, &device);
    
    let model = model.load_record(record);
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


pub fn infer_Cnn_Kan<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: Cnn_KanRecord<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = Cnn_Kan::new(13, &device);
    
    let model = model.load_record(record);
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
