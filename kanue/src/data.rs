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
    pub side_to_move: Vec<f32>,
    pub other_side: Vec<f32>,
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
            ('P', 0),
            ('N', 1),
            ('B', 2),
            ('R', 3),
            ('Q', 4),
            ('p', 5),
            ('n', 6),
            ('b', 7),
            ('r', 8),
            ('q', 9),
        ]);
        let mut position = [[0.0; 64]; 10];
        
        let fen_str: Vec<&str> = item.fen.split_whitespace().collect();
        
        let (to_move, other) = match fen_str[1]{
            "w" => {
                ('K', 'k')
            },
            "b" => {
                ('k', 'K')
            },
            _ => {
                (' ', ' ')
            },
        };
        let mut to_move_square = 0;
        let mut other_square = 0;
        let mut count: usize = 0;
        for piece in fen_str[0].chars(){
            if piece == '/'{
                continue;
            }
            if piece.is_numeric(){
                count += piece.to_digit(10).unwrap() as usize;
                continue;
            }
            if piece == to_move{
                to_move_square = count;
                count += 1;
                continue;
            }
            if piece == other{
                other_square = count;
                count += 1;
                continue;
            }
            if let Some(value) = map.get(&piece){
                position[*value][count] = 1.0;
                count += 1
            }
        }

        let position: Vec<f32> = position.into_iter()
                                .flat_map(|item| item).collect();
        let mut side_to_move: Vec<Vec<f32>> = Vec::new();
        let mut other_to_move: Vec<Vec<f32>> = Vec::new();
        for i in 0..64{
            if i == to_move_square{
                side_to_move.push(position.clone());
            }else {
                side_to_move.push([0.0; 640].to_vec());
            }
            if i == other_square{
                other_to_move.push(position.clone());
            }else {
                other_to_move.push([0.0; 640].to_vec());
            }

        }
        let side_to_move: Vec<f32> = side_to_move.into_iter()
                                    .flat_map(|item| item).collect();
        let other_to_move: Vec<f32> = other_to_move.into_iter()
                                    .flat_map(|item| item).collect();

        assert_eq!(side_to_move.len(), 64 * 64 * 5 * 2);
        assert_eq!(other_to_move.len(), 64 * 64 * 5 * 2);
        ChessPositionItem {
            side_to_move,
            other_side: other_to_move,
            evaluation: match item.evaluation {
                Some(val) => val / 100.0,
                None => {
                    match fen_str[1]{
                        "w" => {
                            1000.0
                        },
                        "b" => {
                            -1000.0
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

    
    pub fn normalize<const D: usize>(&self, inp:Tensor<B, D>) -> Tensor<B, D>{
        inp.div_scalar(100.0)
    }
}

#[derive(Debug, Clone)]
pub struct ChessPositionBatch<B: Backend> {
    pub side_to_move: Tensor<B, 2>,
    pub other_side: Tensor<B, 2>,
    pub evaluations: Tensor<B, 1>,
}

impl <B: Backend> Batcher<ChessPositionItem, ChessPositionBatch<B>> for ChessPositionBatcher<B>{
    fn batch(&self, items: Vec<ChessPositionItem>) -> ChessPositionBatch<B> {
        let mut side_to_move: Vec<Tensor<B, 2>> = Vec::new();
        for item in items.iter() {
            let side_to_move_tensor = Tensor::<B, 1>::from_data(item.side_to_move.as_slice(), &self.device);
            side_to_move.push(side_to_move_tensor.unsqueeze());
        }

        let side_to_move = Tensor::cat(side_to_move, 0);
        
        let mut other_side: Vec<Tensor<B, 2>> = Vec::new();
        for item in items.iter() {
            let other_side_tensor = Tensor::<B, 1>::from_data(item.other_side.as_slice(), &self.device);
            other_side.push(other_side_tensor.unsqueeze());
        }

        let other_side = Tensor::cat(other_side, 0);


        let evaluations = items
            .iter()
            .map(|item| Tensor::<B, 1>::from_data([item.evaluation as f32], &self.device))
            .collect();
        let evaluations = Tensor::cat(evaluations, 0); 
        //let evaluations = self.min_max_norm(evaluations);
        let evaluations = self.normalize(evaluations);

        ChessPositionBatch { side_to_move, other_side, evaluations }
    }
}