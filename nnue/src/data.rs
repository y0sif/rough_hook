use burn::data::dataloader::batcher::Batcher;
use burn::prelude::*;
use burn_dataset::transform::Mapper;
use burn_dataset::transform::MapperDataset;
use burn_dataset::transform::PartialDataset;
use burn_dataset::transform::ShuffledDataset;
use burn_dataset::Dataset;
use burn_dataset::HuggingfaceDatasetLoader;
use burn_dataset::InMemDataset;
use burn_dataset::SqliteDataset;
use nn::Sigmoid;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChessPositionItem{
    pub fen_to_pieces: Vec<i8>,
    pub evaluation: f32,
} 


type MappedDataset = Box<dyn Dataset<ChessPositionItem>>;
pub struct ChessPositionDataSet{
    dataset: MappedDataset,
}

impl Dataset<ChessPositionItem> for ChessPositionDataSet {
    fn get(&self, index: usize) -> Option<ChessPositionItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}



impl ChessPositionDataSet {
    pub fn train() -> Self{
        Self::new("train")
    }

    pub fn test() -> Self{
        Self::new("test")
    }

    fn new(split: &str) -> Self{
        let db_file = Path::new("nnue/data_in_sql_lite/chess_positions_nnue.db");
        
        let dataset = SqliteDataset::from_db_file(db_file, split).unwrap();
        
        let dataset= ShuffledDataset::<SqliteDataset<ChessPositionItem>, ChessPositionItem>::with_seed(dataset, 42);
        type PartialData = PartialDataset<ShuffledDataset::<SqliteDataset<ChessPositionItem>, ChessPositionItem>, ChessPositionItem>;

        let data_split = match split {
            "train" => PartialData::new(dataset, 0, 800000), // Get first 80% dataset
            "test" => PartialData::new(dataset, 800000, 1000000), // Take remaining 20%
            _ => panic!("Invalid split type"),                     // Handle unexpected split types
        };

        let dataset = Box::new(data_split);

        ChessPositionDataSet {
            dataset
        }
    }
}

#[derive(Clone)]
pub struct ChessPositionBatcher<B: Backend>{
    device: B::Device,
}

impl<B: Backend> ChessPositionBatcher<B> {
    pub fn new(device: B::Device) -> Self{
        Self{ device }
    }

    
    pub fn sigd<const D: usize>(&self, inp:Tensor<B, D>) -> Tensor<B, D>{
        let sigmoid = Sigmoid::new();
        sigmoid.forward(inp / 400.0)
    }
}

#[derive(Debug, Clone)]
pub struct ChessPositionBatch<B: Backend> {
    pub fens: Tensor<B, 2>,
    pub evaluations: Tensor<B, 1>,
}

impl <B: Backend> Batcher<ChessPositionItem, ChessPositionBatch<B>> for ChessPositionBatcher<B>{
    fn batch(&self, items: Vec<ChessPositionItem>) -> ChessPositionBatch<B> {
        let mut fens: Vec<Tensor<B, 2>> = Vec::new();

        for item in items.iter() {
            let fens_tensor = Tensor::<B, 1>::from_data(item.fen_to_pieces.as_slice(), &self.device);
            fens.push(fens_tensor.unsqueeze());
        }

        let fens = Tensor::cat(fens, 0);

        let evaluations = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_data([item.evaluation as f32], &self.device))
            .collect();

        let evaluations = Tensor::cat(evaluations, 0); 

        let evaluations = self.sigd(evaluations);

        ChessPositionBatch { fens, evaluations }
    }
}