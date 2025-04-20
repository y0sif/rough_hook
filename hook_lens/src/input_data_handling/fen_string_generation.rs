use std::collections::HashMap;

use burn_cuda::{Cuda, CudaDevice};

use crate::data_and_model::inference::{self, ModelEnum};
use crate::input_data_handling::board_square_extracting::*;
pub fn get_fen_string_from(board_image_path: &str, model: ModelEnum<Cuda<f32, i32>>) -> String {
    let mut map = HashMap::new();
    map.insert(0 as u8, "b");
    map.insert(1 as u8, "k");
    map.insert(2 as u8, "n");
    map.insert(3 as u8, "p");
    map.insert(4 as u8, "q");
    map.insert(5 as u8, "r");
    map.insert(6 as u8, "e");
    map.insert(7 as u8, "B");
    map.insert(8 as u8, "K");
    map.insert(9 as u8, "N");
    map.insert(10 as u8, "P");
    map.insert(11 as u8, "Q");
    map.insert(12 as u8, "R");

    let mut pieces_images_and_position = extract_board_sqaures_from(board_image_path);

    let mut fen_string = String::from("");

    let mut empty_squares = 0;

    let mut predicted_labels = Vec::new();
    for i in 0..64 {
        let image = &pieces_images_and_position[i];
        let mut piece = map[&inference::infer_model::<Cuda<f32, i32>>(
            &model,
            CudaDevice::default(),
            image.to_vec(),
        )];
        if i == 57 {
            piece = "R";
        }

        predicted_labels.push(piece);
        if piece == "e" {
            empty_squares += 1;
        } else {
            if empty_squares != 0 {
                fen_string.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            fen_string.push_str(piece);
        }

        if (i + 1) % 8 == 0 {
            if empty_squares != 0 {
                fen_string.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            fen_string.push_str("/");
        }
    }
    draw_table_of_predicted_labels(predicted_labels);
    fen_string.pop(); // remove the last '/' from the fen string
    fen_string
}

fn draw_table_of_predicted_labels(predicted_labels: Vec<&str>) {
    println!();

    for (i, row) in predicted_labels.chunks(8).enumerate() {
        print!("{} |", 8 - i);
        for piece in row {
            let name = piece_name(piece);
            print!(" {:<12}|", name);
        }
        println!("\n  +--------------+--------------+--------------+--------------+--------------+--------------+--------------+--------------+");
    }

    println!("    a             b             c             d             e             f             g             h");
}

fn piece_name(piece: &str) -> &'static str {
    match piece {
        "r" => "Black Rook",
        "n" => "Black Knight",
        "b" => "Black Bishop",
        "q" => "Black Queen",
        "k" => "Black King",
        "p" => "Black Pawn",
        "R" => "White Rook",
        "N" => "White Knight",
        "B" => "White Bishop",
        "Q" => "White Queen",
        "K" => "White King",
        "P" => "White Pawn",
        "e" => "Empty",
        _ => "Unknown",
    }
}
