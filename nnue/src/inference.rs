// use burn::{backend::wgpu::WgpuDevice, config::Config, data::dataloader::batcher::Batcher, module::Module, prelude::Backend, record::{CompactRecorder, Recorder}, tensor::Tensor};

// use crate::{data::{ChessPositionBatcher, ChessPositionItem}, model::Model, training::TrainingConfig};

// pub fn infer<B: Backend>(model: Model<B>, device: B::Device, features: Vec<i8>) -> i32{
//     let device = WgpuDevice::default();
//     let tensor = Tensor::from_data(&*features, &device);
//     let output = model.infer(tensor) * 400.0;

//     let predicted = output.flatten::<1>(0, 1).into_data().iter::<f32>().next().unwrap().clone();

//     predicted as i32
// }