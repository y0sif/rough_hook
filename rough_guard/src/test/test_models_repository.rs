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
            // ("unnorm_kan_128_grid_size_2_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_128_grid_size_2_spline_order_2", 1),
            // ("unnorm_kan_256_grid_size_2_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_grid_size_2_spline_order_2", 1),
            // ("unnorm_kan_256_grid_size_2_spline_order_6", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_grid_size_2_spline_order_6", 1),
            // ("unnorm_kan_256_grid_size_6_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_grid_size_6_spline_order_2", 1),
            // ("unnorm_kan_512_grid_size_2_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_512_grid_size_2_spline_order_2", 1),
            ("unnorm_kan_1024_grid_size_2_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_1024_grid_size_2_spline_order_2", 1),
            // ("unnorm_kan_128_grid_size_2_spline_order_6", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_128_grid_size_2_spline_order_6", 2),
            // ("unnorm_kan_128_grid_size_6_spline_order_2", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_128_grid_size_6_spline_order_2", 3),
            // ("unnorm_kan_256_128_64_grid_size_6_spline_order_6", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_128_64_grid_size_6_spline_order_6", 4),
            // ("unnorm_kan_256_128_64_grid_size_8_spline_order_8", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_128_64_grid_size_8_spline_order_8", 5),
            // ("unnorm_kan_256_128_64_grid_size_10_spline_order_10", "/home/y0sif/rough_guard_models/kan/unnorm/rough_guard_256_128_64_grid_size_10_spline_order_10", 6),
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
