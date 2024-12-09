use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig, 
};
use burn_dataset::transform::Mapper;
use nnue::{data::{self, RawToItem}, model::ModelConfig, training::TrainingConfig};

fn main() {
    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    let artifact_dir = "/tmp/guide";
    nnue::training::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(), AdamConfig::new()),
        device.clone(),
    );

    // let chess_raw = data::ChessPositionRaw{
    //     fen: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K3 w Qkq - 0 1"),
    //     evaluation: Some(-1500.0),
    // };
    
    // let mapper = RawToItem;

    // let chess_item = mapper.map(&chess_raw);

    // nnue::inference::infer::<MyBackend>(
    //     artifact_dir,
    //     device,
    //     chess_item,
    // );


}