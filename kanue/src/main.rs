use burn::{
    backend::Autodiff,
    optim::AdamConfig, 
};
use kanue::{model::ModelConfig, training::TrainingConfig};
use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let device = CudaDevice::default();
    let artifact_dir = "/tmp/kanue";
    kanue::training::train::<Autodiff<Cuda<f32, i32>>>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
        device.clone(),
    );
}