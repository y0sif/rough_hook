use burn::prelude::Backend;
use image::DynamicImage;
use imageproc::edges::canny;

pub fn infer/*<B: Backend>*/(/*artifact_dir: &str,  device: B::Device, */board: DynamicImage) {
    
    // apply canny edge
    let luma_board = board.to_luma8();
    let low_threshold = 1.0;
    let high_threshold = 50.0;
    let canny_board = canny(&luma_board, low_threshold, high_threshold);
    canny_board.save("hook_lens\\canny_board.png");

    // apply hough line detection

    board.save("hook_lens\\hough_board.png");
    
    // split image to 64 square images so it can be passed to CNN
    for i in 0..64 {


    }
    


}