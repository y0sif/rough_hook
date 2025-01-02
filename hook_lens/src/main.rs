use burn::backend::{Autodiff, Wgpu};
use burn::optim::momentum::MomentumConfig;
use burn::optim::SgdConfig;
use fen_string_generation::*;
// use hook_lens::inference::infer;
use hook_lens::data_and_model::training::{train, TrainingConfig};
use hook_lens::input_data_handling::*;

fn main() {
    // println!("this is the computer vision");
    // // infer();

    // let device = burn::backend::wgpu::WgpuDevice::BestAvailable;
    // train::<Autodiff<Wgpu>>(
    //     TrainingConfig::new(SgdConfig::new().with_momentum(Some(MomentumConfig {
    //         momentum: 0.9,
    //         dampening: 0.,
    //         nesterov: false,
    //     }))),
    //     device
    // );

    let board_image_path = "/home/mostafayounis630/Graduation_Project/rough_hook/hook_lens/test_images/input_img.png";
    get_fen_string_from(board_image_path);
    
}
