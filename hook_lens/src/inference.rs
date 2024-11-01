use std::f32::consts::PI;
use std::f32::EPSILON;

use burn::prelude::Backend;
use image::{DynamicImage, Rgb};
use imageproc::edges::canny;
use imageproc::hough::{self, LineDetectionOptions, PolarLine};

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
    let mut polar_lines = hough::detect_lines(&canny_board, options);
    let mut polar_lines2  : Vec<PolarLine> = Vec::new();

    let mut horizontal_lines : Vec<PolarLine> = Vec::new();
    let mut vertical_lines  :  Vec<PolarLine> = Vec::new();
    for line in &polar_lines { 
        // in this part of code we  must separate the polar lines to horizontals and vertical 
        // to pass it to the function that create intersection points

        // the logic is not correct  so correct this part
        println!("r = {} , angle = {}" , line.r , line.angle_in_degrees);
        if line.angle_in_degrees >100 && line.angle_in_degrees < 130  { 
            horizontal_lines.push(*line);
        }
        else if line.angle_in_degrees < 77 {
            vertical_lines.push(*line);
        }
    }
    let hough_board = hough::draw_polar_lines(&board.to_rgb8(), &polar_lines, Rgb([255, 0, 0]));
    hough_board.save("hook_lens\\hough_board.png");
    
    let intersections = find_intersections(&horizontal_lines, &vertical_lines);
    println!("len = {}" , intersections.len());
    // // split image to 64 square images so it can be passed to CNN
    // for i in 0..64 {


    // }
    


}
struct Point {
    x: f32,
    y: f32,
}
// Calculate intersection between two lines in polar coordinates
fn calculate_intersection(line1: PolarLine, line2: PolarLine) -> Option<Point> {
    let theta1 = (line1.angle_in_degrees as f32) * PI / 180.0;
    let theta2 = (line2.angle_in_degrees as f32) * PI / 180.0;

    let sin1 = theta1.sin();
    let cos1 = theta1.cos();
    let sin2 = theta2.sin();
    let cos2 = theta2.cos();

    let determinant = cos1 * sin2 - sin1 * cos2;

    if determinant.abs() < EPSILON {
        // Lines are parallel
        return None;
    }

    let x = (line2.r * sin1 - line1.r * sin2) / determinant;
    let y = (line1.r * cos2 - line2.r * cos1) / determinant;
    Some(Point { x, y })
}
// Find intersections for horizontal and vertical lines
fn find_intersections(horizontal_lines: &Vec<PolarLine>, vertical_lines: &Vec<PolarLine>) -> Vec<Point> {
    let mut intersections = Vec::new();

    for &h_line in horizontal_lines {
        for &v_line in vertical_lines {
            if let Some(intersection) = calculate_intersection(h_line, v_line) {
                intersections.push(intersection);
            }
        }
    }
    intersections
}