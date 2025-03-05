use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig, 
};
use rough_guard::{data, model::ModelConfig, training::TrainingConfig};

use burn_cuda::{CudaDevice, Cuda};

fn main() {
    let device = CudaDevice::default();

    //type MyBackend = Wgpu<f32, i32>;
    //type MyAutodiffBackend = Autodiff<MyBackend>;

    //let device = burn::backend::wgpu::WgpuDevice::BestAvailable;

    let artifact_dir = "/tmp/rough_guard";

    // CUDA CODE
     rough_guard::training::train::<Autodiff<Cuda<f32, i32>>>(
         artifact_dir,
         TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
         device.clone(),
     );

    // TEST DB
    //data::test_deserialization();

    //rough_guard::training::train::<Autodiff<MyAutodiffBackend>>(
    //         artifact_dir,
    //         TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
    //         device.clone()
    //);
}
