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
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChessPositionRaw {
    pub fen: String,
    #[serde(rename = "cp")]
    pub evaluation: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct ChessPositionItem{
    pub fen_to_pieces: Vec<i8>,
    pub evaluation: f32,
} 

pub struct RawToItem;

impl Mapper<ChessPositionRaw, ChessPositionItem> for RawToItem{
    fn map(&self, item: &ChessPositionRaw) -> ChessPositionItem {
        /*
        White
        king
        pawn
        knight
        bishop
        rook
        queen
        Black
        king
        pawn
        knight
        bishop
        rook
        queen
         */
        let map = HashMap::from([
            ('K', 0),
            ('P', 1),
            ('N', 2),
            ('B', 3),
            ('R', 4),
            ('Q', 5),
            ('k', 6),
            ('p', 7),
            ('n', 8),
            ('b', 9),
            ('r', 10),
            ('q', 11),
        ]);
        let mut position = [[0; 64]; 12];
        let fen_str: Vec<&str> = item.fen.split_whitespace().collect();
        let mut count: usize = 0;
        for piece in fen_str[0].chars(){
            if piece == '/'{
                continue;
            }
            if piece.is_numeric(){
                count += piece.to_digit(10).unwrap() as usize;
                continue;
            }
            if let Some(value) = map.get(&piece){
                position[*value][count] = 1;
                count += 1
            }
        }
        let position: Vec<i8> = position.into_iter()
                                .flat_map(|item| item).collect();
        ChessPositionItem {
            fen_to_pieces: position,
            evaluation: match item.evaluation {
                Some(val) => val,
                None => {
                    match fen_str[1]{
                        "w" => {
                            20000.0
                        },
                        "b" => {
                            -20000.0
                        },
                        _ => {
                            0.0
                        },
                    }
                }
            }
        }
    }
}

type MappedDataset = MapperDataset<InMemDataset<ChessPositionRaw>, RawToItem, ChessPositionRaw>;
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
        println!("dataloading started");
        type ChessEval = SqliteDataset<ChessPositionRaw>;
        let root: SqliteDataset<ChessPositionRaw> = HuggingfaceDatasetLoader::new("Lichess/chess-position-evaluations")
            .dataset("train") // The training split.
            .unwrap();

        println!("dataloading finished");

        let dataset = ShuffledDataset::<ChessEval,ChessPositionRaw>::with_seed(root, 42);
        type PartialData = PartialDataset<ShuffledDataset<ChessEval, ChessPositionRaw>, ChessPositionRaw>;

        let data_split = match split {
            "train" => PartialData::new(dataset, 0, 800000), // Get first 80% dataset
            "test" => PartialData::new(dataset, 800000, 1000000), // Take remaining 20%
            _ => panic!("Invalid split type"),                     // Handle unexpected split types
        };

        println!("data spliting done");
        let mut fens = Vec::new();
        let mut evals = Vec::new();
        for item in data_split.iter(){
            fens.push(item.fen);
            evals.push(item.evaluation);
        }

        let items: Vec<_> = fens
            .into_iter()
            .zip(evals)
            .map(|(fen, eval)| ChessPositionRaw { fen:fen, evaluation:eval})
            .collect();
        let dataset = InMemDataset::new(items);
        let dataset = MapperDataset::new(dataset, RawToItem);

        ChessPositionDataSet { dataset: dataset}
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