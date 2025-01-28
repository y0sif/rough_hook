#[cfg(test)]
mod tests{
    use rusty_brain::board::Board;
    use crate::input_data_handling::fen_string_generation::get_fen_string_from;
    use prettytable::{Table, row};

    #[test]
    fn test_fen_string_of_image_1(){
        let board_image_path = "/home/mostafayounis630/Graduation_Project/test_images/input_img.png";
         // Name - Path - Id
         let models: Vec<(&str, &str , i8)> = vec![
            ("New CNN", "/home/mostafayounis630/Graduation_Project/hook_lens_models/final_cnn_hook_lens" , 1),
            ("Old CNN", "/home/mostafayounis630/Graduation_Project/hook_lens_models/hook_lens_old_aug_resnet" , 1),
            ("KAN", "/home/mostafayounis630/Graduation_Project/hook_lens_models/kan_hook_lens" , 2),
         ];
        // name , correct_pieces  , wrong_pieces , accuracy
        let mut models_results : Vec<(&str , i16 , i16 , f32)> = Vec::new();
        let static_str = String::from(" w - - 0 1");
        let actual_fen_string = String::from("r3q1k1/pppb1ppp/2n5/3P4/8/2B2N2/PP3PPP/R2Q2K1");
        
        let mut flag = false;
        for (model_name , model_path , id) in models{
            if model_name.len() == 0 || model_path.len() == 0 {
                continue;
            }

            println!("\n\n-#-#-#-#-#-#-#-#-#-#-#-#-#-#-#-->  Testing : {}  model  <--#-#-#-#-#-#-#-#-#-#-#-#-#-#-#\n\n" , {model_name});

            let  predicted_fen_string = get_fen_string_from(board_image_path , model_path , id);

            println!("====================== actual board =======================");

            let mut actual_board = Board::from_fen(actual_fen_string.clone() + &static_str);
            actual_board.print_board();

            println!("====================== predicted board =======================");

            let mut predicted_board = Board::from_fen(predicted_fen_string.clone() + &static_str);
            predicted_board.print_board();

            if predicted_fen_string == actual_fen_string {
                flag = true;
            }

            println!("\nactual_fen_string = {}" , actual_fen_string);
            println!("predicted fen string = {}", predicted_fen_string);

            let wrong = count_fen_differences(&actual_fen_string , &predicted_fen_string).unwrap() as i16;
            let correct = 64 - wrong as i16;
            let accuracy = (correct as f32 / 64.0)*100 as f32;
            let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();

            models_results.push((model_name , correct , wrong , accuracy));
            
            println!("\n\n");
        }
        print_results_table(models_results);
        assert_eq!(flag, true);
    }

    fn count_fen_differences(fen1: &str, fen2: &str) -> Result<usize, &'static str> {
    // Split the FEN strings into their components
    let board1 = fen1.split(' ').next().ok_or("Invalid FEN string")?;
    let board2 = fen2.split(' ').next().ok_or("Invalid FEN string")?;

    // Ensure both boards have the same length
    if board1.len() != board2.len() {
        return Err("The FEN strings represent boards of different sizes.");
    }

    // Count the number of differences
    let differences = board1
        .chars()
        .zip(board2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    Ok(differences)
}

fn print_results_table(models_results: Vec<(&str, i16, i16, f32)>) {
    // Create a new table
    let mut table = Table::new();

    // Add a header row
    table.add_row(row!["Model Name", "Correct", "Wrong", "Accuracy"]);

    // Iterate over the vector and add each tuple as a row in the table
    for (model_name, correct, wrong, accuracy) in models_results {
        table.add_row(row![model_name, correct, wrong, accuracy]);
    }
    // Print the table to the console
    table.printstd();
}

}