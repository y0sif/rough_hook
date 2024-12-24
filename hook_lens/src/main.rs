use burn::backend::{Autodiff, Wgpu};
use burn::optim::momentum::MomentumConfig;
use burn::optim::SgdConfig;
// use hook_lens::inference::infer;
use hook_lens::training::{train, TrainingConfig};
fn main() {
    println!("this is the computer vision");
    // infer();

    let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    train::<Autodiff<Wgpu>>(
        TrainingConfig::new(SgdConfig::new().with_momentum(Some(MomentumConfig {
            momentum: 0.9,
            dampening: 0.,
            nesterov: false,
        }))),
        device
    );
}
