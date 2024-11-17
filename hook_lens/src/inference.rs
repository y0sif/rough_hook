// use burn::prelude::Backend;
use opencv::{core::{Mat, Point, Point2f, Vec2f, Vector, CV_PI}, highgui::{imshow, wait_key_def}, imgcodecs::{self}, imgproc::{self, cvt_color, cvt_color_def, COLOR_BGR2GRAY, LINE_AA}};
use std::f64::consts::PI;

pub fn infer/*<B: Backend>*/(/*artifact_dir: &str,  device: B::Device, */) {
    let mut img = imgcodecs::imread("hook_lens\\input_img.png", imgcodecs::IMREAD_COLOR).unwrap();
    
    let mut img_intersections = img.clone();

    let mut gray_scale = Mat::default();

    cvt_color_def(&img, &mut gray_scale, COLOR_BGR2GRAY).unwrap();

    let mut canny_img: Mat = Default::default();
    imgproc::canny_def(&gray_scale, &mut canny_img,  46.0, 250.0).unwrap();

    imshow("canny", &canny_img).unwrap();
    wait_key_def().unwrap();

    let mut s_lines = Vector::<Vec2f>::new();
    imgproc::hough_lines_def(&canny_img, &mut s_lines, 1.0, PI / 280.0, 160).unwrap();
    // imgproc::hough_lines_p_def(&canny_img, &mut s_lines, 1.0, PI / 180.0, 150).unwrap();
    
    let mut vertical_lines_points = Vec::new();
    let mut horizontal_lines_points = Vec::new();
    
    println!("lines {}", s_lines.len());
    for s_line in s_lines {
		let [r, t] = *s_line;
		let cos_t = t.cos();
		let sin_t = t.sin();
		let x0 = r * cos_t;
		let y0 = r * sin_t;
		let alpha = 1000.;

		let pt1 = Point2f::new(x0 + alpha * -sin_t, y0 + alpha * cos_t).to::<i32>().unwrap();
		let pt2 = Point2f::new(x0 - alpha * -sin_t, y0 - alpha * cos_t).to::<i32>().unwrap();
        
        let t = t * 180. / PI as f32;
        if t > 75.0 && t < 105.0 {
            println!("horizontal = {:?}", s_line);
            horizontal_lines_points.push((pt1, pt2));
        }else if t < 15.0 || t > 165.0 {
            vertical_lines_points.push((pt1, pt2));
        }
        imgproc::line(&mut img, pt1, pt2, (255, 0, 0).into(), 1, LINE_AA, 0).unwrap();
	}
    
    for vert in &vertical_lines_points {
        for hor in &horizontal_lines_points {
            if (vert.1.x - vert.0.x) != 0 {
                let m1 = (vert.1.y - vert.0.y) / (vert.1.x - vert.0.x);
                let m2 = (hor.1.y - hor.0.y) / (hor.1.x - hor.0.x);
                
                let x1 = vert.0.x;
                let x2 = hor.0.x;
                let y1 = vert.0.y;
                let y2 = hor.0.y;

                let x_intersect = (m1 * x1 - m2 * x2 - y1 + y2) / (m1 - m2);
                let y_intersect = m2 * (x_intersect - x2) + y2;

                imgproc::circle_def(&mut img_intersections, Point::new(x_intersect, y_intersect), 3, (255, 0, 0).into()).unwrap();

            }else {
                let m2 = (hor.1.y - hor.0.y) / (hor.1.x - hor.0.x);
                
                let x1 = vert.0.x;
                let x2 = hor.0.x;
                let y2 = hor.0.y;

                let x_intersect = x1;
                let y_intersect = m2 * (x_intersect - x2) + y2;

                imgproc::circle_def(&mut img_intersections, Point::new(x_intersect, y_intersect), 3, (255, 0, 0).into()).unwrap();
                
            }
        }
    }
    
    imshow("hough", &img).unwrap();
    wait_key_def().unwrap();

    imshow("intersections", &img_intersections).unwrap();
    wait_key_def().unwrap();
}

