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
            //      ("unnorm_mlp_64_128_256_512_1024_512_256_128_64", "/home/y0sif/rough_guard_models/stratified/rough_guard_64_128_256_512_1024_512_256_128_64", 1),
            //      ("unnorm_mlp_64_128_256_512_1024_2048_1024_512_256_128_64", "/home/y0sif/rough_guard_models/stratified/rough_guard_64_128_256_512_1024_2048_1024_512_256_128_64", 2),
            ("temp test", "/tmp/rough_guard", 5),
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
