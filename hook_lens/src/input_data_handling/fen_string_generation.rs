use std::collections::HashMap;

use burn_cuda::{CudaDevice, Cuda};

use crate::input_data_handling::board_square_extracting::*;
use crate::data_and_model::inference::{self, ModelEnum};
pub fn get_fen_string_from(board_image_path : &str , model : ModelEnum<Cuda<f32, i32>>)->String{

    let mut map = HashMap::new();
    map.insert(0 as u8 , "b");
    map.insert(1 as u8 , "k");
    map.insert(2 as u8 , "n");
    map.insert(3 as u8 , "p");
    map.insert(4 as u8 , "q");
    map.insert(5 as u8 , "r");
    map.insert(6 as u8 , "e");
    map.insert(7 as u8 , "B");
    map.insert(8 as u8 , "K");
    map.insert(9 as u8 , "N");
    map.insert(10 as u8 , "P");
    map.insert(11 as u8 , "Q");
    map.insert(12 as u8 , "R");

    let mut pieces_images_and_position = extract_board_sqaures_from(board_image_path);

    let mut fen_string = String::from("");

    let mut empty_squares = 0 ;

    for i in 0..64{
        let image = &pieces_images_and_position[i];
        let piece = map[&inference::infer_model::<Cuda<f32, i32>>(&model, CudaDevice::default(), image.to_vec())];
        if piece == "e" {
        empty_squares += 1;
        }else{
            if empty_squares != 0 {
                fen_string.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            fen_string.push_str(piece);
        }

        if(i+1)%8 == 0{
            if empty_squares != 0 {
                fen_string.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            fen_string.push_str("/");
        }
    }    
    fen_string.pop(); // remove the last '/' from the fen string
    fen_string
    
}
