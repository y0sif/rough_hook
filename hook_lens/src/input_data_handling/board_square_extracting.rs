use image::imageops::FilterType;
use image::{imageops, ColorType, DynamicImage, GenericImageView, ImageFormat};
use opencv::core::{MatTraitConst, MatTraitConstManual, Point_, Rect, Size, VectorToVec};
use opencv::highgui;
use opencv::imgcodecs::{imread, imwrite, IMREAD_COLOR};
use opencv::types::VectorOfu8;
use opencv::{
    core::{Mat, Point, Point2f, Vec2f, Vector},
    highgui::{imshow, wait_key_def},
    imgcodecs::{self},
    imgproc::{self, cvt_color_def, COLOR_BGR2GRAY, LINE_AA},
};
use std::f64::consts::PI;
use std::io::BufReader;

use std::fs::File;
use std::path::Path;

pub fn extract_board_sqaures_from(img_data: &Vec<u8>) -> Vec<Vec<u8>> {
    let prepared_img = prepare_image_on_template_image(img_data);
    let (img, path_of_image_to_draw_on) = remove_borders(prepared_img, 0, 0, 0, 15);
    /////
    let mut img = img.unwrap();
    //let mut img = imgcodecs::imread(board_image_path, imgcodecs::IMREAD_COLOR).unwrap();
    //convert image to gray sacle image
    let gray_scale_image = convert_image_to_gray_scale(&img);
    //apply canny on the gray scale image
    let canny_image = apply_canny(&gray_scale_image);

    // testing code that display affter we apply canny  (in dev env only)
    #[cfg(debug_assertions)]
    {
        imshow("canny", &canny_image).unwrap();
        wait_key_def().unwrap();
    }

    //using canny image we will apply hough line detection algorithm
    let mut s_lines = Vector::<Vec2f>::new();
    imgproc::hough_lines_def(&canny_image, &mut s_lines, 1.0, PI / 131.0, 123).unwrap();

    let mut intersection_points = get_intersection_points(&s_lines, &mut img);

    intersection_points.sort_by(|a, b| a.1.cmp(&b.1));
    // prepare intersection points
    for i in 1..65 {
        if i <= 8 {
            intersection_points[i - 1].1 -= 60;
        } else {
            intersection_points[i - 1].1 -= 30;
        }
    }
    //intersection_points.truncate(16);
    #[cfg(debug_assertions)]
    draw_intersection_points_on(&mut img, &intersection_points);

    let mut buf = opencv::core::Vector::<u8>::new();
    imgcodecs::imencode(".jpg", &img, &mut buf, &opencv::core::Vector::<i32>::new()).unwrap();
    let intersection_img_data = buf.to_vec();
    // crop images from the original image and return them with their positions
    let pieces_images_and_position = crop_images_from(&intersection_img_data, intersection_points);
    
    pieces_images_and_position
}

fn convert_image_to_gray_scale(colored_image: &Mat) -> Mat {
    let mut gray_scale_image = Mat::default();
    cvt_color_def(&colored_image, &mut gray_scale_image, COLOR_BGR2GRAY).unwrap();
    gray_scale_image
}

fn apply_canny(gray_scale_image: &Mat) -> Mat {
    let mut canny_img: Mat = Default::default();
    imgproc::canny_def(&gray_scale_image, &mut canny_img, 180.0, 340.0).unwrap();
    canny_img
}

fn get_intersection_points(s_lines: &Vector<Vec2f>, img: &mut Mat) -> Vec<(i32, i32)> {
    let mut vertical_lines_points = Vec::new();
    let mut horizontal_lines_points = Vec::new();
    for s_line in s_lines {
        let [r, t] = *s_line;
        let cos_t = t.cos();
        let sin_t = t.sin();
        let x0 = r * cos_t;
        let y0 = r * sin_t;
        let alpha = 1000.;

        let pt1 = Point2f::new(x0 + alpha * -sin_t, y0 + alpha * cos_t)
            .to::<i32>()
            .unwrap();
        let pt2: Point_<i32> = Point2f::new(x0 - alpha * -sin_t, y0 - alpha * cos_t)
            .to::<i32>()
            .unwrap();

        let t = t * 180. / PI as f32;
        if t > 75.0 && t < 105.0 {
            //println!("horizontal = {:?}", s_line);
            horizontal_lines_points.push((pt1, pt2));
        } else if t < 15.0 || t > 165.0 {
            vertical_lines_points.push((pt1, pt2));
        }
    }

    // sort the lines to get the first 8 lines from top to down
    horizontal_lines_points.sort_by(|a, b| a.0.y.cmp(&b.0.y));
    horizontal_lines_points.truncate(8);
    // sort the lines to get the first 8 lines from left to right
    vertical_lines_points.sort_by(|a, b| a.0.x.cmp(&b.0.x));
    vertical_lines_points.truncate(8);

    // draw the lines on the board in the debug env
    #[cfg(debug_assertions)]
    {
        draw_horizontals_lines(img, &horizontal_lines_points);
        draw_vertical_lines(img, &vertical_lines_points);
    }

    println!("h = {}", horizontal_lines_points.len());
    println!("V = {}", vertical_lines_points.len());
    let mut points: Vec<(i32, i32)> = Vec::new();
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

                points.push((x_intersect, y_intersect));
            } else {
                let m2 = (hor.1.y - hor.0.y) / (hor.1.x - hor.0.x);

                let x1 = vert.0.x;
                let x2 = hor.0.x;
                let y2 = hor.0.y;

                let x_intersect = x1;
                let y_intersect = m2 * (x_intersect - x2) + y2;
                points.push((x_intersect, y_intersect));
            }
        }
    }
    // sort the points by x value to get the points in the correct order (first 8 points represent the first column and so on)
    points.sort_by(|a, b| a.0.cmp(&b.0));
    // we don't need the points on the last column so we delte them
    points
}

fn draw_horizontals_lines(
    img: &mut Mat,
    horizontal_lines_points: &Vec<(Point_<i32>, Point_<i32>)>,
) {
    for (pt1, pt2) in horizontal_lines_points {
        imgproc::line(img, *pt1, *pt2, (255, 0, 0).into(), 1, LINE_AA, 0).unwrap();
    }
}

fn draw_vertical_lines(img: &mut Mat, vertical_lines_points: &Vec<(Point_<i32>, Point_<i32>)>) {
    for (pt1, pt2) in vertical_lines_points {
        imgproc::line(img, *pt1, *pt2, (255, 0, 0).into(), 1, LINE_AA, 0).unwrap();
    }
}

fn crop_images_from(
    img_data: &Vec<u8>,
    intersection_points: Vec<(i32, i32)>,
) -> Vec<Vec<u8>> {
    println!("I'm here ya man !!");
    let mut pieces_images_and_position = Vec::new();
    let mut input_image = image::load_from_memory(img_data.as_slice())
        .expect("Cannot load image");

    // calculate edge lenth of the square
    let edge_lengh = intersection_points[1].0 - intersection_points[0].0;

    let mut image_number = 1;

    for point in &intersection_points {
        let cropped_image = input_image.crop(
            point.0 as u32,
            point.1 as u32,
            (edge_lengh + 2) as u32,
            (edge_lengh + 2) as u32,
        );

        let resized_img = cropped_image.resize(32, 32, imageops::FilterType::Lanczos3);
        // convert the resized image to vec of u8
        let resized_img_as_vec = resized_img.to_rgb8().into_raw();
        // push the image and its position to the vector to use it to generate the fen string
        pieces_images_and_position.push(resized_img_as_vec);

        #[cfg(debug_assertions)]
        {
            let standard_name = String::from("cropped");
            let folder_name = "/home/y0sif/cropped";
            let image_name = format!("{}{}{}", standard_name, image_number.to_string(), ".png");
            let path = format!("{}{}", folder_name, image_name);
            cropped_image.save(path);
            image_number += 1;
        }
    }
    pieces_images_and_position
}

#[cfg(debug_assertions)]
fn draw_intersection_points_on(image_to_draw_on: &mut Mat, intersection_points: &Vec<(i32, i32)>) {
    for point in intersection_points {
        imgproc::circle_def(
            image_to_draw_on,
            Point::new(point.0, point.1),
            3,
            (0, 0, 255).into(),
        )
        .unwrap();
    }

    imshow("intersections", image_to_draw_on).unwrap();
    wait_key_def().unwrap();
}

fn prepare_image_on_template_image(img_data: &Vec<u8>) -> Result<Mat, opencv::Error> {
    let first_image_path = "/home/y0sif/college/gp/rough_hook/hook_lens/data_in_sql_lite/board.png";
    let output_path = "/home/y0sif/college/gp/rough_hook/hook_lens/data_in_sql_lite/aboyounis.png";
    // Load first image to get dimensions and color type
    let img1_reader =
        BufReader::new(File::open(first_image_path).expect("Cannot open first image"));
    let img1_format = image::guess_format(&std::fs::read(first_image_path).unwrap())
        .expect("Cannot detect format");
    let img1 = image::load(img1_reader, img1_format).expect("Cannot load first image");

    let (width, height) = img1.dimensions();
    let target_color = img1.color();

    let img2 = image::load_from_memory(img_data.as_slice())
        .expect("Cannot load second image");

    // Resize and convert color if needed
    let resized = img2.resize_exact(width, height, FilterType::Lanczos3);
    let converted = match target_color {
        ColorType::Rgb8 => resized.to_rgb8(),
        _ => panic!("Unsupported color type: {:?}", target_color),
    };

    converted
        .save_with_format(output_path, img1_format)
        .expect("Failed to save the converted image");

    // Convert image::RgbaImage to OpenCV Mat
    let raw_data = converted.into_raw();
    let binding = Mat::from_slice(&raw_data)?;
    let mat = binding.reshape(3, height as i32)?; // 4 channels for RGBA8
    let mut converted_mat = Mat::default();
    imgproc::cvt_color(&mat, &mut converted_mat, imgproc::COLOR_RGB2BGR, 0)?;

    Ok(converted_mat)
}

fn remove_borders(
    img: Result<Mat, opencv::Error>,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
) -> (Result<Mat, opencv::Error>, String) {
    // Read image
    let temp_img = img.unwrap().clone();
    let size = temp_img.size();
    let (width, height) = match size {
        Ok(size) => (size.width, size.height),
        Err(err) => panic!("Error getting image size: {}", err),
    };

    // Validate border sizes
    if top + bottom >= height || left + right >= width {
        panic!("Border sizes are too large for the image dimensions.");
    }

    // Define cropping rectangle
    let roi = Rect::new(left, top, width - left - right, height - top - bottom);

    //let temp_img = img.unwrap().clone();
    let cropped_img = Mat::roi(&temp_img, roi);
    let cropped_img2 = Mat::roi(&temp_img, roi);
    let cropped_img3 = Mat::roi(&temp_img, roi);

    // // Show the cropped image
    #[cfg(debug_assertions)]
    {
        highgui::imshow("Cropped Chessboard", &cropped_img.unwrap());
        highgui::wait_key(0);
    }

    // Save the cropped image
    let output_path = "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/images_for_real_life_test/cropped_chessboard_1.png";
    imwrite(
        output_path,
        &cropped_img2.unwrap(),
        &opencv::core::Vector::<i32>::new(),
    )
    .unwrap();
    println!("Saved cropped image to cropped_chessboard_1.png");
    (
        Ok(cropped_img3.unwrap().clone_pointee()),
        output_path.to_string(),
    )
}
