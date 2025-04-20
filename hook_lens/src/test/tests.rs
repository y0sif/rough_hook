#[cfg(test)]
mod tests {
    use crate::data_and_model::inference::{self, load_model_paramter, ModelEnum};
    use crate::input_data_handling::fen_string_generation::get_fen_string_from;
    use image::{imageops, GenericImageView};
    use imageops::FilterType;
    use opencv::highgui::{wait_key_def, WINDOW_NORMAL};
    use opencv::{highgui, imgcodecs};
    use rusty_brain::board::Board;
    //use crate::input_data_handling::fen_string_generation::get_fen_string_from;
    use crate::test::test_models_repository::Repository;
    use burn_cuda::{Cuda, CudaDevice};
    use prettytable::{row, Table};
    use std::collections::HashMap;
    use std::fs;
    use std::os::unix::thread;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn test_all() {
        // comment one of them to test the another one

        //rela life test (test the models in real board)
        let flag = test_fen_string_of_image_1();
        assert_eq!(flag, true);

        // generalization test
        // test_models_on_un_seen_data();
        // assert_eq!(1, 0);
    }

    fn test_fen_string_of_image_1() -> bool {
        #[cfg(debug_assertions)]
        {
            let img_path = "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/images_for_real_life_test/real1.png";
            let img = imgcodecs::imread(img_path, imgcodecs::IMREAD_COLOR).unwrap();
            // Display the image
            //set window size
            highgui::named_window("Android_cam", WINDOW_NORMAL).unwrap();
            highgui::resize_window("Android_cam", 800, 600).unwrap();
            highgui::imshow("Android_cam", &img);
            wait_key_def().unwrap();
        }

        let board_image_path = "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/images_for_real_life_test/real2.png";
        // Name - Path - Id
        let mut repository = Repository::new();

        //repository.load_all_models();
        repository.load_models_by_ids(vec![1]); // uncomment it to provide the models you want to test

        // name , correct_pieces  , wrong_pieces , accuracy
        let mut models_results: Vec<(&str, i16, i16, f32, f32)> = Vec::new();
        let static_str = String::from(" w - - 0 1");
        let actual_fen_string = String::from("r3q1k1/pppb1ppp/2n5/3P4/8/2B2N2/PP3PPP/R2Q2K1");

        let mut flag = false;

        for (model_name, model_path, id) in repository.test_models {
            let model: ModelEnum<Cuda<f32, i32>> =
                load_model_paramter::<Cuda<f32, i32>>(id, &model_path, CudaDevice::default());

            println!("\n\n-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-->  Testing : {}  model  <--#-#-#-#-#-#-#-#-#-#-#-#-#-#-#\n\n" , {model_name});
            let start = Instant::now();
            let mut predicted_fen_string = get_fen_string_from(board_image_path, model);

            let duration = start.elapsed().as_secs_f32();
            //println!("====================== actual board =======================");

            let mut actual_board = Board::from_fen(actual_fen_string.clone() + &static_str);
            //actual_board.print_board();

            println!("====================== predicted board =======================");

            let mut predicted_board = Board::from_fen(predicted_fen_string.clone() + &static_str);

            predicted_board.print_board();

            if predicted_fen_string == actual_fen_string {
                flag = true;
            }

            // println!("\nactual_fen_string = {}", actual_fen_string);
            println!("predicted fen string = {}", predicted_fen_string);

            let wrong =
                count_fen_differences(&actual_fen_string, &predicted_fen_string).unwrap() as i16;
            let correct = 64 - wrong as i16;
            let accuracy = (correct as f32 / 64.0) * 100 as f32;
            let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();

            // println!("Correct = {}", correct);
            // println!("wrong = {}", wrong);
            // println!("accuracy = {}", accuracy);
            // println!("duaration = {:?}", duration);

            models_results.push((model_name, correct, wrong, accuracy, duration));

            println!("\n\n");
        }
        //print_results_table(models_results);
        //sleep(2000);

        return flag;
    }

    fn count_fen_differences(fen1: &str, fen2: &str) -> Result<usize, &'static str> {
        // Split the FEN strings into their components
        let board1 = fen1.split(' ').next().ok_or("Invalid FEN string")?;
        let board2 = fen2.split(' ').next().ok_or("Invalid FEN string")?;

        // Count the number of differences
        let differences = board1
            .chars()
            .zip(board2.chars())
            .filter(|(c1, c2)| c1 != c2)
            .count();

        Ok(differences)
    }

    fn test_models_on_un_seen_data() {
        // to store the results of the models on it then use it to create a table that contain info all tested models
        let mut models_results: Vec<(&str, i16, i16, f32, f32)> = Vec::new();
        // get the models to be tested from the repository of test models
        let mut repository = Repository::new();

        //repository.load_all_models();
        //repository.load_models_by_ids(vec![1,13]);  // uncomment it to provide the models you want to test
        repository.load_models_by_ids(vec![27]); // uncomment it to provide the models you want to test

        for (model_name, model_path, id) in repository.test_models {
            println!("model name : {}", model_name);
            let model: ModelEnum<Cuda<f32, i32>> =
                load_model_paramter::<Cuda<f32, i32>>(id, &model_path, CudaDevice::default());

            if model_name.len() == 0 || model_path.len() == 0 {
                continue;
            }
            let start = Instant::now();
            let (total_num_of_images, correct_predictions) = test_model(model);
            let duration = start.elapsed().as_secs_f32();
            let accuracy = (correct_predictions as f32 / total_num_of_images as f32) * 100 as f32;
            let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();
            models_results.push((
                model_name,
                total_num_of_images,
                correct_predictions,
                accuracy,
                duration,
            ));

            println!("\n======================================================================================\n");
        }
        print_results_table(models_results);
    }

    fn test_model(model: ModelEnum<Cuda<f32, i32>>) -> (i16, i16) {
        let test_dir_path = "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/hook_lens_data/big_test";

        let mut map = HashMap::new();
        map.insert(0 as u8, "bb");
        map.insert(1 as u8, "bk");
        map.insert(2 as u8, "bn");
        map.insert(3 as u8, "bp");
        map.insert(4 as u8, "bq");
        map.insert(5 as u8, "br");
        map.insert(6 as u8, "empty");
        map.insert(7 as u8, "wb");
        map.insert(8 as u8, "wk");
        map.insert(9 as u8, "wn");
        map.insert(10 as u8, "wp");
        map.insert(11 as u8, "wq");
        map.insert(12 as u8, "wr");

        let mut total_images = 0;
        let mut correct_predictions = 0;
        // Read subdirectories (labels)
        if let Ok(entries) = fs::read_dir(test_dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let mut total_num_of_that_piece = 0;
                    let mut piece_correct_predictions = 0;

                    let label = path.file_name().unwrap().to_string_lossy().to_string();

                    // Read images inside the subdirectory
                    if let Ok(images) = fs::read_dir(&path) {
                        for img_entry in images.flatten() {
                            let img_path = img_entry.path();
                            if img_path
                                .extension()
                                .map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "jpeg")
                            {
                                let img = image::open(img_path).unwrap();
                                let img = img.resize_exact(32, 32, FilterType::Nearest);
                                let rgb_image = img.to_rgb8();
                                let image = rgb_image.into_raw(); // Convert to Vec<u8>

                                let prediction = map[&inference::infer_model::<Cuda<f32, i32>>(
                                    &model,
                                    CudaDevice::default(),
                                    image.to_vec(),
                                )];

                                total_num_of_that_piece += 1;
                                if prediction == label {
                                    piece_correct_predictions += 1;
                                }

                                total_images += 1;
                                if prediction == label {
                                    correct_predictions += 1;
                                }
                            }
                        }

                        let accuracy = (piece_correct_predictions as f32
                            / total_num_of_that_piece as f32)
                            * 100 as f32;
                        let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();
                        println!(
                            "piece = {}  , total = {} ,  correct = {} , with accuracy = {}",
                            label, total_num_of_that_piece, piece_correct_predictions, accuracy
                        );
                    }
                }
            }
        }
        (total_images, correct_predictions)
    }

    fn print_results_table(models_results: Vec<(&str, i16, i16, f32, f32)>) {
        // Create a new table
        let mut table = Table::new();

        // Add a header row
        table.add_row(row![
            "Model Name",
            "Total",
            "Correct",
            "Accuracy",
            "Time(s)"
        ]);

        // Iterate over the vector and add each tuple as a row in the table
        for (model_name, correct, wrong, accuracy, time) in models_results {
            table.add_row(row![model_name, correct, wrong, accuracy, time]);
        }
        // Print the table to the console
        table.printstd();
    }
}
