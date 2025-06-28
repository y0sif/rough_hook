import os
import chess.pgn
import json
import sqlite3
import argparse
from tqdm import tqdm

def update_labels(pgn_directory, mapping_file, db_file):
    # Load the JSON mapping of usernames to flagged status (0 or 1)
    with open(mapping_file, 'r', encoding='utf-8') as f:
        cheater_mapping = json.load(f)

    # Connect to the database (using SQLite in this example)
    conn = sqlite3.connect(db_file)
    cursor = conn.cursor()

    # Get a sorted list of PGN files from the directory
    pgn_files = sorted([f for f in os.listdir(pgn_directory) if f.endswith('.pgn')])
    pgn_files = [os.path.join(pgn_directory, f) for f in pgn_files]

    # Loop through each file and update the corresponding row in the DB
    row_id = 1  # row_id corresponds to index+1
    with tqdm(total=len(pgn_files), desc="Processing games") as pbar:
        for file_path in pgn_files:
            filename = os.path.basename(file_path)
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    game = chess.pgn.read_game(f)
                    if game is None:
                        print(f"Invalid Game {filename}")
                        continue
                    
                    # Get the player names from the PGN headers
                    white_name = game.headers.get("White", "").strip()
                    black_name = game.headers.get("Black", "").strip()
                    
                    # Retrieve player status from the mapping (defaulting to 0 if not found)
                    white_flagged = cheater_mapping.get(white_name, 1)
                    black_flagged = cheater_mapping.get(black_name, 1)
                    
                    # Calculate the label based on the formula:
                    # 0: both clean, 1: white flagged, 2: black flagged, 3: both flagged.
                    label = (white_flagged) + (2 * black_flagged)
                    
                    # Update the DB row that corresponds to the current game.
                    # It assumes that your DB table has a column "row_id" that matches the file order.
                    cursor.execute("UPDATE test SET label = ? WHERE row_id = ?", (label, row_id))
                    
                    row_id += 1  # Increment the row_id for the next game

            except Exception as e:
                print(f"Error processing {filename}: {str(e)}")
            finally:
                pbar.update(1)

    conn.commit()
    conn.close()
    print(f"Updated labels for {row_id - 1} games.")

if __name__ == '__main__':
    
    parser = argparse.ArgumentParser(description='Process chess games into feature vectors')
    parser.add_argument('input_dir', help='Directory containing PGN files')
    parser.add_argument('db_file', help='Database file to update')
    args = parser.parse_args()

    mapping_file = "/home/sasa/My_Projects/Graduation_Project/rough_hook/scripts/rough_guard_scripts/3. Database Labeling/Preporcess Players/2. Unique Player Concatentation/all_players.json"       # JSON file mapping usernames to 0 or 1

    update_labels(args.input_dir, mapping_file, args.db_file)
