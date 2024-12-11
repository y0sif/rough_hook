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
<<<<<<< HEAD
        let mut position = [[0; 64]; 12];
=======
        
        // map used to make other side perspective
        
        let piece_to_piece_map: HashMap<usize, usize> = HashMap::from([
            (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
            (5, 0), (6, 1), (7, 2), (8, 3), (9, 4)            
        ]);

        let other_side_map: HashMap<usize, usize> = HashMap::from([
            (0, 56), (1, 57), (2, 58), (3, 59), (4, 60), (5, 61), (6, 62), (7, 63),
            (8, 48), (9, 49), (10, 50), (11, 51), (12, 52), (13, 53), (14, 54), (15, 55),
            (16, 40), (17, 41), (18, 42), (19, 43), (20, 44), (21, 45), (22, 46), (23, 47),
            (24, 32), (25, 33), (26, 34), (27, 35), (28, 36), (29, 37), (30, 38), (31, 39),
            (32, 24), (33, 25), (34, 26), (35, 27), (36, 28), (37, 29), (38, 30), (39, 31),
            (40, 16), (41, 17), (42, 18), (43, 19), (44, 20), (45, 21), (46, 22), (47, 23),
            (48, 8), (49, 9), (50, 10), (51, 11), (52, 12), (53, 13), (54, 14), (55, 15),
            (56, 0), (57, 1), (58, 2), (59, 3), (60, 4), (61, 5), (62, 6), (63, 7),
        ]);

        let mut position = [[0; 64]; 10];
        
>>>>>>> f44d247dd8cb524bc055f1eba9e62f36720fa799
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
<<<<<<< HEAD
        let position: Vec<i8> = position.into_iter()
                                .flat_map(|item| item).collect();
=======
        
        let mut other_perspective_position = position.clone();

        let mut set = HashSet::new();
        for i in 0..other_perspective_position.len() {
            for j in 0..other_perspective_position[i].len() {
                if other_perspective_position[i][j] == 1 {
                    if !set.insert((i, j)){
                        continue;
                    }
                    let idx_i = piece_to_piece_map.get(&i).unwrap();
                    let idx_j = other_side_map.get(&j).unwrap();
                    if !set.insert((*idx_i, *idx_j)){
                        continue;
                    }
                    other_perspective_position[i][j] = 0;
                    other_perspective_position[*idx_i][*idx_j] = 1;
                }
            }
        }
        
        other_square = *other_side_map.get(&other_square).unwrap();

        let position: Vec<i8> = position.into_iter()
                                .flat_map(|item| item).collect();

        let other_perspective_position: Vec<i8> = other_perspective_position.into_iter()
                                .flat_map(|item| item).collect();

        let mut side_to_move: Vec<Vec<i8>> = Vec::new();
        let mut other_to_move: Vec<Vec<i8>> = Vec::new();
        for i in 0..64{
            if i == to_move_square{
                side_to_move.push(position.clone());
            }else {
                side_to_move.push([0; 640].to_vec());
            }
            if i == other_square{
                other_to_move.push(other_perspective_position.clone());
            }else {
                other_to_move.push([0; 640].to_vec());
            }

        }
        let side_to_move: Vec<i8> = side_to_move.into_iter()
                                    .flat_map(|item| item).collect();
        let other_to_move: Vec<i8> = other_to_move.into_iter()
                                    .flat_map(|item| item).collect();

        assert_eq!(side_to_move.len(), 64 * 64 * 5 * 2);
        assert_eq!(other_to_move.len(), 64 * 64 * 5 * 2);
>>>>>>> f44d247dd8cb524bc055f1eba9e62f36720fa799
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
        let root: SqliteDataset<ChessPositionRaw> = HuggingfaceDatasetLoader::new("Lichess/chess-evaluations")
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

    
<<<<<<< HEAD
    pub fn sigd<const D: usize>(&self, inp:Tensor<B, D>) -> Tensor<B, D>{
        let sigmoid = Sigmoid::new();
        sigmoid.forward(inp / 400.0)
=======
    pub fn min_max_norm(&self, inp:Tensor<B, 1>) -> Tensor<B, 1>{
        let min = inp.clone().min();
        let max = inp.clone().max();
        (inp - min.clone()).div(max - min)
>>>>>>> f44d247dd8cb524bc055f1eba9e62f36720fa799
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
<<<<<<< HEAD
=======
        let evaluations = Tensor::cat(evaluations, 0); 
        let evaluations = self.min_max_norm(evaluations);
>>>>>>> f44d247dd8cb524bc055f1eba9e62f36720fa799

        let evaluations = Tensor::cat(evaluations, 0); 

        let evaluations = self.sigd(evaluations);

        ChessPositionBatch { fens, evaluations }
    }
}