use rusty_brain::uci;
use nnue::training::TrainingConfig;
use burn::{backend::wgpu::WgpuDevice, config::Config, module::Module, record::{CompactRecorder, Recorder}};
use burn::backend::Wgpu;
fn main() {
    let device = WgpuDevice::default();
    let artifact_dir = "/tmp/nnue";
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = config.model.init::<Wgpu<f32, i32>>(&device).load_record(record);

    let mut uci = uci::Uci::<Wgpu<f32, i32>>::new(model);
    uci.listen();
}
