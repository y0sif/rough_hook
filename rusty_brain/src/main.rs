use rusty_brain::uci;
use nnue::training::TrainingConfig;
use burn::{backend::{ndarray::NdArrayDevice, wgpu::WgpuDevice}, config::Config, module::Module, record::{CompactRecorder, Recorder}};
use burn::backend::NdArray;
use burn_cuda::{CudaDevice, Cuda};
fn main() {
    let device = NdArrayDevice::Cpu;
    let artifact_dir = "/home/y0sif/nnue_models/nnue";
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = config.model.init::<NdArray<f32, i8>>(&device).load_record(record);

    let mut uci = uci::Uci::<NdArray<f32, i8>>::new(model, device.clone());
    uci.listen();
}
