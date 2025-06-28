import os
import json
import chess.pgn
from tqdm import tqdm

def extract_players(pgn_dir, output_file="players.json"):
    """
    Extract unique player usernames from PGN files
    Save as JSON-serializable dictionary with default value 0
    """
    players = {}
    
    pgn_files = sorted([f for f in os.listdir(pgn_dir) if f.endswith('.pgn')])
    
    with tqdm(total=len(pgn_files), desc="Processing PGN files") as pbar:
        for filename in pgn_files:
            file_path = os.path.join(pgn_dir, filename)
            
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                while True:
                    try:
                        game = chess.pgn.read_game(f)
                        if not game:
                            break
                            
                        # Get player names with empty string fallback
                        white = game.headers.get("White", "").strip()
                        black = game.headers.get("Black", "").strip()
                        
                        # Add to dictionary with default 0 value
                        players[white] = players.get(white, 0)
                        players[black] = players.get(black, 0)
                    except Exception as e:
                        print("Error Processing File: ",e)
                        break

            pbar.update(1)
    
    # Remove empty string key if exists
    players.pop("", None)
    
    # Save as JSON for easy reloading
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(players, f, indent=2, ensure_ascii=False)
    
    print(f"Saved {len(players)} unique players to {output_file}")

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='Extract chess players from PGN files')
    parser.add_argument('input_dir', help='Directory containing PGN files')
    parser.add_argument('-o', '--output', default="players.json",
                       help='Output file path (default: players.json)')
    
    args = parser.parse_args()
    
    extract_players(args.input_dir, args.output)