use burn::data;
use burn::data::dataloader::batcher::Batcher;
use burn::prelude::*;
use burn_dataset::transform::ShuffledDataset;
use burn_dataset::Dataset;
use burn_dataset::SqliteDataset;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChessGameItem {
    pub white_time_distance: f32,
    pub white_remaining_distance: f32,
    pub white_win_chance_distance: f32,
    pub white_move_accuracy_distance: f32,
    pub white_board_material_distance: f32,
    pub white_legal_moves_distance: f32,

    pub black_time_distance: f32,
    pub black_remaining_distance: f32,
    pub black_win_chance_distance: f32,
    pub black_move_accuracy_distance: f32,
    pub black_board_material_distance: f32,
    pub black_legal_moves_distance: f32,

    pub bucket_index: i32,
    pub label: i32,
}

pub fn test_deserialization() {
    let dataset = ChessGameDataSet::train();
    let item = dataset.get(0).unwrap();

    println!("w_time_distance: {}", item.white_time_distance);
    println!("w_remaining_distance: {}", item.white_remaining_distance);
    println!("w_win_chance_distance: {}", item.white_win_chance_distance);
    println!("w_move_accuracy_distance: {}", item.white_move_accuracy_distance);
    println!("w_board_material_distance: {}", item.white_board_material_distance);
    println!("w_legal_moves_distance: {}", item.white_legal_moves_distance);

    println!("b_time_distance: {}", item.black_time_distance);
    println!("b_remaining_distance: {}", item.black_remaining_distance);
    println!("b_win_chance_distance: {}", item.black_win_chance_distance);
    println!("b_move_accuracy_distance: {}", item.black_move_accuracy_distance);
    println!("b_board_material_distance: {}", item.black_board_material_distance);
    println!("b_legal_moves_distance: {}", item.black_legal_moves_distance); 

    println!("bucket_index: {}", item.bucket_index);
    println!("label: {}", item.label);
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
        let db_file = Path::new("/home/khaled/rough_hook/rough_guard/src/data_in_sql_lite/concat/pgn_unnorm.db");
        let dataset = SqliteDataset::from_db_file(db_file, "distances").unwrap();
       
        let (train_indices, test_indices) = Self::create_stratified_split(&dataset);
       
        let indices = match split {
            "train" => train_indices,
            "test" => test_indices,
            _ => panic!("Invalid split type"),
        };
        
        // Create a filtered dataset and shuffle it
        let dataset = FilteredDataset {
            source: dataset,
            indices,
        };

        let dataset = ShuffledDataset::with_seed(Box::new(dataset), 42);
        let dataset = Box::new(dataset);

        ChessGameDataSet { dataset }
    }
    fn create_stratified_split(dataset: &SqliteDataset<ChessGameItem>) -> (Vec<usize>, Vec<usize>) {
        let mut class_indices: Vec<Vec<usize>> = vec![Vec::new(), Vec::new()];
        for i in 0..dataset.len() {
            if let Some(item) = dataset.get(i) {
                let label = item.label as usize;
                if label < 2 {
                    class_indices[label].push(i);
                }
            }
        }
        
        let mut train_indices = Vec::new();
        let mut test_indices = Vec::new();
        
        for indices in class_indices.iter() {
            let train_count = indices.len() * 8 / 10;
            
            train_indices.extend(indices.iter().take(train_count).cloned());
            test_indices.extend(indices.iter().skip(train_count).cloned());
        }
        (train_indices, test_indices)
    }
}

struct FilteredDataset<D: Dataset<ChessGameItem>> {
    source: D,
    indices: Vec<usize>,
}

impl<D: Dataset<ChessGameItem>> Dataset<ChessGameItem> for FilteredDataset<D> {
    fn get(&self, index: usize) -> Option<ChessGameItem> {
        if index < self.indices.len() {
            self.source.get(self.indices[index])
        } else {
            None
        }
    }
    fn len(&self) -> usize {
        self.indices.len()
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
    pub features: Tensor<B, 2>,
    pub label: Tensor<B, 1, Int>,
}

impl<B: Backend> ChessGameBatch<B> {
    pub fn new(items: Vec<ChessGameItem>, device: &B::Device) -> Self {
        let features = Tensor::cat(
            items.iter().map(|item| {
                Tensor::<B, 1>::from_data(
                    vec![
                        item.white_time_distance,
                        item.white_remaining_distance,
                        item.white_win_chance_distance,
                        item.white_move_accuracy_distance,
                        item.white_board_material_distance,
                        item.white_legal_moves_distance,
                        
                        item.black_time_distance,
                        item.black_remaining_distance,
                        item.black_win_chance_distance,
                        item.black_move_accuracy_distance,
                        item.black_board_material_distance,
                        item.black_legal_moves_distance,
                        
                        item.bucket_index as f32,
                    ]
                    .as_slice(),
                    device,
                ).unsqueeze()
            }).collect::<Vec<_>>(),
            0,
        );
        let label = Tensor::cat(
            items.iter().map(|item| {
                Tensor::<B, 1, Int>::from_data([item.label], device)
            }).collect::<Vec<_>>(),
            0,
        );
        ChessGameBatch { features, label }
    }
}

impl<B: Backend> Batcher<ChessGameItem, FeaturesBatch<B>> for ChessGameBatcher<B> {
    fn batch(&self, items: Vec<ChessGameItem>) -> FeaturesBatch<B> {
        let batch = ChessGameBatch::new(items, &self.device);
        FeaturesBatch { features: batch.features, label: batch.label }
    }
}