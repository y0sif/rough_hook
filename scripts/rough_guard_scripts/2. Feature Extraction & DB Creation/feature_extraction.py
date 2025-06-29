import os
import chess.pgn
import numpy as np
import argparse
import re
import sqlite3
from tqdm import tqdm
import shutil

# Configuration
A_VALUE = 0.00368208  # Logistic function parameter (1/271.7)
MAX_PLY = 60          # 30 moves * 2 plies
START_PLY = 21        # Move 11 starts at ply 21
CLK_REGEX = re.compile(r'\[%clk (\d+):(\d+):(\d+)\]')
EVAL_REGEX = re.compile(r'\[%eval ([#+-]?\d+\.?\d*)\]')


def create_table(conn):
    """Create the database table structure"""
    conn.execute('''
        CREATE TABLE IF NOT EXISTS test (
            row_id INTEGER PRIMARY KEY,

            white_response_time BLOB NOT NULL,
            white_remaining_time BLOB NOT NULL,
            white_win_chance BLOB NOT NULL,
            white_move_accuracy BLOB NOT NULL,
            white_board_material BLOB NOT NULL,
            white_legal_moves BLOB NOT NULL,

            black_response_time BLOB NOT NULL,
            black_remaining_time BLOB NOT NULL,
            black_win_chance BLOB NOT NULL,
            black_move_accuracy BLOB NOT NULL,
            black_board_material BLOB NOT NULL,
            black_legal_moves BLOB NOT NULL,
                 
            bucket_index INTEGER NOT NULL,
            label INTEGER NOT NULL
        )
    ''')
    conn.commit()

def process_directory(input_dir, conn):
    """Process all games in directory with progress tracking"""
    if not os.path.isdir(input_dir):
        raise ValueError(f"Input directory {input_dir} does not exist")

    # Get sorted list of PGN files
    pgn_files = sorted([f for f in os.listdir(input_dir) if f.endswith('.pgn')])
    pgn_files = [os.path.join(input_dir, f) for f in pgn_files]

    # Check how many files we've already processed
    cursor = conn.cursor()
    cursor.execute('SELECT COUNT(row_id) FROM test')
    processed_count = cursor.fetchone()[0]

    # Skip already processed files
    if processed_count > 0:
        print(f"Skipping first {processed_count} already processed files")
    pgn_files = pgn_files[processed_count:]

    with tqdm(total=len(pgn_files), desc="Processing games") as pbar:
        for file_path in pgn_files:
            filename = os.path.basename(file_path)
            try:
                features = process_game(file_path)
                if features:
                    binned = bin_features(features)
                    insert_into_db(conn, binned)
                else:
                    move_file(filename, file_path, "Erronous/")
            except Exception as e:
                print(f"Error processing {filename}: {str(e)}")
            finally:
                pbar.update(1)

def process_game(file_path):
    """Extract features from a single PGN game file"""
    features = {
        'white': {k: [] for k in ['time', 'remaining', 'eval', 'accuracy', 'legal', 'material']},
        'black': {k: [] for k in ['time', 'remaining', 'eval', 'accuracy', 'legal', 'material']},
        'label': 0,
        'bucket': 0
    }
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            game = chess.pgn.read_game(f)
            if not game:
                return None

            features['label'] = 0 #(1 * white_flagged) + (2 * black_flagged)
            
            # Set Rating Bucket
            white_elo = game.headers.get("WhiteElo", None)
            black_elo = game.headers.get("BlackElo", None)
            rating = None
            if white_elo:
                rating = white_elo
            elif black_elo:
                rating = black_elo
            
            if rating is not None:
                features['bucket'] = get_rating_bucket(rating)
            else:
                raise Exception("No Rating Found for this game")
            
            board = game.board()
            white_time = black_time = 600.0
            prev_win_chance = 50.0
            prev_legal_moves = 0
            prev_material = 0
            ply_counter = 1

            node = game
            node = node.next()
            if node:
                board.push(node.move)
            while node and ply_counter <= MAX_PLY:
                is_white = (ply_counter % 2) == 1
                player = 'white' if is_white else 'black'
                current_time = white_time if is_white else black_time
                
                # Time calculations
                clk_match = CLK_REGEX.search(node.comment)

                if clk_match:
                    h, m, s = map(int, clk_match.groups())
                    new_time = h * 3600 + m * 60 + s
                else:
                    new_time = current_time

                if ply_counter == START_PLY - 1:
                    eval_match = EVAL_REGEX.search(node.comment)
                    prev_win_chance = win_chance(parse_eval(eval_match.group(1)))
                    
                    prev_legal_moves = board.legal_moves.count()
                    prev_material = sum({
                        chess.PAWN: 1,
                        chess.KNIGHT: 3,
                        chess.BISHOP: 3,
                        chess.ROOK: 5,
                        chess.QUEEN: 9
                    }.get(p.piece_type, 0) for p in board.piece_map().values())

                if ply_counter >= START_PLY:
                    time_taken = max(0, min(current_time - new_time, 30))
                    
                    # Evaluation processing
                    eval_match = EVAL_REGEX.search(node.comment)
                    current_win_chance = win_chance(parse_eval(eval_match.group(1))) if eval_match else prev_win_chance

                    # Accuracy calculation
                    if not is_white:
                        current_win_chance = 100.0 - current_win_chance
                        prev_win_chance = 100.0 - prev_win_chance

                    delta = prev_win_chance - current_win_chance
                    accuracy = min(100.0, max(0.0, 100.0 - (delta * 3)))
                    
                    # Game state features
                    legal_moves = board.legal_moves.count()
                    material = sum({
                        chess.PAWN: 1,
                        chess.KNIGHT: 3,
                        chess.BISHOP: 3,
                        chess.ROOK: 5,
                        chess.QUEEN: 9
                    }.get(p.piece_type, 0) for p in board.piece_map().values())

                    # Store features
                    features[player]['time'].append(time_taken)
                    features[player]['remaining'].append(new_time)
                    features[player]['eval'].append(prev_win_chance)
                    features[player]['accuracy'].append(accuracy)
                    features[player]['legal'].append(prev_legal_moves)
                    features[player]['material'].append(prev_material)
                    
                    prev_material = material
                    prev_legal_moves = legal_moves

                # Update Clock
                if is_white:
                    white_time = new_time
                    prev_win_chance = current_win_chance if ply_counter >= START_PLY else prev_win_chance
                else:
                    black_time = new_time
                    prev_win_chance = 100.0 - current_win_chance if ply_counter >= START_PLY else prev_win_chance

                ply_counter += 1
                node = node.next()
                if node:
                    board.push(node.move)

        # Validate feature lengths
        if not all(len(v) == 20 for color in ['white', 'black'] for v in features[color].values()):
            return None
        
        return features
    except Exception as e:
        print(f"Exception: {e}")
        return None

def get_rating_bucket(rating):
    rating = int(rating)
    if rating < 600:
        return 0
    elif rating >= 3200:
        return 13
    else:
        return ((rating - 600) // 200) + 1

def win_chance(cp):
    """Convert centipawns to win chance"""
    return 100 / (1 + np.exp(-A_VALUE * cp))

def parse_eval(eval_str):
    """Handle mate and centipawn evaluations"""
    if '#' in eval_str:
        return 10000 * (1 if '+' in eval_str else -1)
    return float(eval_str) * 100

def bin_features(features):
    """Discretize features into bins"""
    return {
        'white': {
            'time': np.digitize(features['white']['time'], np.arange(0, 31, 1)) - 1,
            'remaining': np.digitize(features['white']['remaining'], np.arange(0, 602, 2)) - 1,
            'eval': np.digitize(features['white']['eval'], np.arange(0, 100.1, 2)) - 1,
            'accuracy': np.digitize(features['white']['accuracy'], np.arange(0, 100.1, 2)) - 1,
            'legal': np.clip(np.array(features['white']['legal']) - 1, 0, 59),
            'material': np.clip(np.array(features['white']['material']) - 20, 0, 58)
        },
        'black': {
            'time': np.digitize(features['black']['time'], np.arange(0, 31, 1)) - 1,
            'remaining': np.digitize(features['black']['remaining'], np.arange(0, 602, 2)) - 1,
            'eval': np.digitize(features['black']['eval'], np.arange(0, 100.1, 2)) - 1,
            'accuracy': np.digitize(features['black']['accuracy'], np.arange(0, 100.1, 2)) - 1,
            'legal': np.clip(np.array(features['black']['legal']) - 1, 0, 59),
            'material': np.clip(np.array(features['black']['material']) - 20, 0, 58)
        },
        'label': features['label'],
        'bucket': features['bucket']
    }

def insert_into_db(conn, binned):
    """Insert processed features into SQLite database"""
    cursor = conn.cursor()
    
    data = (
        sqlite3.Binary(binned['white']['time'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['white']['remaining'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['white']['eval'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['white']['accuracy'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['white']['material'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['white']['legal'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['time'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['remaining'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['eval'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['accuracy'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['material'].astype(np.int32).tobytes()),
        sqlite3.Binary(binned['black']['legal'].astype(np.int32).tobytes()),
        binned['bucket'],
        binned['label']
    )

    cursor.execute('''
        INSERT INTO test (
            white_response_time, white_remaining_time, white_win_chance, white_move_accuracy,
            white_board_material, white_legal_moves,
            black_response_time, black_remaining_time, black_win_chance, black_move_accuracy,
            black_board_material, black_legal_moves,
            bucket_index, label
        ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)
    ''', data)
    conn.commit()

def move_file(file_name, file_path, dest_dir):
    
    if not os.path.exists(dest_dir):
        os.makedirs(dest_dir)
        print(f"Created destination directory: {dest_dir}")
    if os.path.isfile(file_path):
        dest_path = os.path.join(dest_dir, file_name)
        shutil.move(file_path, dest_path)
        print(f"Moved '{file_path}' to '{dest_path}'")
    else:
        print(f"Couldn't move {file_name}")


if __name__ == "__main__":
    
    parser = argparse.ArgumentParser(description='Process chess games into feature vectors')
    parser.add_argument('input_dir', help='Directory containing PGN files')
    parser.add_argument('output_db', help='SQLite database file to store features')
    args = parser.parse_args()
    
    db_file = args.output_db
    db_exists = os.path.exists(db_file)
    conn = sqlite3.connect(db_file)
    create_table(conn)

    # Check existing row_id
    cursor = conn.cursor()
    cursor.execute('SELECT COUNT(row_id) FROM test')
    max_row = cursor.fetchone()[0]
    if max_row > 0:
        print(f"Resuming from file {max_row + 1}")
    else:
        print("Starting with new database")

    try:
        process_directory(args.input_dir, conn)
    finally:
        conn.close()