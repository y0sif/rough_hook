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
        self.test_models = vec![
            ("No Dropout Expansion MLP (7-16-16-16-8-8-8-4-4-4-2)", "/home/khaled/mlp_rough_guard", 1),
            ("Dropout 0.3 Expansion MLP (7-16-16-16-8-8-8-4-4-4-2)", "/home/khaled/mlp_rough_guard_no_dropout", 2),
            ("No Dropout Reduction MLP (7-7-7-7-6-6-6-5-5-5-4-4-4-3-3-3-2-2-2)", "/home/khaled/salooha_mlp_rough_guard_no_dropout", 3),
            ("Dropout 0.3 Reduction MLP (7-7-7-7-6-6-6-5-5-5-4-4-4-3-3-3-2-2-2)", "/home/khaled/salooha_mlp_rough_guard_with_dropout", 4),
            ("Batch Normalization Dropout 0.15 Reduction MLP (7-7-7-7-6-6-6-5-5-5-4-4-4-3-3-3-2-2-2)", "/home/khaled/final_mlp_bn2", 5),
            ("KAN Reduction (7-6-5-4-3-2-2) (g_size = [20,10,5], s_order = [3,2,1])", "/home/khaled/kan_salooha", 6),
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