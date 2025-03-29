use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
};
use rough_guard::data;

use burn_cuda::{Cuda, CudaDevice};

fn main() {
    let artifact_dir = "/tmp/rough_guard";

    //Wgpu Code
    //type MyBackend = Wgpu<f32, i32>;
    //type MyAutodiffBackend = Autodiff<MyBackend>;
    //let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    //rough_guard::training::train::<Autodiff<MyAutodiffBackend>>(
    //    artifact_dir,
    //    TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
    //    device.clone()
    //);

    // CUDA CODE
    // let device = CudaDevice::default();
    // rough_guard::training::train::<Autodiff<Cuda<f32, i32>>>(
    //     artifact_dir,
    //     TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
    //     device.clone(),
    // );

    // TEST DB
    //data::test_deserialization();
}
