use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamWConfig, 
};
use kanue::{model::KanConfig, training::{KanTrainingConfig, train}};
use burn_efficient_kan::KanOptions;

fn main() {
    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    let artifact_dir = "/tmp/guide";
    let config_optimizer = AdamWConfig::new().with_weight_decay(1e-4);

    train::<MyAutodiffBackend>(
        artifact_dir,
        KanTrainingConfig::new(KanConfig::new(), config_optimizer,
        KanOptions::new([768, 2, 1])),
        device.clone()
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