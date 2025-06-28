//use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use burn_cuda::{Cuda, CudaDevice};
use burn_dataset::{Dataset, SqliteDataset};
use rough_guard::data::ChessGameItem;
use rough_guard::inference::{ModelEnum, infer, load_model_paramter};
use std::env;

fn main() {
    // Clean up existing database files
    cleanup_existing_db_files();

    let args: Vec<String> = env::args().collect();

    // Get folder path from command line arguments
    //
    if args.len() > 1 {
        let folder_path = &args[1];

        let sample_test_path = load_features_to_db(folder_path).unwrap();
        //label_db(&sample_test_path, &folder_path);
        let final_db_path = separate_features(&sample_test_path);
        compute_distances(
            &final_db_path,
            "/home/sasa/My_Projects/Graduation_Project/rough_hook/scripts/rough_guard_scripts/5. Compute Euclidean Distance/bucket_averages.json",
        );

        let model_path = "/home/sasa/My_Projects/Graduation_Project/rough_hook/rough_guard/rough_guard_models/mlp_model/model";

        let model: ModelEnum<Cuda<f32, i32>> =
            load_model_paramter::<Cuda<f32, i32>>(5, &model_path, CudaDevice::default());

        let predictions = predict(model, &final_db_path);
        let white_cheating = match &predictions.0 {
            0 => "No",
            1 => "Yes",
            _ => "Unknown",
        };
        let black_cheating = match &predictions.1 {
            0 => "No",
            1 => "Yes",
            _ => "Unknown",
        };
        let final_result = format!(
            "White cheating: {} , Black cheating: {}",
            white_cheating, black_cheating
        );
        println!("{}", final_result);
    }
}

fn cleanup_existing_db_files() {
    let current_dir =
        "/home/sasa/My_Projects/Graduation_Project/rough_hook/rough_guard_integration";

    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();

                    // Check if it's a database file
                    if file_name_str.ends_with(".db") {
                        match fs::remove_file(&path) {
                            Ok(_) => println!(""),
                            Err(e) => eprintln!("❌ Failed to delete {}: {}", file_name_str, e),
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("❌ Failed to read directory: {}", current_dir);
    }
}

fn load_features_to_db(folder_path: &str) -> Result<String, String> {
    let script_path =
        "../scripts/rough_guard_scripts/2. Feature Extraction & DB Creation/feature_extraction.py";
    let db_path =
        "/home/sasa/My_Projects/Graduation_Project/rough_hook/rough_guard_integration/sample.db";

    // Execute the Python script
    let output = Command::new("python3")
        .arg(script_path)
        .arg(folder_path)
        .arg(db_path)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python script failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(db_path.to_string())
}
fn label_db(db_path: &str, folder_path: &str) {
    let script_path = "../scripts/rough_guard_scripts/3. Database Labeling/label_db.py";
    let db_path =
        "/home/sasa/My_Projects/Graduation_Project/rough_hook/rough_guard_integration/sample.db";

    // Execute the Python script
    let output = Command::new("python3")
        .arg(script_path)
        .arg(folder_path)
        .arg(db_path)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e));
}

fn separate_features(db_path: &str) -> String {
    let new_db_path = "/home/sasa/My_Projects/Graduation_Project/rough_hook/rough_guard_integration/final_sample.db";
    let script_path =
        "../scripts/rough_guard_scripts/4. Separate White & Black/pgn_separate_features.py";

    // Execute the Python script
    let output = Command::new("python3")
        .arg(script_path)
        .arg(db_path)
        .arg(new_db_path)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e));
    new_db_path.to_string()
}
fn compute_distances(new_db_path: &str, average: &str) {
    let script_path = "../scripts/rough_guard_scripts/5. Compute Euclidean Distance/compute_test_distances_using_averages.py";

    // Execute the Python script
    let output = Command::new("python3")
        .arg(script_path)
        .arg(new_db_path)
        .arg(average)
        .output()
        .map_err(|e| format!("Failed to execute Python script: {}", e));
}

fn predict(model: ModelEnum<Cuda<f32, i32>>, db_file: &str) -> (u8, u8) {
    let dataset: SqliteDataset<ChessGameItem> =
        SqliteDataset::from_db_file(db_file, "distances").unwrap();
    let game_items = dataset.iter().collect::<Vec<_>>();

    let mut predictions = Vec::new();
    for game in game_items.iter() {
        let prediction = infer(&model, CudaDevice::new(0), game);
        predictions.push(prediction.0);
    }

    return (predictions[0], predictions[1]);
}
