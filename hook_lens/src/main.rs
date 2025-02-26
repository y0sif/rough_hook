

fn main() {
    #[cfg(debug_assertions)]
    {

        use hook_lens::input_data_handling::*;
        use fen_string_generation::*;
        
        let model_path = "/home/sasa630/Graduation_Project/hook_lens_models/hook_lens_cnn";
        let board_image_path = "/home/sasa630/Graduation_Project/test_images/test_real_time_image.png";
        //let fen_string = get_fen_string_from(board_image_path, model_path, 1);
       // println!("Fen String : {}", fen_string);

        // println!("|-------------------------- Testing ---------------------------|");
        // println!("| Testing in Debug Mode : \'cargo test --release -p hook_lens\'  |");
        // println!("| Testing in Development Mode : \'cargo test -p hook_lens\'      |"); 
        // println!("|--------------------------------------------------------------|");
    }
    
} 