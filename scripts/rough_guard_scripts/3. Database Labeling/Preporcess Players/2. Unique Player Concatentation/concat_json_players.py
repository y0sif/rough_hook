import json
import argparse

def combine_json(file1_data, file2_data):
    """
    Combine two JSON dictionaries without shuffling.
    
    - All usernames from file1_data are output first with their original labels.
    - For each username in file2_data that does not appear in file1_data,
      add it with a label of 0.
    """
    combined = {}
    
    # Add entries from file1 first (preserving order)
    for username, label in file1_data.items():
        combined[username] = label

    # Add only those entries from file2 that are not already in file1
    for username in file2_data:
        if username not in combined:
            combined[username] = 0

    return combined

def main():
    parser = argparse.ArgumentParser(
        description="Combine two JSON files while preserving the order of file1 and "
                    "adding only unique usernames from file2 with a 0 label."
    )
    parser.add_argument("file1", help="Path to the first JSON file (primary)")
    parser.add_argument("file2", help="Path to the second JSON file (only unique usernames are added)")
    parser.add_argument("-o", "--output", help="Output file path", default="combined.json")
    args = parser.parse_args()

    try:
        with open(args.file1, 'r', encoding='utf-8') as f1:
            file1_data = json.load(f1)
    except Exception as e:
        print(f"Error reading {args.file1}: {e}")
        return

    try:
        with open(args.file2, 'r', encoding='utf-8') as f2:
            file2_data = json.load(f2)
    except Exception as e:
        print(f"Error reading {args.file2}: {e}")
        return

    # Ensure the inputs are dictionaries
    if not isinstance(file1_data, dict) or not isinstance(file2_data, dict):
        print("Error: Both JSON files must contain a dictionary mapping usernames to labels.")
        return

    combined = combine_json(file1_data, file2_data)

    try:
        with open(args.output, 'w', encoding='utf-8') as out_file:
            json.dump(combined, out_file, indent=4)
        print(f"Combined JSON written to {args.output}")
    except Exception as e:
        print(f"Error writing output: {e}")

if __name__ == "__main__":
    main()
