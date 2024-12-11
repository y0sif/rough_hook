use burn::{
    module::Module, prelude::*, tensor::{
        backend:: Backend,
        Tensor,
    }
};
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};
use nn::Sigmoid;


#[derive(Module, Debug)]
pub struct Kan<B: Backend> {
    kan_layer: EfficientKan<B>,
    //sig: Sigmoid
}

#[derive(Config, Debug)]
pub struct KanConfig;

impl KanConfig {
    pub fn init<B: Backend>(&self, kan_options: &KanOptions, device: &B::Device) -> Kan<B>
    where
        B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
    {
        Kan{
            kan_layer: EfficientKan::new(kan_options, device),
            //sig: Sigmoid::new()
        }

    }
}

impl<B: Backend> Kan<B> {
    pub fn forward(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {                
        self.kan_layer.forward(positions)

        //Idk the purpose of this line, but remove if it breaks things
        //self.sig.forward(res)
    }
    pub fn infer(&self, positions: Tensor<B, 2>) -> Tensor<B, 2> {
        self.kan_layer.forward(positions)
    }
}