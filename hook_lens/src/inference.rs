use std::f32::consts::PI;
// use burn::prelude::Backend;
use image::{DynamicImage, GenericImageView, Rgb};
use imageproc::edges::canny;
use imageproc::hough::{self, LineDetectionOptions};

pub fn infer/*<B: Backend>*/(/*artifact_dir: &str,  device: B::Device, */board: DynamicImage) {
    
    // apply canny edge
    let luma_board = board.to_luma8();
    let low_threshold = 44.0;
    let high_threshold = 55.0;
    let canny_board = canny(&luma_board, low_threshold, high_threshold);
    canny_board.save("hook_lens\\canny_board.png").unwrap();

    // apply hough line detection
    let options = LineDetectionOptions {
        vote_threshold: 120,
        suppression_radius: 8,
    };

    let polar_lines = hough::detect_lines(&canny_board, options);
    let mut vert_lines = Vec::new();
    let mut hor_lines = Vec::new();

    for line in &polar_lines {
        if (line.angle_in_degrees < 15) || (line.angle_in_degrees > 165) {
            println!("vertical lines");
            println!("angle = {}", line.angle_in_degrees);
            println!("r = {}", line.r);
            vert_lines.push(line.clone());
        }else if (line.angle_in_degrees < 105) && (line.angle_in_degrees > 75) {
            println!("horizontal lines");
            println!("angle = {}", line.angle_in_degrees);
            println!("r = {}", line.r);
            hor_lines.push(line.clone());
        }
    }
    
    println!("horizontal distance");
    let mut h = Vec::new();
    for i in 0..hor_lines.len() -1 {
        let r1 = hor_lines[i].r;
        let r2 = hor_lines[i+1].r;
        let theta1 = hor_lines[i].angle_in_degrees as f32 * PI/180.0;
        let theta2 = hor_lines[i].angle_in_degrees as f32 * PI/180.0;

        let distance = f32::sqrt(r1.powi(2) + r2.powi(2) - 2.0 * r1 * r2 * f32::cos(theta1 - theta2)); 
        if distance > 50.0 && distance < 60.0 {
            h.push(i+1);
        }
        println!("distance = {}", distance);
    }
    
    for i in h {
        hor_lines.remove(i);
    }
    
    println!("vertical distance");
    let mut v = Vec::new();
    for i in 0..vert_lines.len() -1 {
        let r1 = vert_lines[i].r;
        let r2 = vert_lines[i+1].r;
        let theta1 = vert_lines[i].angle_in_degrees as f32 * PI/180.0;
        let theta2 = vert_lines[i].angle_in_degrees as f32 * PI/180.0;

        let distance = f32::sqrt(r1.powi(2) + r2.powi(2) - 2.0 * r1 * r2 * f32::cos(theta1 - theta2)); 
        if distance > 50.0 && distance < 60.0 {
            v.push(i+1);
        }
        println!("distance = {}", distance);
    }

    for i in v {
        vert_lines.remove(i);
    }
    
    // calc intersections 
    let mut counter = 1;
    for vert in &vert_lines {
        for hor in &hor_lines {
            let r1 = vert.r;
            let theta1 = vert.angle_in_degrees as f32 * PI/180.0;
            let r2 = hor.r;
            let theta2 = hor.angle_in_degrees as f32 * PI/180.0;

            let x1 = r1 * f32::cos(theta1);
            let y1 = r1 * f32::sin(theta1);

            let x2 = r2 * f32::cos(theta2);
            let y2 = r2 * f32::sin(theta2);
            
            let c1 = r1 * (f32::cos(theta1) - f32::sin(theta1));
            let c2 = r2 * (f32::cos(theta2) - f32::sin(theta2));
            
            let slope1 = y1 / x1;
            let slope2 = y2 / x2;
            
            let x_intercept = (c2 - c1) / (slope1 - slope2);
            let y_intercept = x_intercept * slope1 + c1;
            
            let sub = board.view(x_intercept as u32, y_intercept as u32, 38, 38);

            sub.to_image().save("hook_lens\\squares\\sub_image".to_owned() + &counter.to_string() + ".png").unwrap();
            counter += 1;
        }
    }

    let hor_hough = hough::draw_polar_lines(&board.to_rgb8(), &hor_lines, Rgb([255, 0, 0]));
    hor_hough.save("hook_lens\\hor_hough.png").unwrap();

    let vert_hough = hough::draw_polar_lines(&board.to_rgb8(), &vert_lines, Rgb([255, 0, 0]));
    vert_hough.save("hook_lens\\vert_hough.png").unwrap();

    let hough_board = hough::draw_polar_lines(&board.to_rgb8(), &polar_lines, Rgb([255, 0, 0]));
    hough_board.save("hook_lens\\hough_board.png").unwrap();
    
}
