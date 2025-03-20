use burn::{backend::Autodiff, optim::AdamConfig};
use hook_lens::data_and_model::training::{train, TrainingConfig};
use burn::backend::libtorch::{LibTorch, LibTorchDevice};

fn main() {
    let device = LibTorchDevice::Cuda(0);
    train::<Autodiff<LibTorch>>(
        TrainingConfig::new(AdamConfig::new()),
        device
    );
}
