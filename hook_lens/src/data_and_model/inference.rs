use burn::{module::Module, prelude::Backend, record::{CompactRecorder, Recorder}, tensor::{Shape, Tensor, TensorData}};

use super::model::{Cnn, CnnRecord};


pub fn infer<B: Backend> (artifact_dir: &str, id : i8 ,  device: B::Device , image: Vec<u8>)->u8
where
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    match id {
        1 => infere_cnn::<B>(artifact_dir , device , image),
        2 => infere_resnet::<B>(artifact_dir , device , image),
        _ => panic!("Model with id {} not found", id),
    }
}

fn infere_cnn<B:Backend>(artifact_dir: &str , device: B::Device , image: Vec<u8>)->u8 
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

fn infere_resnet<B:Backend>(artifact_dir: &str , device: B::Device , image: Vec<u8>)->u8 
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

