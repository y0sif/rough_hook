use burn::{
    backend::Autodiff,
    optim::AdamConfig, 
};
use nnue::{model::ModelConfig, training::TrainingConfig};
use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let device = CudaDevice::default();
    let artifact_dir = "/tmp/nnue";
    nnue::training::train::<Autodiff<Cuda<f32, i32>>>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
        device.clone(),
    );
}