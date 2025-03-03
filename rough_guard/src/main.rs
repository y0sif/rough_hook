use burn::{
    backend::Autodiff,
    optim::AdamConfig, 
};
use rough_guard::{model::ModelConfig, training::TrainingConfig};
use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let device = CudaDevice::default();
    let artifact_dir = "/tmp/rough_guard";
    rough_guard::training::train::<Autodiff<Cuda<f32, i32>>>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
        device.clone(),
    );
}