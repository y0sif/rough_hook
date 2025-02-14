use opencv::{prelude::*, imgcodecs, highgui, imgproc, core};
use reqwest;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    // put url from ip webcam app here    
    let url = "http://192.168.1.15:8080/shot.jpg";

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

        // Resize the image
        let mut resized = Mat::default();
        imgproc::resize(
            &img_mat,
            &mut resized,
            opencv::core::Size::new(1800, 1800),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )?;

        // Display the image
        highgui::imshow("Android_cam", &resized)?;

        // Wait for key press and check for 'f' (FEN extraction) or 'Esc' (quit)
        let key = highgui::wait_key(1)?;
        if key == 27 { // Esc key
            break;
        } else if key == 102 { // 'f' key
            println!("Processing image for FEN string extraction...");
            // Example: let fen_string = extract_fen_string(&resized);
            // println!("FEN String: {}", fen_string);
        }

        // Small delay to prevent excessive requests
        sleep(Duration::from_millis(100));
    }

    highgui::destroy_all_windows()?;
    Ok(())
}