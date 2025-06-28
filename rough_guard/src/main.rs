use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig, tensor::Tensor,
};
use rough_guard::{
    data::{self, ChessGameDataSet},
    inference::ModelEnum,
    model::{Mlp, ModifiedKan},
    training::{TrainingConfig},
    training::compute_class_weights
};
use burn_cuda::{Cuda, CudaDevice};
use serde::de;
use std::io::{self, Write};
use burn::backend::ndarray::NdArray;


//type MyBackend = Autodiff<Autodiff<Autodiff<Cuda>>>;
//type MyBackend = Autodiff<Autodiff<Autodiff<NdArray>>>;
fn main() {
    let artifact_dir = "/home/khaled/final_mlp_bn2";
    //let artifact_dir = "/home/khaled/kan";

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

    let device: CudaDevice = CudaDevice::default();    
    //let device = burn::backend::ndarray::NdArrayDevice::default();


    let train_dataset = ChessGameDataSet::train();
    let class_weights = compute_class_weights::<Autodiff<Cuda<f32, i32>>>(&train_dataset, &device);

    let mlp_model= ModelEnum::Mlp(
        Mlp::new(vec![(7, 7), (7, 7), (7, 7), (7, 6), (6, 6), (6, 6), (6, 5), (5, 5), (5, 5), (5, 4),
        (4, 4), (4, 4), (4, 3), (3, 3), (3, 3), (3, 2), (2, 2), (2, 2)],
        class_weights, 0.15, &device)
    );

    // let kan_model = ModelEnum::ModifiedKan(ModifiedKan::new(
    //     vec![
    //         ([7, 6, 5], [Some(20), Some(3), None, None]),
    //         ([5, 4, 3], [Some(10), Some(2), None, None]),
    //         ([3, 2, 2], [Some(5), Some(1), None, None]),
    //     ],
    //     class_weights,
    //     &device,)
    // );

    rough_guard::training::train::<Autodiff<Cuda<f32, i32>>>(
        artifact_dir,
        TrainingConfig::new(AdamConfig::new()),
        device.clone(),
        mlp_model
        //kan_model,
    );
    // TEST DB
    //data::test_deserialization();

    println!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}