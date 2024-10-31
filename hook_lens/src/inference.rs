use burn::prelude::Backend;
use image::{DynamicImage, Rgb};
use imageproc::edges::canny;
use imageproc::hough::{self, LineDetectionOptions};

pub fn infer/*<B: Backend>*/(/*artifact_dir: &str,  device: B::Device, */board: DynamicImage) {
    
    // apply canny edge
    let luma_board = board.to_luma8();
    let low_threshold = 1.0;
    let high_threshold = 50.0;
    let canny_board = canny(&luma_board, low_threshold, high_threshold);
    canny_board.save("hook_lens\\canny_board.png");

    // apply hough line detection
    let options = LineDetectionOptions {
        vote_threshold: 120,
        suppression_radius: 8,
    };
    let polar_lines = hough::detect_lines(&canny_board, options);
    let hough_board = hough::draw_polar_lines(&board.to_rgb8(), &polar_lines, Rgb([255, 0, 0]));
    hough_board.save("hook_lens\\hough_board.png");
    
    // split image to 64 square images so it can be passed to CNN
    for i in 0..64 {


    }
    


}