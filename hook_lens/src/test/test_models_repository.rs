use std::ptr::null;


pub struct repository<'a>{
    pub test_models: Vec<(&'a str , & 'a str, i8)>
}

impl<'a> repository<'a> {

    pub fn new()->Self{
        repository{
            test_models: Vec::new()
        }
    }
    pub fn load_all_models(& mut self){
        self. test_models = vec![
             // kan models
            ("ultra_cnn", "/home/sasa630/Graduation_Project/hook_lens_models/ultra_agumented_cnn_models/cnn_hook_lens", 1),
            ("ultra_kan_cnn", "/home/sasa630/Graduation_Project/hook_lens_models/ultra_agumented_kan_cnn_models/kan_cnn_hook_lens", 13),
        ]
    }

    pub fn get_model_by_id(self , id : i8)->(&'a str , &'a str, i8){
        let mut target_model = ("" , "" ,0);
        for model in self.test_models{
            if model.2 == id {
                target_model = (model.0 , model.1 , model.2)
            }
        }
        match target_model.2 {
            0=> panic!("there is no model with such id ]"),
            _=> target_model
        }
    }
}