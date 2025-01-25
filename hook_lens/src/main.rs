use burn::{backend::{Autodiff, Wgpu}, optim::AdamConfig};
use hook_lens::data_and_model::training::{train, TrainingConfig};

fn main() {
    let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    train::<Autodiff<Wgpu>>(
        TrainingConfig::new(AdamConfig::new()),
        device
    );
}