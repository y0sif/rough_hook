import sqlite3
import numpy as np
import json
from tqdm import tqdm

# Configuration
DB_FILE = "test.db"
TEST_TABLE = "test"
DISTANCES_TABLE = "distances"
AVERAGES_JSON = "F:\Linux\\rough_guard_db\db_shit\\bucket_averages.json"

# Helper functions

def blob_to_int_array(blob, length=20):
    return np.frombuffer(blob, dtype=np.int32, count=length)


def build_histogram(arr, max_bin):
    hist = np.zeros(max_bin + 1, dtype=np.float64)
    for val in arr:
        if 0 <= val <= max_bin:
            hist[val] += 1
    return hist


def remove_sparse_bins(hist1, hist2):
    mask = (hist1 != 0) | (hist2 != 0)
    return hist1[mask], hist2[mask]


def normalize_histogram(hist):
    total = np.sum(hist)
    return hist / total if total > 0 else hist


def euclidean_distance(hist1, hist2):
    return np.linalg.norm(hist1 - hist2)

# Create distances table if it doesn't exist
def create_distances_table(conn):
    conn.execute(f'''
        CREATE TABLE IF NOT EXISTS {DISTANCES_TABLE} (
            row_id INTEGER PRIMARY KEY,
            time_distance REAL,
            remaining_distance REAL,
            win_chance_distance REAL,
            move_accuracy_distance REAL,
            board_material_distance REAL,
            legal_moves_distance REAL,
            bucket_index INTEGER,
            label INTEGER
        )
    ''')
    conn.commit()

# Main function
if __name__ == '__main__':
    # Load precomputed averages
    with open(AVERAGES_JSON, 'r') as f:
        raw_averages = json.load(f)
    # Convert lists back to numpy arrays
    bucket_averages = {
        int(bucket): {
            feat: np.array(hist) if hist is not None else None
            for feat, hist in feats.items()
        }
        for bucket, feats in raw_averages.items()
    }

    # Define feature max bins
    max_bins = {"time":30, "remaining":300, "eval":50,
                "accuracy":50, "material":58, "legal":59}

    # Connect to DB
    conn = sqlite3.connect(DB_FILE)
    create_distances_table(conn)
    cursor = conn.cursor()

    # Fetch test rows
    cursor.execute(f"""
        SELECT row_id, response_time, remaining_time,
               win_chance, move_accuracy,
               board_material, legal_moves,
               bucket_index, label
        FROM {TEST_TABLE}
    """ )

    rows = cursor.fetchall()
    insert_q = f'''INSERT OR REPLACE INTO {DISTANCES_TABLE} (
        row_id, time_distance, remaining_distance,
        win_chance_distance, move_accuracy_distance,
        board_material_distance, legal_moves_distance,
        bucket_index, label) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)'''

    # Compute and insert distances
    for row in tqdm(rows, desc="Computing test distances"):
        row_id, resp_blob, rem_blob, eval_blob, acc_blob, mat_blob, leg_blob, bucket_idx, label = row
        if bucket_idx not in bucket_averages:
            continue

        feature_arrays = {
            "time": blob_to_int_array(resp_blob),
            "remaining": blob_to_int_array(rem_blob),
            "eval": blob_to_int_array(eval_blob),
            "accuracy": blob_to_int_array(acc_blob),
            "material": blob_to_int_array(mat_blob),
            "legal": blob_to_int_array(leg_blob)
        }

        distances = []
        for feature, max_bin in max_bins.items():
            bench = bucket_averages[bucket_idx].get(feature)
            if bench is None:
                distances.append(np.nan)
                continue
            row_hist = build_histogram(feature_arrays[feature], max_bin)
            b_cut, r_cut = remove_sparse_bins(bench, row_hist)
            if b_cut.size == 0:
                distances.append(np.nan)
                continue
            distances.append(euclidean_distance(normalize_histogram(b_cut), 
                                                 normalize_histogram(r_cut)))

        cursor.execute(insert_q, (
            row_id, *distances, bucket_idx, label
        ))

    conn.commit()
    conn.close()
    print(f"Inserted test distances into table '{DISTANCES_TABLE}'")
