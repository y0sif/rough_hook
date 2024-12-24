use burn::{data::{dataloader::batcher::Batcher, dataset::{vision::{Annotation, ImageFolderDataset, PixelDepth}, Dataset, InMemDataset}}, prelude::Backend, tensor::{ElementConversion, Int, Shape, Tensor, TensorData}};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChessBoardSquareItem {
    pub board_squares: Vec<u8>,
    pub square_label: usize 
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
    pub fn train() -> Self { // return self
        Self::new("train")
    }

    /// Creates a new test dataset.
    pub fn test() -> Self { // return self
        Self::new("test")
    }
    
    fn new(split: &str) -> Self { // return self
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
        let mut images = Vec::new();
        let mut labels = Vec::new();
        for item in root.iter() {
            let img = item.image.as_slice();
            let mut board_squares = Vec::new();
            for i in img {
                let pixel_value = match *i {
                    PixelDepth::U8(val) => val,
                    _ => { panic!("expected u8"); }
                };
                board_squares.push(pixel_value);
            }

            images.push(board_squares);

            let label = match item.annotation {
                Annotation::Label(val) => val,
                _ => { panic!("expected label"); }
            };
            labels.push(label);
        }

        let dataset = InMemDataset::new(images.iter().zip(labels.iter()).map(|(board_squares, square_label)| {
            ChessBoardSquareItem {
                board_squares: board_squares.clone(),
                square_label: *square_label
            }
        }).collect());

        ChessDataset {
            dataset
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
            .map(|item| TensorData::new(item.board_squares, Shape::new([227, 227, 3])))
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