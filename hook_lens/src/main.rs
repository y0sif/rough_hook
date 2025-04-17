use burn::backend::wgpu::WgpuDevice;
use burn::backend::Wgpu;
use burn::module::Module;
use burn::record::{CompactRecorder, Recorder};
use hook_lens::data_and_model::model::Cnn;
use hook_lens::input_data_handling::fen_string_generation::get_fen_string_from;
use opencv::highgui::wait_key_def;
use opencv::{prelude::*, imgcodecs, highgui, imgproc};
use reqwest;
use rusty_brain::board::Board;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // put url from ip webcam app here    
    let url = "http://192.168.1.18:8080/shot.jpg";
    let model_path = "/home/y0sif/models/layer_128_then_64";
    let device = WgpuDevice::default();
    let record = CompactRecorder::new()
        .load(format!("{model_path}/model").into(), &device)
        .expect("Trained model should exist");
    let model: Cnn<Wgpu> = Cnn::new(13, &device, 0);
    let model = model.load_record(record);
    
    let img_path = "";
    let img = imgcodecs::imread(img_path, imgcodecs::IMREAD_COLOR)?;
    // Display the image
    highgui::imshow("Android_cam", &img)?;
    wait_key_def().unwrap();

    let mut buf = opencv::core::Vector::<u8>::new();
    imgcodecs::imencode(".jpg", &img, &mut buf, &opencv::core::Vector::<i32>::new())?;
    let cropped_img_data = buf.to_vec();

    // Wait for key press and check for 'f' (FEN extraction) or 'Esc' (quit)
    println!("Processing image for FEN string extraction...");
    let static_str = String::from(" w - - 0 1");
    let start = Instant::now();
    let predicted_fen_string = get_fen_string_from(cropped_img_data, model.clone(), &device);
    let duration = start.elapsed().as_secs_f32();

    println!("====================== predicted board =======================");

    let mut predicted_board = Board::from_fen(predicted_fen_string.clone() + &static_str);
    predicted_board.print_board();

    println!("predicted fen string = {}", predicted_fen_string);

    println!("duaration = {:?}", duration);

    println!("\n\n");
    
    return Ok(());

    // Create a window
    highgui::named_window("Android_cam", highgui::WINDOW_AUTOSIZE)?;
    println!("Press 'f' to extract FEN string, 'Esc' to quit");

    loop {
        // Fetch image bytes from URL
        let response = reqwest::blocking::get(url)?; 
        let bytes = response.bytes()?;
        let img_data = bytes.to_vec();

        // Decode the image
        let img_mat = imgcodecs::imdecode(&opencv::core::Mat::from_slice(&img_data)?, imgcodecs::IMREAD_COLOR)?;

        // Define the cropping region (x, y, width, height)
        // Adjust these values as needed
        let crop_rect = opencv::core::Rect::new(580, 0, 1080, 1080); 

        // Ensure the crop_rect is within the image bounds
        let validated_rect = opencv::core::Rect {
            x: crop_rect.x.max(0),
            y: crop_rect.y.max(0),
            width: if crop_rect.x + crop_rect.width > img_mat.cols() { img_mat.cols() - crop_rect.x.max(0) } else { crop_rect.width },
            height: if crop_rect.y + crop_rect.height > img_mat.rows() { img_mat.rows() - crop_rect.y.max(0) } else { crop_rect.height },
        };

        // Crop the image
        let cropped_mat = Mat::roi(&img_mat, validated_rect)?;

        // Display the image
        highgui::imshow("Android_cam", &cropped_mat)?;

        let mut buf = opencv::core::Vector::<u8>::new();
        imgcodecs::imencode(".jpg", &cropped_mat, &mut buf, &opencv::core::Vector::<i32>::new())?;
        let cropped_img_data = buf.to_vec();

        // Wait for key press and check for 'f' (FEN extraction) or 'Esc' (quit)
        let key = highgui::wait_key(1)?;
        if key == 27 { // Esc key
            break;
        } else if key == 102 { // 'f' key
            println!("Processing image for FEN string extraction...");
            let static_str = String::from(" w - - 0 1");
            let start = Instant::now();
            let predicted_fen_string = get_fen_string_from(cropped_img_data, model.clone(), &device);
            let duration = start.elapsed().as_secs_f32();

            println!("====================== predicted board =======================");

            let mut predicted_board = Board::from_fen(predicted_fen_string.clone() + &static_str);
            predicted_board.print_board();

            println!("predicted fen string = {}", predicted_fen_string);

            println!("duaration = {:?}", duration);

            println!("\n\n");
        }

        // Small delay to prevent excessive requests
        sleep(Duration::from_millis(100));
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
