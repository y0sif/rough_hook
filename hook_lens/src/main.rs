use burn::{backend::{Autodiff, Wgpu}, optim::AdamConfig};
use hook_lens::data_and_model::training::{train, TrainingConfig};
use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let device = CudaDevice::default();
    train::<Autodiff<Cuda<f32, i32>>>(
        TrainingConfig::new(AdamConfig::new()),
        device
    );
}
