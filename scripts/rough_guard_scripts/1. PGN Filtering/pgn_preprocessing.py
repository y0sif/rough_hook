import os
import sys
import inspect
import chess.pgn

# Define the rating ranges as (lower_bound, upper_bound) tuples.
RATING_RANGES = [
    (0, 599),
    (600, 799),
    (800, 999),
    (1000, 1199),
    (1200, 1399),
    (1400, 1599),
    (1600, 1799),
    (1800, 1999),
    (2000, 2199),
    (2200, 2399),
    (2400, 2599),
    (2600, 2799),
    (2800, 2999),
    (3000, 3200)
]

def get_rating_bucket(elo):
    """
    Given an Elo rating, return the rating bucket as a string (e.g. "600-799").
    If the rating doesn't fall into any predefined bucket, return "Other".
    """
    for lower, upper in RATING_RANGES:
        if lower <= elo <= upper:
            return f"{lower}-{upper}"
    return "Other"

def split_filtered_pgn(input_file, output_dir="apr_filtered_games"):
    """Process PGN file with strict filtering criteria.
    
    If input_file is '-' the script reads from sys.stdin (e.g. via piping).
    """
    # Handle piping: if input_file is '-', read from sys.stdin
    if input_file == '-':
        pgn_file = sys.stdin
    else:
        # Validate input file if not reading from stdin
        if not os.path.exists(input_file):
            print(f"Error: Input file '{input_file}' not found!")
            sys.exit(1)
        if os.path.getsize(input_file) == 0:
            print(f"Error: Input file '{input_file}' is empty!")
            sys.exit(1)
        pgn_file = open(input_file, 'r', encoding='utf-8')

    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    
    # Initialize counters:
    game_count = 0      # Accepted games
    processed = 0       # Total processed games
    rating_counters = {f"{lower}-{upper}": 0 for (lower, upper) in RATING_RANGES}
    rating_counters["Other"] = 0

    try:
        while True:
            try:
                game = chess.pgn.read_game(pgn_file)
                if game is None:
                    break  # End of file
                
                processed += 1
                # Process the game; if it passes all filters, update counters and save it.
                if not process_game(game, output_dir, game_count + 1, rating_counters):
                    continue
                
                game_count += 1
                if game_count % 100 == 0:
                    print(f"Accepted {game_count} games (processed {processed})...")
                    
            except Exception as e:
                print(f"Error processing game {processed}: {str(e)}")
                continue
    finally:
        # Only close the file if it's not sys.stdin
        if pgn_file is not sys.stdin:
            pgn_file.close()
                
    print(f"\nFinal results:")
    print(f"- Processed games: {processed}")
    print(f"- Accepted games: {game_count}")
    if processed > 0:
        print(f"- Rejection rate: {(processed - game_count)/processed:.1%}")

    # Print rating bucket statistics for accepted games only.
    print("\nRating breakdown for accepted games:")
    def sort_key(k):
        if k == "Other":
            return 999999  # Send "Other" to the end.
        return int(k.split('-')[0])
    for bucket in sorted(rating_counters.keys(), key=sort_key):
        print(f"{bucket}: {rating_counters[bucket]}")

def process_game(game, output_dir, index, rating_counters):
    """Apply all filters to a game; return True if accepted."""
    
    # Filter 1: Event must contain "rapid"
    event = game.headers.get("Event", "").lower()
    if "rapid" not in event:
        return False

    # Filter 2: Strict time control
    tc = game.headers.get("TimeControl", "")
    if tc != "600+0":
        return False

    # Filter 3: No bot players
    if game.headers.get("WhiteTitle") == "BOT" or game.headers.get("BlackTitle") == "BOT":
        return False

    # Filter 4: Elo range check
    try:
        white_elo = int(game.headers["WhiteElo"])
        black_elo = int(game.headers["BlackElo"])
        # Ensure both players fall into the same 200-point bucket.
        if (white_elo // 200) != (black_elo // 200):
            return False
    except (KeyError, ValueError):
        return False

    # Filter 5: Minimum moves filter
    # Only accept games with at least 30 full moves (i.e. 60 half-moves)
    half_moves = list(game.mainline_moves())
    if len(half_moves) < 60:
        return False

    # Filter 6: Check for [%eval annotations in at least one comment.
    if not has_eval_annotation(game):
        return False

    # Game is accepted: update the rating counter.
    # (Since both players are in the same group, we use white_elo as representative.)
    bucket = get_rating_bucket(white_elo)
    rating_counters[bucket] += 1

    # Save qualifying game with the given index.
    save_game(game, output_dir, index)
    return True

def has_eval_annotation(game):
    """Check if game contains at least one [%eval annotation."""
    node = game
    while node is not None:
        if "[%eval" in node.comment:
            return True
        node = node.next()
    return False

def save_game(game, output_dir, index):
    """Save game to file with proper formatting."""
    filename = f"rapid_{index:08d}.pgn"
    output_path = os.path.join(output_dir, filename)
    
    with open(output_path, 'w', encoding='utf-8') as out_file:
        exporter = get_string_exporter()
        out_file.write(game.accept(exporter))

def get_string_exporter():
    """
    Create a StringExporter.
    Use the 'precision' parameter if it is supported by the installed python-chess version.
    """
    exporter_params = dict(
        headers=True,
        variations=True,
        comments=True,
    )
    # Use inspect to see if 'precision' is an accepted parameter.
    params = inspect.signature(chess.pgn.StringExporter).parameters
    if 'precision' in params:
        exporter_params['precision'] = 3
    return chess.pgn.StringExporter(**exporter_params)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python pgn_filter.py <input_file.pgn or -> [output_dir]")
        sys.exit(1)
        
    input_file = sys.argv[1]
    output_dir = sys.argv[2] if len(sys.argv) > 2 else "apr_filtered_games"
    
    split_filtered_pgn(input_file, output_dir)
