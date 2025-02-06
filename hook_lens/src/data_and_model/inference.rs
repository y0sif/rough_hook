use burn::{module::Module, prelude::Backend, record::{CompactRecorder, Recorder}, tensor::{Shape, Tensor, TensorData}};
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


pub fn infer<B: Backend> (artifact_dir: &str, id : i8 ,  device: B::Device , image: Vec<u8>)->u8
where
        B::IntElem: TryInto<u8> + std::fmt::Debug,
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    match id {
        //cnn models
        1 => infer_cnn::<B>(artifact_dir , device , image),
        //Kan_models
        2 => infer_kan::<B>(artifact_dir , device , image),
        3 => infer_kan512::<B>(artifact_dir , device , image),
        4 => infer_kan1024::<B>(artifact_dir , device , image),
        5 => infer_kan_256_spline_order_6::<B>(artifact_dir , device , image),
        6 => infer_kan_256_spline_order_12::<B>(artifact_dir , device , image),
        7 => infer_kan_256_scale_base_2_scale_noise_2::<B>(artifact_dir , device , image),
        8 => infer_kan_256_scale_base_4_scale_noise_4::<B>(artifact_dir , device , image),
        9 => infer_kan_256_scale_base_6_scale_noise_6::<B>(artifact_dir , device , image),
        10 => infer_kan_256_grid_size_10::<B>(artifact_dir , device , image),
        11 => infer_kan_256_grid_size_20::<B>(artifact_dir , device , image),
        12 => infer_kan_256_grid_size_30::<B>(artifact_dir , device , image),
        //kan_cnn models
        13 => infer_kan_cnn_256::<B>(artifact_dir , device , image),
        14 => infer_kan_cnn_512::<B>(artifact_dir , device , image),
        15 => infer_kan_cnn_1024::<B>(artifact_dir , device , image),

        16 => infer_kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2::<B>(artifact_dir , device , image),
        17 => infer_kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2::<B>(artifact_dir , device , image),
        _ => panic!("Model with id {} not found", id),
    }
}

//================================= infer Cnn models ====================================//
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
//==================================== infer Kan models ==============================================//
// kan 256
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
//kan 512
pub fn infer_kan512<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: Kan512Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = Kan512::new(13, &device);
    
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
// kan 1024
pub fn infer_kan1024<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: Kan1024Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = Kan1024::new(13, &device);
    
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

//kan_256_spline_order_6
pub fn infer_kan_256_spline_order_6<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
    B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_spline_order_6Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_spline_order_6::new(13, &device);

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

//kan_256_spline_order_12
pub fn infer_kan_256_spline_order_12<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_spline_order_12Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_spline_order_12::new(13, &device);
    
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

//kan_256_scale_base_2_scale_noise_2
pub fn infer_kan_256_scale_base_2_scale_noise_2<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_scale_base_2_scale_noise_2Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_scale_base_2_scale_noise_2::new(13, &device);
    
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

//kan_256_scale_base_4_scale_noise_4
pub fn infer_kan_256_scale_base_4_scale_noise_4<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_scale_base_4_scale_noise_4Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_scale_base_4_scale_noise_4::new(13, &device);
    
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

//kan_256_scale_base_6_scale_noise_6
pub fn infer_kan_256_scale_base_6_scale_noise_6<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_scale_base_6_scale_noise_6Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_scale_base_6_scale_noise_6::new(13, &device);
    
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

//kan_256_grid_size_10
pub fn infer_kan_256_grid_size_10<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_grid_size_10Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_grid_size_10::new(13, &device);
    
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

//kan_256_grid_size_20
pub fn infer_kan_256_grid_size_20<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_grid_size_20Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_grid_size_20::new(13, &device);
    
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

//kan_256_grid_size_30
pub fn infer_kan_256_grid_size_30<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_256_grid_size_30Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_256_grid_size_30::new(13, &device);
    
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








//======================================== infer Cnn_Kan models ===========================//
//infer kan_cnn_256
pub fn infer_kan_cnn_256<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_cnn_256Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_cnn_256::new(13, &device);
    
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
//infer kan_cnn_512
pub fn infer_kan_cnn_512<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_cnn_512Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_cnn_512::new(13, &device);
    
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
//infer kan_cnn_1024
pub fn infer_kan_cnn_1024<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_cnn_1024Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_cnn_1024::new(13, &device);
    
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

//infer kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2
pub fn infer_kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_cnn_256_grid_size_10_spline_order_6_scale_base_2_scale_noise_2::new(13, &device);
    
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

//infer kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2
pub fn infer_kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2<B: Backend> (artifact_dir: &str,  device: B::Device , image: Vec<u8>)->u8
where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
        B::IntElem: TryInto<u8> + std::fmt::Debug,
{
    let record: kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2Record<B> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = kan_cnn_256_grid_size_20_spline_order_8_scale_base_2_scale_noise_2::new(13, &device);
    
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
