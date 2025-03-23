use rusty_brain::uci;
use kanue::training::TrainingConfig;
use burn::{backend::wgpu::WgpuDevice, config::Config, module::Module, record::{CompactRecorder, Recorder}};
use burn::backend::Wgpu;
use burn_cuda::{CudaDevice, Cuda};
fn main() {
    let device = CudaDevice::new(0);
    let artifact_dir = "/home/y0sif/kanue_models/kanue_1024";
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = config.model.init::<Cuda<f32, i32>>(&device).load_record(record);

    let mut uci = uci::Uci::<Cuda<f32, i32>>::new(model, device.clone());
    uci.listen();
}
