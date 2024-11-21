use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig, 
};
use nnue::{model::ModelConfig, training::TrainingConfig};

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
    //     fen: String::from("rnbqkb1r/pppppppp/5n2/8/4P1Q1/8/PPPP1PPP/RNB1KBNR b KQkq - 2 2"),
    //     evaluation: String::from("0"),
    // };
    // let mapper = data::RawToItem;
    // let chess_item = mapper.map(&chess_raw);

    // crate::inference::infer::<MyBackend>(
    //     artifact_dir,
    //     device,
    //     chess_item,
    // );


}