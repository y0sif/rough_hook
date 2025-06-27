use std::env;
fn main() {
    use burn_cuda::{Cuda, CudaDevice};
    use fen_string_generation::*;
    use hook_lens::{
        data_and_model::inference::{load_model_paramter, ModelEnum},
        input_data_handling::*,
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let image_path = &args[1];

        let model_path = "/home/sasa/My_Projects/Graduation_Project/rough_hook/hook_lens/hook_lens_models/cnn_models/cnn_hook_lens";

        let model: ModelEnum<Cuda<f32, i32>> =
            load_model_paramter::<Cuda<f32, i32>>(1, &model_path, CudaDevice::default());

        let mut predicted_fen_string = get_fen_string_from(image_path, model);

        println!("{}", predicted_fen_string);

        //let fen_string = get_fen_string_from(board_image_path, model_path, 1);
        // println!("Fen String : {}", fen_string);

        // println!("|-------------------------- Testing ---------------------------|");
        // println!("| Testing in Debug Mode : \'cargo test --release -p hook_lens\'  |");
        // println!("| Testing in Development Mode : \'cargo test -p hook_lens\'      |");
        // println!("|--------------------------------------------------------------|");
    }
}
