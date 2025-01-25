use image::imageops;
use opencv::{core::{Mat, Point, Point2f, Vec2f, Vector}, highgui::{imshow, wait_key_def}, imgcodecs::{self}, imgproc::{self, cvt_color_def, COLOR_BGR2GRAY, LINE_AA}};
use std::f64::consts::PI;



pub fn extract_board_sqaures_from(board_image_path : &str)->Vec<(Vec<u8> , u8)>{
    let mut img = imgcodecs::imread(board_image_path, imgcodecs::IMREAD_COLOR).unwrap();
    //convert image to gray sacle image
    let  gray_scale_image = convert_image_to_gray_scale(&img);
    //apply canny on the gray scale image
    let  canny_image = apply_canny(&gray_scale_image);

    // testing code that display affter we apply canny  (in dev env only)
    #[cfg(debug_assertions)]
    {
        imshow("canny", &canny_image).unwrap();
        wait_key_def().unwrap();
    }

     //using canny image we will apply hough line detection algorithm
    let mut s_lines = Vector::<Vec2f>::new();
    imgproc::hough_lines_def(&canny_image, &mut s_lines, 1.0, PI / 260.0, 170).unwrap();
    
    let intersection_points = get_intersection_points(&s_lines , &mut img);

    #[cfg(debug_assertions)]
    draw_intersection_points_on(&mut img , &intersection_points);
    // crop images from the original image and return them with their positions
    let pieces_images_and_position = crop_images_from(board_image_path , intersection_points);
    
    pieces_images_and_position
    

}

fn convert_image_to_gray_scale(colored_image : &Mat)->Mat{
    let mut gray_scale_image = Mat::default();
    cvt_color_def(&colored_image, &mut gray_scale_image, COLOR_BGR2GRAY).unwrap(); 
    gray_scale_image
}

fn apply_canny(gray_scale_image : &Mat)-> Mat{
    let mut canny_img: Mat = Default::default();
    imgproc::canny_def(&gray_scale_image, &mut canny_img,  46.0, 250.0).unwrap();
    canny_img
}

fn get_intersection_points(s_lines:&Vector<Vec2f> , img : &mut Mat)->Vec<(i32, i32)>{
    let mut vertical_lines_points = Vec::new();
    let mut horizontal_lines_points = Vec::new();
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
            //println!("horizontal = {:?}", s_line);
            horizontal_lines_points.push((pt1, pt2));
        }else if t < 15.0 || t > 165.0 {
            vertical_lines_points.push((pt1, pt2));
        }
        // draw the lines on the image (in dev env only)
        #[cfg(debug_assertions)]
        imgproc::line(img, pt1, pt2, (255, 0, 0).into(), 1, LINE_AA, 0).unwrap();
	}
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

                points.push((x_intersect , y_intersect));

            }else {
                let m2 = (hor.1.y - hor.0.y) / (hor.1.x - hor.0.x);
                
                let x1 = vert.0.x;
                let x2 = hor.0.x;
                let y2 = hor.0.y;

                let x_intersect = x1;
                let y_intersect = m2 * (x_intersect - x2) + y2;
                points.push((x_intersect , y_intersect));
            }
        }
    }
    // sort the points by x value to get the points in the correct order (first 8 points represent the first column and so on)
    points.sort_by(|a, b| a.0.cmp(&b.0));
    // we don't need the points on the last column so we delte them
    points.truncate(points.len() - 8); 

    points
    
}

fn crop_images_from(original_image_path: &str , intersection_points : Vec<(i32,i32)>)->Vec<(Vec<u8> , u8)>{
    let mut pieces_images_and_position  = Vec::new();
    let mut input_image = image::open(original_image_path).unwrap();

    // calculate edge lenth of the square
    let edge_lengh = intersection_points[8].0- intersection_points[0].0; 

    // gropu all ponits to 8 groups  (each column represent a group)
    let mut columns: Vec<Vec<(i32 ,i32)>> = intersection_points.chunks(8)
                                  .map(|chunk| chunk.to_vec())
                                  .collect();
    
    let mut image_number = 1;

    
    let initial_positions = vec![56,57,58,59,60,61,62,63];
    let mut index = 0;

    // for each column sort the points of each one by y value then crop images from the original image
    for column in &mut columns{
        let mut position = initial_positions[index] as i32;
        column.sort_by_key(|&(_, second)| second);

        for point in column{
            // crop image from the original image
            let cropped_image = input_image.crop(point.0 as u32, point.1 as u32, (edge_lengh+2) as u32, (edge_lengh+2) as u32);
            // resize the cropped image to 28*28  to be suitable for the model
            let resized_img = cropped_image.resize(32, 32, imageops::FilterType::Lanczos3);
            // convert the resized image to vec of u8
            let resized_img_as_vec = resized_img.to_rgb8().into_raw();
            // push the image and its position to the vector to use it to generate the fen string
            pieces_images_and_position.push((resized_img_as_vec , position as u8));

            // save the cropped image to the disk (in dev env only)
            #[cfg(debug_assertions)]
            {
                let standard_name = String::from("cropped");
                let folder_name = "//home//mostafayounis630//Graduation_Project//rough_hook/hook_lens//test_images//cropped_images//"; 
                let image_name = format!("{}{}{}", standard_name, image_number.to_string(),".png");
                let path = format!("{}{}" ,folder_name ,image_name);
                println!(" image number {} , has position of piece = {}" , image_number ,position);
                cropped_image.save(path).unwrap();
                image_number+=1;
            }
            position-=8;
        }
        index+=1;
    }
    pieces_images_and_position

}

#[cfg(debug_assertions)]
fn draw_intersection_points_on(image_to_draw_on:&mut Mat ,intersection_points : &Vec<(i32,i32)>){
    for point in intersection_points{
        imgproc::circle_def(image_to_draw_on, Point::new(point.0, point.1), 3, (255, 0, 0).into()).unwrap();
    }
    imshow("intersections", image_to_draw_on).unwrap();
    wait_key_def().unwrap();
}