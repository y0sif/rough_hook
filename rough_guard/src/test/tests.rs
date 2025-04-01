#[cfg(test)]
mod test {
    use crate::data::ChessGameBatch;
    use crate::data::ChessGameItem;
    use crate::data::FeaturesBatch;
    use crate::inference::infer;
    use crate::inference::load_model_paramter;
    use crate::inference::ModelEnum;
    use crate::test::test_models_repository::Repository;
    use burn::prelude::*;
    use burn_cuda::{Cuda, CudaDevice};
    use burn_dataset::Dataset;
    use burn_dataset::SqliteDataset;
    use prettytable::{row, Table};
    use std::path::Path;
    use std::time::Instant;

    #[test]
    fn test_all() {
        test_feb_data();
        assert_eq!(1, 0);
    }
    fn test_feb_data() {
        let mut models_results: Vec<(&str, i64, i64, f32, f32)> = Vec::new();
        // get the models to be tested from the repository of test models
        let mut repository = Repository::new();
        repository.load_models_by_ids(vec![1, 2]);

        for (model_name, model_path, id) in repository.test_models {
            println!("######## model name = {} ##########", model_name);

            let model: ModelEnum<Cuda<f32, i32>> =
                load_model_paramter::<Cuda<f32, i32>>(id, &model_path, CudaDevice::new(0));

            if model_name.len() == 0 || model_path.len() == 0 {
                continue;
            }

            let start = Instant::now();
            let (total_num_of_games, correct_predictions) = test_model(model);
            let duration = start.elapsed().as_secs_f32();
            let accuracy = (correct_predictions as f32 / total_num_of_games as f32) * 100 as f32;
            let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();

            models_results.push((
                model_name,
                total_num_of_games,
                correct_predictions,
                accuracy,
                duration,
            ));

            println!("\n======================================================================================\n");
        }

        print_results_table(models_results);
    }

    fn test_model(model: ModelEnum<Cuda<f32, i32>>) -> (i64, i64) {
        let db_file = Path::new("rough_guard/data_in_sql_lite/test_unnorm.db");
        let dataset: SqliteDataset<ChessGameItem> =
            SqliteDataset::from_db_file(db_file, "train").unwrap();
        let game_items = dataset.iter().collect::<Vec<_>>();

        let mut total_games = 0;
        let mut correct_predictions = 0;

        let mut actual_labels = [0; 4];
        let mut predicted_labels = [0; 4];
        let mut correct_prediction_labels = [0; 4];

        for game in game_items.iter() {
            total_games += 1;

            let prediction = infer(&model, CudaDevice::new(0), game);

            if prediction.0 == game.label as u8 {
                correct_predictions += 1;
                correct_prediction_labels[game.label as usize] += 1;
            }

            actual_labels[game.label as usize] += 1;
            predicted_labels[prediction.0 as usize] += 1;
        }
        println!(" #---- > Model  report : ");
        generate_report(
            &actual_labels,
            &predicted_labels,
            &correct_prediction_labels,
        );
        return (total_games, correct_predictions);
    }

    fn print_results_table(models_results: Vec<(&str, i64, i64, f32, f32)>) {
        // Create a new table
        let mut table = Table::new();

        // Add a header row
        table.add_row(row![
            "Model Name",
            "Total",
            "Correct",
            "Accuracy",
            "Time(s)"
        ]);

        // Iterate over the vector and add each tuple as a row in the table
        for (model_name, correct, wrong, accuracy, time) in models_results {
            table.add_row(row![model_name, correct, wrong, accuracy, time]);
        }
        // Print the table to the console
        table.printstd();
    }

    fn generate_report(
        actual_labels: &[i32],
        predicted_labels: &[i32],
        correct_prediction_labels: &[i32],
    ) {

        let mut table = Table::new();

        // Add a header row
        table.add_row(row![
            "Label",
            "#Actaul",
            "#Predicited",
            "#Correct Predictied",
            "Accuracy",
        ]);

        for i in 0..4 {
            let label = match i {
                0 => "Clean Game",
                1 => "White Cheating",
                2 => "Black Cheating",
                3 => "Both Cheating",
                _ => unreachable!(),
            };
            let accuracy =
                (correct_prediction_labels[i] as f32 / actual_labels[i] as f32) * 100 as f32;
            let accuracy = format!("{:.2}", accuracy).parse::<f32>().unwrap();

            table.add_row(row![
                label,
                actual_labels[i],
                predicted_labels[i],
                correct_prediction_labels[i],
                accuracy
            ]);
        }
        
        table.printstd();
    }
}

/*
println!("Confidence:");
    println!("- None:    {:.2}%", probs[0] * 100.0);
    println!("- White:   {:.2}%", probs[1] * 100.0);
    println!("- Black:   {:.2}%", probs[2] * 100.0);
    println!("- Both: {:.2}%", probs[3] * 100.0);
 */
