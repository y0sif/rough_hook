use burn::data::dataloader::batcher::Batcher;
use burn::prelude::*;
use burn_dataset::transform::PartialDataset;
use burn_dataset::transform::ShuffledDataset;
use burn_dataset::Dataset;
use burn_dataset::SqliteDataset;
use nn::Sigmoid;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]

//Change Struct Name, and feature name is same as db (Check DType) 
pub struct ChessGameItem{
    pub white_response_time: Vec<i32>,
    pub white_remaining_time: Vec<i32>,
    pub white_win_chance: Vec<i32>,
    pub white_move_accuracy: Vec<i32>,
    pub white_board_material: Vec<i32>,
    pub white_legal_moves: Vec<i32>,

    pub black_response_time: Vec<i32>,
    pub black_remaining_time: Vec<i32>,
    pub black_win_chance: Vec<i32>,
    pub black_move_accuracy: Vec<i32>,
    pub black_board_material: Vec<i32>,
    pub black_legal_moves: Vec<i32>,

    pub label: i32
} 


type MappedDataset = Box<dyn Dataset<ChessGameItem>>;
pub struct ChessGameDataSet{
    dataset: MappedDataset,
}

impl Dataset<ChessGameItem> for ChessGameDataSet {
    fn get(&self, index: usize) -> Option<ChessGameItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}



impl ChessGameDataSet {
    pub fn train() -> Self{
        Self::new("train")
    }

    pub fn test() -> Self{
        Self::new("test")
    }
    //Change Path
    fn new(split: &str) -> Self{
        let db_file = Path::new("rough_guard_db/pgn_features.db");
        
        let dataset = SqliteDataset::from_db_file(db_file, split).unwrap();
        
        let dataset= ShuffledDataset::<SqliteDataset<ChessGameItem>, ChessGameItem>::with_seed(dataset, 42);
        type PartialData = PartialDataset<ShuffledDataset::<SqliteDataset<ChessGameItem>, ChessGameItem>, ChessGameItem>;
        //Split to Percentages
        let data_split = match split {
            "train" => PartialData::new(dataset, 0, dataset.len() * 0.8), // Get first 80% dataset
            "test" => PartialData::new(dataset, dataset.len() * 0.8, dataset.len()), // Take remaining 20%
            _ => panic!("Invalid split type"),                     // Handle unexpected split types
        };

        let dataset = Box::new(data_split);

        ChessGameDataSet {
            dataset
        }
    }
}

#[derive(Clone)]
pub struct ChessGameBatcher<B: Backend>{
    device: B::Device,
}

impl<B: Backend> ChessGameBatcher<B> {
    pub fn new(device: B::Device) -> Self{
        Self{ device }
    }
}

//Same as Item Struct + DType Tensors
#[derive(Debug, Clone)]
pub struct ChessGameBatch<B: Backend> {
    pub white_response_time: Tensor<B, 2>,
    pub white_remaining_time: Tensor<B, 2>,
    pub white_win_chance: Tensor<B, 2>,
    pub white_move_accuracy: Tensor<B, 2>,
    pub white_board_material: Tensor<B, 2>,
    pub white_legal_moves: Tensor<B, 2>,

    pub black_response_time: Tensor<B, 2>,
    pub black_remaining_time: Tensor<B, 2>,
    pub black_win_chance: Tensor<B, 2>,
    pub black_move_accuracy: Tensor<B, 2>,
    pub black_board_material: Tensor<B, 2>,
    pub black_legal_moves: Tensor<B, 2>,

    pub label: Tensor<B, 1>,
}

// Create vector of tensors for all features, for loop once to insert
// labels instead of eval
// Cat All and Batch
impl <B: Backend> Batcher<ChessGameItem, ChessGameBatch<B>> for ChessGameBatcher<B>{
    fn batch(&self, items: Vec<ChessGameItem>) -> ChessGameBatch<B> {
        let mut white_response_time: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_remaining_time: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_win_chance: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_move_accuracy: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_board_material: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_legal_moves: Vec<Tensor<B, 2>> = Vec::new();
        let mut white_response_time: Vec<Tensor<B, 2>> = Vec::new();

        let mut black_response_time: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_remaining_time: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_win_chance: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_move_accuracy: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_board_material: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_legal_moves: Vec<Tensor<B, 2>> = Vec::new();
        let mut black_response_time: Vec<Tensor<B, 2>> = Vec::new();


        for item in items.iter() {
            let white_response_tensor = Tensor::<B, 1>::from_data(item.white_response_time.as_slice(), &self.device);
            let white_remaining_tensor = Tensor::<B, 1>::from_data(item.white_remaining_time.as_slice(), &self.device);
            let white_win_tensor = Tensor::<B, 1>::from_data(item.white_win_chance.as_slice(), &self.device);
            let white_move_tensor = Tensor::<B, 1>::from_data(item.white_move_accuracy.as_slice(), &self.device);
            let white_legal_tensor = Tensor::<B, 1>::from_data(item.white_legal_moves.as_slice(), &self.device);
            let white_material_tensor = Tensor::<B, 1>::from_data(item.white_board_material.as_slice(), &self.device);

            let black_response_tensor = Tensor::<B, 1>::from_data(item.black_response_time.as_slice(), &self.device);
            let black_remaining_tensor = Tensor::<B, 1>::from_data(item.black_remaining_time.as_slice(), &self.device);
            let black_win_tensor = Tensor::<B, 1>::from_data(item.black_win_chance.as_slice(), &self.device);
            let black_move_tensor = Tensor::<B, 1>::from_data(item.black_move_accuracy.as_slice(), &self.device);
            let black_legal_tensor = Tensor::<B, 1>::from_data(item.black_legal_moves.as_slice(), &self.device);
            let black_material_tensor = Tensor::<B, 1>::from_data(item.black_board_material.as_slice(), &self.device);

            white_response_time.push(white_response_tensor.unsqueeze());
            white_remaining_time.push(white_remaining_tensor.unsqueeze());
            white_win_chance.push(white_win_tensor.unsqueeze());
            white_move_accuracy.push(white_move_tensor.unsqueeze());
            white_legal_moves.push(white_legal_tensor.unsqueeze());
            white_board_material.push(white_material_tensor.unsqueeze());

            black_response_time.push(black_response_tensor.unsqueeze());
            black_remaining_time.push(black_remaining_tensor.unsqueeze());
            black_win_chance.push(black_win_tensor.unsqueeze());
            black_move_accuracy.push(black_move_tensor.unsqueeze());
            black_legal_moves.push(black_remaining_tensor.unsqueeze());
            black_board_material.push(black_remaining_tensor.unsqueeze());
        }

        let white_response_time = Tensor::cat(white_response_time, 0);
        let white_remaining_time = Tensor::cat(white_remaining_time, 0);
        let white_win_chance = Tensor::cat(white_win_chance, 0);
        let white_move_accuracy = Tensor::cat(white_move_accuracy, 0);
        let white_legal_moves = Tensor::cat(white_legal_moves, 0);
        let white_board_material = Tensor::cat(white_board_material, 0);

        let black_response_time = Tensor::cat(black_response_time, 0);
        let black_remaining_time = Tensor::cat(black_remaining_time, 0);
        let black_win_chance = Tensor::cat(black_win_chance, 0);
        let black_move_accuracy = Tensor::cat(black_move_accuracy, 0);
        let black_legal_moves = Tensor::cat(black_legal_moves, 0);
        let black_board_material = Tensor::cat(black_board_material, 0);

        let label = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_data([item.label as i32], &self.device))
            .collect();

        let label = Tensor::cat(label, 0); 

        ChessGameBatch { 
            white_response_time, white_remaining_time, white_win_chance,
            white_move_accuracy, white_board_material, white_legal_moves,
            
            black_response_time, black_remaining_time, black_win_chance,
            black_move_accuracy, black_board_material, black_legal_moves,
            
            label,
        }
    }
}