use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
};
use rough_guard::{data::{self, ChessGameDataSet}, inference::ModelEnum, model::{Mlp, ModifiedKan}, training::{compute_class_weights, TrainingConfig}};

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
    let device = CudaDevice::default();
    let train_dataset = ChessGameDataSet::train();
    let class_weights = compute_class_weights::<Autodiff<Cuda<f32, i32>>>(&train_dataset, &device);

    // let mlp_model = ModelEnum::Mlp(
    //     Mlp::new(vec![], class_weights.clone(), &device)
    // );

    let kan_model = ModelEnum::ModifiedKan(
        ModifiedKan::new(vec![], class_weights, &device)
    );

    rough_guard::training::train::<Autodiff<Cuda<f32, i32>>>(
        artifact_dir,
        TrainingConfig::new(AdamConfig::new()),
        device.clone(),
        //mlp_model
        kan_model
    );

    // TEST DB
    //data::test_deserialization();
}
