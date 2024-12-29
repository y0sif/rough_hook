use std::path::Path;

use burn::{data::{dataloader::batcher::Batcher, dataset::{Dataset, SqliteDataset}}, prelude::Backend, tensor::{ElementConversion, Int, Shape, Tensor, TensorData}};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChessBoardSquareItem {
    pub board_squares: Vec<u8>,
    pub square_label: u8 
}

type MappedDataset = SqliteDataset<ChessBoardSquareItem>;

pub struct ChessDataset {
    dataset: MappedDataset
}

impl Dataset<ChessBoardSquareItem> for ChessDataset {
    fn get(&self, index: usize) -> Option<ChessBoardSquareItem> {
        self.dataset.get(index)
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl ChessDataset {
    /// Creates a new train dataset.
    pub fn train() -> Self { // return self
        Self::new("train")
    }

    /// Creates a new test dataset.
    pub fn test() -> Self { // return self
        Self::new("test")
    }
    
    // no panics exist here
    fn new(split: &str) -> Self { // return self
        let train_db_file = Path::new("hook_lens//data_in_sql_lite//chess_pieces_images_tr.db");
        let test_db_file = Path::new("hook_lens//data_in_sql_lite//chess_pieces_images_ts.db");
        
        match split {
            "train" => {
                let dataset = SqliteDataset::from_db_file(train_db_file, split).unwrap();
                
                ChessDataset {
                    dataset
                }
            },
            _ => {
                
                let dataset = SqliteDataset::from_db_file(test_db_file, split).unwrap();
        
                ChessDataset {
                    dataset
                }
            }
        }
        
        
    }
    
}

#[derive(Clone, Debug)]
pub struct ChessBoardBatcher<B: Backend> {
    device: B::Device,
}

#[derive(Clone, Debug)]
pub struct ChessBoardBatch<B: Backend> {
    pub images: Tensor<B, 4>,
    pub targets: Tensor<B, 1, Int>,
}

impl<B: Backend> ChessBoardBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<ChessBoardSquareItem, ChessBoardBatch<B>> for ChessBoardBatcher<B> {
    fn batch(&self, items: Vec<ChessBoardSquareItem>) -> ChessBoardBatch<B> {
        let targets = items
            .iter()
            .map(|item| {
                Tensor::<B, 1, Int>::from_data(
                    TensorData::from([(item.square_label as i64).elem::<B::IntElem>()]),
                    &self.device,
                )
            })
            .collect();
            

        let images: Vec<Tensor<B, 3>> = items
            .into_iter()
            .map(|item| TensorData::new(item.board_squares, Shape::new([28, 28, 3])))
            .map(|data| {
                Tensor::<B, 3>::from_data(data.convert::<B::FloatElem>(), &self.device)
                    // permute(2, 0, 1)
                    .swap_dims(2, 1) // [H, C, W]
                    .swap_dims(1, 0) // [C, H, W]
            })
            .map(|tensor| tensor / 255) // normalize between [0, 1]
            .collect();

        let images = Tensor::stack(images, 0);
        let targets = Tensor::cat(targets, 0);

        ChessBoardBatch { images, targets }
    }
}