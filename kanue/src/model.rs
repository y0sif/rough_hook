use burn::{
    prelude::*,
    module::Module,
    tensor::{
        backend:: Backend,
        Tensor,
    },
};
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};


#[derive(Module, Debug)]
pub struct Kan<B: Backend> {
    to_move: EfficientKan<B>,
    other_side: EfficientKan<B>,
    kan_layer: EfficientKan<B>
}

#[derive(Config, Debug)]
pub struct KanConfig;

impl KanConfig {
    pub fn init<B: Backend>(&self, init_options: &KanOptions, kan_options: &KanOptions, device: &B::Device) -> Kan<B>
    where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
    {
        Kan{
            to_move: EfficientKan::new(init_options, device),
            other_side: EfficientKan::new(init_options, device),
            kan_layer: EfficientKan::new(kan_options, device)
        }

    }
}

impl<B: Backend> Kan<B> {
    pub fn forward(&self, side_to_move: Tensor<B, 2>, other_side: Tensor<B, 2> ) -> Tensor<B, 2> {
        let left = self.to_move.forward(side_to_move);
        let right = self.other_side.forward(other_side);

        let merge = Tensor::cat([left, right].to_vec(), 1);

        self.kan_layer.forward(merge)
    }

}