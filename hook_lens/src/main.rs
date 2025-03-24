use burn::{backend::Autodiff, module::Module, optim::AdamConfig, record::{CompactRecorder, Recorder}};
use hook_lens::data_and_model::{model::{Cnn, CnnRecord, CustomKanCnn}, training::{train, TrainingConfig}};
use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let artifact_dir = "/tmp/hook_lens";
    let device = CudaDevice::default();

    let record: CnnRecord<Autodiff<Cuda<f32, i32>>> = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = Cnn::new(13, &device);
    
    let model = model.load_record(record);
    
    let custom_model = CustomKanCnn::new(
        model.conv1.no_grad().clone(), 
        model.conv2.no_grad().clone(), 
        model.conv3.no_grad().clone(), 
        model.conv4.no_grad().clone(), 
        model.conv5.no_grad().clone(), 
        model.conv6.no_grad().clone(), 
        model.batch1.no_grad().clone(), 
        model.batch2.no_grad().clone(), 
        model.batch3.no_grad().clone(), 
        model.batch4.no_grad().clone(), 
        model.batch5.no_grad().clone(), 
        model.batch6.no_grad().clone(), 13, &device);

    train::<Autodiff<Cuda<f32, i32>>>(
        custom_model,
        TrainingConfig::new(AdamConfig::new()),
        device
    );
}
