pub struct Repository<'a> {
    pub test_models: Vec<(&'a str, &'a str, i8)>,
}

impl<'a> Repository<'a> {
    pub fn new() -> Self {
        Repository {
            test_models: Vec::new(),
        }
    }

    pub fn load_all_models(&mut self) {
        self. test_models = vec![
             // kan models
            ("cnn", "hook_lens/hook_lens_models/cnn_models/cnn_hook_lens/fdf/....", 1),
            ("kan_cnn", "hook_lens/hook_lens_models/ultra_agumented_kan_cnn_models/kan_cnn_hook_lens", 13),
            ("kan_cnn_256_grid_size_15_spline_order_12_scale_base_4_scale_noise_2" , "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/hook_lens_models/cnn_kan_models/kan_cnn_256_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2", 20),
            ("kan_cnn_512_grid_size_15_spline_order_12_scale_base_4_scale_noise_2" , "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/hook_lens_models/cnn_kan_models/kan_cnn_512_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2", 21),
            ("kan_cnn_1024_grid_size_15_spline_order_12_scale_base_4_scale_noise_2" , "/home/mostafayounis630/My_Projects/Graduation_Project/rough_hook/hook_lens/hook_lens_models/cnn_kan_models/kan_cnn_1024_hook_lens_grid_size_15_spline_order_12_scale_base_4_scale_noise_2", 22),
        ]
    }

    pub fn load_models_by_ids(&mut self, required_models_ids: Vec<i8>) {
        self.load_all_models();
        let mut required_models = Vec::new();

        println!("test models length = {}", self.test_models.len());
        for required_model_id in required_models_ids {
            for model in &self.test_models {
                if model.2 == required_model_id {
                    required_models.push((model.0, model.1, model.2));
                }
            }
        }
        if required_models.len() == 0 {
            panic!(" #### there is not any model match any of the ids you enterd ####");
        }
        self.test_models = required_models;
    }
}
