# Scripts Directory

This directory contains data preprocessing and augmentation scripts for the Rough Hook project components.

## Hook Lens Scripts

Located in `hook_lens_scripts/` - Python scripts for computer vision data processing:

### `data_augmentation_script.py`
- Performs image data augmentation for training chess piece classification models
- Includes rotation, scaling, and other transformations to increase dataset diversity

### `image_resize_script.py` 
- Batch image resizing utilities for standardizing input dimensions
- Optimizes images for neural network training and inference

### `image_splitting_script.py`
- Dataset splitting functionality for train/validation/test sets
- Ensures proper data distribution for model evaluation

## Rough Guard Scripts

Located in `rough_guard_scripts/` - Python scripts for behavioral analysis data processing:

### `1. PGN Filtering/`
- Contains `pgn_preprocessing.py` for chess game filtering and preprocessing
- Filters PGN files based on game quality and player criteria

### `2. Feature Extraction & DB Creation/`
- Contains `feature_extraction.py` for extracting behavioral features from chess games
- Creates databases suitable for machine learning training

### `3. Database Labeling/`
- Contains `label_db.py` for automated labeling of training data
- Includes player preprocessing utilities in `Preprocess Players/`

### `4. Separate White & Black/`
- Contains `pgn_separate_features.py` for separating game data by player color
- Enables color-specific behavioral analysis

### `5. Compute Euclidean Distance/`
- Contains distance computation scripts for behavioral analysis
- Includes `bucket_averages.json`, `compute_test_distances_using_averages.py`, and `compute_train_distances.py`
- Computes player behavioral similarity metrics

## Usage

### Prerequisites
- Python 3.8+
- Required Python packages (install via `pip install -r requirements.txt` if available)
- Chess-related libraries (python-chess, pandas, numpy)

### Running Scripts
Navigate to the appropriate script directory and run individual Python scripts as needed:

```bash
# Example: Run data augmentation for Hook Lens
cd hook_lens_scripts/
python data_augmentation_script.py

# Example: Run PGN preprocessing for Rough Guard  
cd rough_guard_scripts/1.\ PGN\ Filtering/
python pgn_preprocessing.py
```

## Implementation Status

**Current Status**: Implemented and functional
- All scripts are implemented and ready for use
- Scripts support the data pipelines for both Hook Lens and Rough Guard components