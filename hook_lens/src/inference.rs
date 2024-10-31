use burn::prelude::Backend;
use image::DynamicImage;

pub fn infer/*<B: Backend>*/(/*artifact_dir: &str,  device: B::Device, */board: DynamicImage) {
    // apply canny edge

    board.save("hook_lens\\canny_board.png");

    // apply hough line detection

    board.save("hook_lens\\hough_board.png");
    
    // split image to 64 square images so it can be passed to CNN
    for i in 0..64 {


    }
    


}