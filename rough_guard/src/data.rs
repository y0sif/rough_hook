use burn::data::dataloader::batcher::Batcher;
use burn::prelude::*;
use burn_dataset::transform::PartialDataset;
use burn_dataset::transform::ShuffledDataset;
use burn_dataset::Dataset;
use burn_dataset::SqliteDataset;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChessGameItem {
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

    pub bucket: i32,
    pub label: i32,
} 

type MappedDataset = Box<dyn Dataset<ChessGameItem>>;
pub struct ChessGameDataSet {
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
    pub fn train() -> Self {
        Self::new("train")
    }

    pub fn test() -> Self {
        Self::new("test")
    }
    
    fn new(split: &str) -> Self {
        let db_file = Path::new("C:\\Users\\user\\Desktop\\Home\\Study\\University\\GP\\rough_hook\\rough_guard\\db\\pgn_features.db");
        let dataset = SqliteDataset::from_db_file(db_file, "train").unwrap();
        let dataset = ShuffledDataset::with_seed(dataset, 42);
        
        let total = dataset.len();
        let train_count = (total as f32 * 0.8).round() as usize;
        
        type PartialData = PartialDataset<ShuffledDataset<SqliteDataset<ChessGameItem>, ChessGameItem>, ChessGameItem>;
        let data_split = match split {
            "train" => PartialData::new(dataset, 0, train_count),
            "test" => PartialData::new(dataset, train_count, total),
            _ => panic!("Invalid split type"),
        };

        let dataset = Box::new(data_split);
        ChessGameDataSet { dataset }
    }
}

#[derive(Clone)]
pub struct ChessGameBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> ChessGameBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

#[derive(Debug, Clone)]
pub struct FeaturesBatch<B: Backend> {
    pub features: Tensor<B, 2>,
    pub label: Tensor<B, 1, Int>,
}

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

    pub bucket: Tensor<B, 1>,
    pub label: Tensor<B, 1, Int>,
}

impl<B: Backend> ChessGameBatch<B> {
    pub fn flatten(&self) -> Tensor<B, 2> {
        let tensors = vec![
            self.white_response_time.clone(), 
            self.white_remaining_time.clone(), 
            self.white_win_chance.clone(),
            self.white_move_accuracy.clone(), 
            self.white_board_material.clone(), 
            self.white_legal_moves.clone(),
            self.black_response_time.clone(), 
            self.black_remaining_time.clone(), 
            self.black_win_chance.clone(),
            self.black_move_accuracy.clone(), 
            self.black_board_material.clone(), 
            self.black_legal_moves.clone(),
            self.bucket.clone().unsqueeze(),
        ];

        Tensor::cat(tensors, 1)
    }
}

impl<B: Backend> Batcher<ChessGameItem, FeaturesBatch<B>> for ChessGameBatcher<B> {
    fn batch(&self, items: Vec<ChessGameItem>) -> FeaturesBatch<B> {
        let label = Tensor::cat(
            items.iter()
                .map(|item| Tensor::<B, 1, Int>::from_data([item.label], &self.device))
                .collect::<Vec<_>>(),
            0,
        );
        
        let feature_tensors = |f: fn(&ChessGameItem) -> &Vec<i32>| -> Tensor<B, 2> {
            Tensor::cat(
                items.iter()
                    .map(|item| Tensor::<B, 1>::from_data(f(item).as_slice(), &self.device).unsqueeze())
                    .collect::<Vec<_>>(),
                0,
            )
        };

        let bucket = Tensor::cat(
            items.iter()
                .map(|item| Tensor::<B, 1>::from_data([item.bucket], &self.device))
                .collect::<Vec<_>>(),
            0,
        );
        
        let batch = ChessGameBatch {
            white_response_time: feature_tensors(|item| &item.white_response_time),
            white_remaining_time: feature_tensors(|item| &item.white_remaining_time),
            white_win_chance: feature_tensors(|item| &item.white_win_chance),
            white_move_accuracy: feature_tensors(|item| &item.white_move_accuracy),
            white_board_material: feature_tensors(|item| &item.white_board_material),
            white_legal_moves: feature_tensors(|item| &item.white_legal_moves),

            black_response_time: feature_tensors(|item| &item.black_response_time),
            black_remaining_time: feature_tensors(|item| &item.black_remaining_time),
            black_win_chance: feature_tensors(|item| &item.black_win_chance),
            black_move_accuracy: feature_tensors(|item| &item.black_move_accuracy),
            black_board_material: feature_tensors(|item| &item.black_board_material),
            black_legal_moves: feature_tensors(|item| &item.black_legal_moves),
            
            bucket,
            label,
        };
        FeaturesBatch{
            features: batch.flatten(),
            label: batch.label
        }
    }
}
