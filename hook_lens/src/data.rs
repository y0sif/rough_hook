use burn::{data::dataset::{vision::ImageFolderDataset, Dataset, InMemDataset}, train::metric::store::Split};
use burn::data::dataset::transform::Mapper;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChessBoardSquareItem {
    pub board_squares: Vec<u8>,
    pub square_label: u8 
}

type MappedDataset = InMemDataset<ChessBoardSquareItem>;

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
    pub fn train() -> () { // return self
        Self::new("train")
    }

    /// Creates a new test dataset.
    pub fn test() -> () { // return self
        Self::new("test")
    }
    
    fn new(split: &str) -> () { // return self
        let root = match split {
            "train" => {
                let dataset = ImageFolderDataset::new_classification("hook_lens\\data\\train").unwrap();
                
                dataset
            },
            _ => {
                let dataset = ImageFolderDataset::new_classification("hook_lens\\data\\test").unwrap();

                dataset
            }
        };
        
        println!("data loaded");
        println!("data len {}", root.len());
        for item in root.iter() {
            let img = item.image.as_slice();
            for i in img {
                println!("{:?}", i);
                
                break;
            }
            println!("annot: {:?}", item.annotation);
            break;
        }

        todo!();
    }
    
}