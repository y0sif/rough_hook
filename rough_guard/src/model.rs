use burn::nn::{BatchNormConfig, Dropout, DropoutConfig, Linear, LinearConfig};
use burn::nn::BatchNorm;
use burn::prelude::*;
use burn::tensor::backend::AutodiffBackend;
use burn::tensor::{activation::{relu, log_softmax}, Tensor};
use burn_efficient_kan::{Kan as EfficientKan, KanOptions};
use burn_jit::cubecl::prelude::le;

pub trait DeepLearningModel<B: Backend>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2>;
}

//////////////////////////////////////////////// MLP with ReLU, Sigmoid, and Dropout ///////////////////////////////////////////////////////
#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    pub linear_layers: Vec<Linear<B>>,
    pub dropout_layers: Vec<Dropout>,
    pub bn_layers: Vec<BatchNorm<B, 0>>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> Mlp<B> {
    pub fn new(
        layers_info: Vec<(usize, usize)>,
        class_weights: Tensor<B, 1>,
        dropout_prob: f64,
        device: &Device<B>,
    ) -> Self {
        let mut linear_layers: Vec<Linear<B>> = Vec::new();
        let mut dropout_layers: Vec<Dropout> = Vec::new();
        let mut bn_layers: Vec<BatchNorm<B, 0>> = Vec::new();

        for (i, &layer_info) in layers_info.iter().enumerate() {
            // Create linear layer
            let layer = LinearConfig::new(layer_info.0, layer_info.1).init(device);
            linear_layers.push(layer);

            if i < layers_info.len() - 1 {
                // Add BatchNorm after each linear layer except the last one
                let bn = BatchNormConfig::new(layer_info.1).init(device);
                bn_layers.push(bn);
                
                if i < 2 {
                    let dropout = DropoutConfig::new(dropout_prob).init();
                    dropout_layers.push(dropout);
                }
            }   
        }

        Self {
            linear_layers,
            bn_layers,
            dropout_layers,
            class_weights,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for Mlp<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;
        let num_layers = self.linear_layers.len();

        for i in 0..num_layers {
            // Apply linear transformation
            x = self.linear_layers[i].forward(x);

            // For all but the last layer: apply ReLU + Dropout
            if i < num_layers - 1 {
                x = self.bn_layers[i].forward(x);
                x = relu(x);
                if i < 2 {
                    // Apply dropout only for the first two layers
                    x = self.dropout_layers[i].forward(x);
                }
            }
        }
        x
    }
}


#[derive(Module, Debug)]
pub struct Mlp_no_bn<B: Backend> {
    pub linear_layers: Vec<Linear<B>>,
    pub dropout_layers: Vec<Dropout>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> Mlp_no_bn<B> {
    pub fn new(
        layers_info: Vec<(usize, usize)>,
        class_weights: Tensor<B, 1>,
        dropout_prob: f64,
        device: &Device<B>,
    ) -> Self {
        let mut linear_layers: Vec<Linear<B>> = Vec::new();
        let mut dropout_layers: Vec<Dropout> = Vec::new();

        for (i, &layer_info) in layers_info.iter().enumerate() {
            // Create linear layer
            let layer = LinearConfig::new(layer_info.0, layer_info.1).init(device);
            linear_layers.push(layer);

            if i < layers_info.len() - 1 {
                let dropout = DropoutConfig::new(dropout_prob).init();
                dropout_layers.push(dropout);
                
            }   
        }

        Self {
            linear_layers,
            dropout_layers,
            class_weights,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for Mlp_no_bn<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = input;
        let num_layers = self.linear_layers.len();

        for i in 0..num_layers {
            // Apply linear transformation
            x = self.linear_layers[i].forward(x);

            // For all but the last layer: apply ReLU + Dropout
            if i < num_layers - 1 {
                x = relu(x);
                x = self.dropout_layers[i].forward(x);
            }
        }
        x
    }
}


//////////////////////////////////////////////// Focal Loss (Hangs - BROKEN) ///////////////////////////////////////////////////////

// pub struct FocalLoss<B: AutodiffBackend> {
//     gamma: f64,
//     alpha: Tensor<B, 1>,
// }

// impl<B: AutodiffBackend> FocalLoss<B> {
//     pub fn new(device: &Device<B>, alpha: Tensor<B, 1>, gamma: f64) -> Self {
//         Self { gamma, alpha }
//     }
// }
// impl<B: AutodiffBackend> FocalLoss<B> {
//     pub fn forward(
//         &self,
//         logits: Tensor<B, 2>,
//         labels: Tensor<B, 1, burn::tensor::Int>
//         ) -> Tensor<B, 1> {

//         // 1) compute log-probs: [B, C]
//         let log_probs = log_softmax(logits, 1);

//         // 2) gather the log-prob of the true class → [B]
//         //    gather(dim, indices) expects indices: Tensor<B, D, Int>
//         let labels_idx: Tensor<B, 2, burn::tensor::Int> = 
//             labels.clone().unsqueeze_dim(1);     // [B, 1]
//         let true_log_p = log_probs
//             .gather(1, labels_idx)        // [B, 1]
//             .squeeze(1);                      // → [B]

//         // 3) focal weight (1 - p_t)^γ
//         let raw_pt = true_log_p.clone().exp();
//         let pt = raw_pt.clamp(1e-8, 1.0); // avoid log(0)

//         let ones = Tensor::ones(pt.shape(), &pt.device());      // [B]
//         let base = (ones - pt).clamp(0.0, 1.0);                  // ensure [0,1]
//         let weight = base.powf_scalar(self.gamma);

//         // 4) per-class α gathered at labels → [B]

//         let alpha_factor = self
//             .alpha.clone()         // [C]
//             .gather(0, labels); // [B]

//         // 5) focal loss per sample: -α * (1-p_t)^γ * log p_t
//         let loss = -alpha_factor * weight * true_log_p; // [B]

//         //let loss_sum_1d = loss.clone().sum();
//         //let loss_sum: Tensor<B, 0> = loss_sum_1d.reshape([] as [usize; 0]);
//         //let mean_loss: Tensor<B, 1> = loss_sum_1d / (loss.shape().num_elements() as f32);
        
//         //mean_loss
//         loss.mean()
//     }
// }

//////////////////////////////////////////////// Kan Template ///////////////////////////////////////////////////////
#[derive(Module, Debug)]
pub struct ModifiedKan<B: Backend> {
    pub kan_layers: Vec<EfficientKan<B>>,
    pub class_weights: Tensor<B, 1>,
}

impl<B: Backend> ModifiedKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    pub fn new(
        layers_info: Vec<([i32; 3], [Option<i32>; 4])>,
        class_weights: Tensor<B, 1>,
        device: &Device<B>,
    ) -> Self {
        // layers info : each vector in it represent one Kan layer

        let mut kan_layers: Vec<EfficientKan<B>> = Vec::new();

        for layer_info in layers_info.iter() {
            let kan_layer = construct_kan_layer(&layer_info.0, &layer_info.1, device);
            kan_layers.push(kan_layer);
        }
        Self {
            kan_layers,
            class_weights,
        }
    }
}

impl<B: Backend> DeepLearningModel<B> for ModifiedKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    fn forward(&self, games: Tensor<B, 2>) -> Tensor<B, 2> {
        let mut x = games;
        for kan_layer in &self.kan_layers {
            x = kan_layer.forward(x)
        }
        return x;
    }
}

fn construct_kan_layer<B: Backend>(
    options_values: &[i32; 3],
    hyper_parameters: &[Option<i32>; 4],
    device: &Device<B>,
) -> EfficientKan<B>
where
    B::FloatElem: ndarray_linalg::Scalar + ndarray_linalg::Lapack,
{
    let mut kan_options = KanOptions::new([
        options_values[0] as u32,
        options_values[1] as u32,
        options_values[2] as u32,
    ]);

    if let Some(grid_size) = hyper_parameters[0] {
        kan_options = kan_options.with_grid_size(grid_size as u16);
    }

    if let Some(spline_order) = hyper_parameters[1] {
        kan_options = kan_options.with_spline_order(spline_order as u32);
    }

    if let Some(scale_base) = hyper_parameters[2] {
        kan_options = kan_options.with_scale_base(scale_base as f32);
    }

    if let Some(scale_noise) = hyper_parameters[3] {
        kan_options = kan_options.with_scale_noise(scale_noise as f32);
    }

    EfficientKan::new(&kan_options, device)
}
