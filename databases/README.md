# Databases Directory

This directory contains the datasets required for training and evaluation across all three research components of the Rough Hook project.

## 📁 Expected Directory Structure

```
databases/
├── chess_positions/           # Chess position datasets for Rusty Brain
│   ├── stockfish_binpack/     # Stockfish binary packed training data (any sfbinpack format)
│   ├── training_positions.db  # Training positions with evaluations
│   └── test_positions.db      # Test positions for engine evaluation
├── piece_images/              # Chess piece images for Hook Lens (10,000+ samples)
│   ├── train/                 # Training dataset
│   │   ├── bb/               # Black bishop images
│   │   ├── bk/               # Black king images
│   │   ├── bn/               # Black knight images
│   │   ├── bp/               # Black pawn images
│   │   ├── bq/               # Black queen images
│   │   ├── br/               # Black rook images
│   │   ├── wb/               # White bishop images
│   │   ├── wk/               # White king images
│   │   ├── wn/               # White knight images
│   │   ├── wp/               # White pawn images
│   │   ├── wq/               # White queen images
│   │   ├── wr/               # White rook images
│   │   └── empty/            # Empty square images
│   ├── test/                 # Test dataset (same structure as train)
│   └── validation/           # Validation dataset (same structure as train)
└── rough_guard_data/      # Behavioral datasets for Rough Guard
    ├── train_distances.db    # Training data with behavioral features
    ├── test_distances.db     # Test data with behavioral features
    └── raw_game_files/       # Raw game data files
        └── [Placeholder: Individual game files structure to be confirmed]
```

## 📥 Dataset Download Instructions

Due to the large size of the datasets (multiple GB), they are hosted externally:

### Step 1: Download from Google Drive
- [Placeholder: link to be added]

### Step 2: Extract to Databases Directory
- [Placeholder: extraction to be added]

## 📊 Dataset Specifications

### Chess Piece Images Dataset
- **Total Images**: 10,000+
- **Classes**: 13 (6 black pieces, 6 white pieces, 1 empty square)
- **Image Size**: 32x32 pixels (standardized)
- **Format**: PNG/JPG
- **Split**: 70% train, 15% validation, 15% test

### Chess Positions Dataset  
- **Format**: Stockfish binary packed data (sfbinpack format)
- **Size**: 1M+ positions
- **Sources**: Engine self-play, master games, tactical puzzles
- **Evaluations**: Centipawn values and game outcomes

### Behavioral Analysis Dataset
- **Format**: SQLite database with extracted features
- **Games**: 1M+ annotated chess games
- **Features**: Time patterns, move accuracy, engine correlation
- **Labels**: Binary classification (legitimate/suspicious)

## 🔧 Database Setup

- [Placeholder: setup to be added]

## 📋 Usage Notes

### For Hook Lens (Computer Vision)
- Images should be preprocessed to 32x32 pixels
- Ensure consistent lighting and perspective in custom datasets
- Use the provided train/test split for reproducible results

### For Rusty Brain (Chess Engine)
- Position evaluations should be in centipawn format
- Use Stockfish binary packed format (sfbinpack) for training data
- Include diverse position types (opening, middlegame, endgame)

### For Rough Guard (Behavioral Analysis)
- Features should be normalized for optimal model performance
- Maintain class balance or use appropriate weighting strategies
- Include temporal features for behavioral pattern detection

---

**Note**: This directory is currently empty. Please follow the download instructions to populate it with the required datasets.