#[cfg(test)]
mod test {
    use std::path::Path;
    use burn_dataset::Dataset;
    use burn_dataset::SqliteDataset;
    use burn::prelude::*;
    use burn_cuda::{Cuda, CudaDevice};
    use crate::data::ChessGameBatch;
    use crate::data::ChessGameItem;
    use crate::data::FeaturesBatch;
    use crate::inference::infer;
    #[test]
    fn test_feb_data() {
        let device = CudaDevice::new(0);
        let db_file = Path::new("rough_guard/data_in_sql_lite/pgn_features_without_norm_feb.db");
        let dataset: SqliteDataset<ChessGameItem> = SqliteDataset::from_db_file(db_file, "train").unwrap();
        
        let items = dataset.iter().collect::<Vec<_>>();

        // do inference on each game 
        for game in items.iter() {
            // let prediction = infer(model, device, game);            
            
        }
    }
}