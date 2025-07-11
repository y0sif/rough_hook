use burn::data;
use burn::data::dataloader::batcher::Batcher;
use burn::prelude::*;
use burn_dataset::transform::ShuffledDataset;
use burn_dataset::Dataset;
use burn_dataset::SqliteDataset;
use serde::{Deserialize, Serialize};
use std::path::Path;

//Unnormalized Separated Data

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChessGameItem {
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub response_time: Vec<i32>,
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub remaining_time: Vec<i32>,
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub win_chance: Vec<i32>,
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub move_accuracy: Vec<i32>,
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub board_material: Vec<i32>,
    #[serde(deserialize_with = "deserialize_chess_blob")]
    pub legal_moves: Vec<i32>,

    pub bucket_index: i32,
    pub label: i32,
}

pub fn deserialize_chess_blob<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let bytes: Vec<u8> = Vec::deserialize(deserializer)?;

    // Validate length for exactly 20 f32 elements (80 bytes)
    if bytes.len() != 80 {
        return Err(Error::custom(format!(
            "Invalid blob length: expected 80 bytes, got {}",
            bytes.len()
        )));
    }

    bytes
        .chunks_exact(4)
        .map(|chunk| {
            chunk
                .try_into()
                .map_err(|_| Error::custom("Failed to convert 4-byte chunk to array"))
                .map(i32::from_le_bytes)
        })
        .collect()
}

pub fn test_deserialization() {
    let dataset = ChessGameDataSet::train();
    let item = dataset.get(0).unwrap();

    //println!("First element: {}", item.bucket_index);
    item.response_time
        .iter()
        .for_each(|item| println!("{}", item));
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
        let db_file = Path::new("/home/khaled/rough_hook/rough_guard/src/data_in_sql_lite/pgn_unnorm.db");
        let dataset = SqliteDataset::from_db_file(db_file, "train").unwrap();

        // Create stratified train/test splits
        let (train_indices, test_indices) = Self::create_stratified_split(&dataset);

        // Create a filtered dataset using the appropriate indices
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
        // Group indices by class
        let mut class_indices: Vec<Vec<usize>> =
            vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for i in 0..dataset.len() {
            if let Some(item) = dataset.get(i) {
                let label = item.label as usize;
                if label < 2 {
                    class_indices[label].push(i);
                }
            }
        }

        // Create train and test indices with stratified split
        let mut train_indices = Vec::new();
        let mut test_indices = Vec::new();

        for indices in class_indices.iter() {
            let train_count = indices.len() * 8 / 10; // 80% for training

            // Add indices to respective splits
            train_indices.extend(indices.iter().take(train_count).cloned());
            test_indices.extend(indices.iter().skip(train_count).cloned());
        }

        (train_indices, test_indices)
    }
}

// Define the FilteredDataset struct outside the impl block
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
    pub response_time: Tensor<B, 2>,
    pub remaining_time: Tensor<B, 2>,
    pub win_chance: Tensor<B, 2>,
    pub move_accuracy: Tensor<B, 2>,
    pub board_material: Tensor<B, 2>,
    pub legal_moves: Tensor<B, 2>,

    pub bucket_index: Tensor<B, 2>,
    pub label: Tensor<B, 1, Int>,
}

impl<B: Backend> ChessGameBatch<B> {
    pub fn flatten(&self) -> Tensor<B, 2> {
        let tensors = vec![
            self.response_time.clone(),
            self.remaining_time.clone(),
            self.win_chance.clone(),
            self.move_accuracy.clone(),
            self.board_material.clone(),
            self.legal_moves.clone(),
            
            self.bucket_index.clone(),
        ];

        Tensor::cat(tensors, 1)
    }
}

impl<B: Backend> Batcher<ChessGameItem, FeaturesBatch<B>> for ChessGameBatcher<B> {
    fn batch(&self, items: Vec<ChessGameItem>) -> FeaturesBatch<B> {
        let label = Tensor::cat(
            items
                .iter()
                .map(|item| Tensor::<B, 1, Int>::from_data([item.label], &self.device))
                .collect::<Vec<_>>(),
            0,
        );

        let feature_tensors = |f: fn(&ChessGameItem) -> &Vec<i32>| -> Tensor<B, 2> {
            Tensor::cat(
                items
                    .iter()
                    .map(|item| {
                        Tensor::<B, 1>::from_data(f(item).as_slice(), &self.device).unsqueeze()
                    })
                    .collect::<Vec<_>>(),
                0,
            )
        };

        let bucket_index = Tensor::cat(
            items
                .iter()
                .map(|item| Tensor::<B, 1>::from_data([item.bucket_index], &self.device))
                .collect::<Vec<_>>(),
            0,
        )
        .unsqueeze_dim(1);

        let batch = ChessGameBatch {
            response_time: feature_tensors(|item| &item.response_time),
            remaining_time: feature_tensors(|item| &item.remaining_time),
            win_chance: feature_tensors(|item| &item.win_chance),
            move_accuracy: feature_tensors(|item| &item.move_accuracy),
            board_material: feature_tensors(|item| &item.board_material),
            legal_moves: feature_tensors(|item| &item.legal_moves),

            bucket_index,
            label,
        };
        FeaturesBatch {
            features: batch.flatten(),
            label: batch.label,
        }
    }
}