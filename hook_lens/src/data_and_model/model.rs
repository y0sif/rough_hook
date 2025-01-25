use burn::
    prelude::*
;

use burn_efficient_kan::{Kan as EfficientKan, KanOptions};

#[derive(Module, Debug)]
pub struct Kan<B: Backend> {
    kan_layer: EfficientKan<B>,
}

impl<B: Backend> Kan<B> {
    pub fn new(num_classes: usize, device: &Device<B>) -> Self
    where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack, 
    {
        let kan_layer = EfficientKan::new(&KanOptions::new([
            3072,
            256,
            num_classes as u32
            ]), device);

        Self {
            kan_layer
        }
    }

    pub fn forward(&self, x: Tensor<B, 4>) -> Tensor<B, 2> {
        let x = x.flatten(1, 3);
        
        self.kan_layer.forward(x)
    }
}