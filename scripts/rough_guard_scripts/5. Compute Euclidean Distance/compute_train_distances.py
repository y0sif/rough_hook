import sqlite3
import numpy as np
import json
from collections import defaultdict
from tqdm import tqdm

DB_FILE = "merged_unnorm.db"
TABLE_NAME = "train"
DISTANCES_TABLE = "distances"
AVERAGES_JSON = "bucket_averages.json"

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
    return (hist / total) if total > 0 else hist

def euclidean_distance(hist1, hist2):
    return np.linalg.norm(hist1 - hist2)

# Create distances table if needed
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

# Main pipeline
def main():
    conn = sqlite3.connect(DB_FILE)
    create_distances_table(conn)
    cursor = conn.cursor()

    # 1. Load data
    cursor.execute(f"""
        SELECT row_id, response_time, remaining_time,
               win_chance, move_accuracy,
               board_material, legal_moves,
               bucket_index, label
        FROM {TABLE_NAME}
    """)

    all_rows = []
    bucket_to_rows = defaultdict(list)
    for row in cursor.fetchall():
        row_id, resp_blob, rem_blob, eval_blob, acc_blob, mat_blob, leg_blob, bucket_idx, label = row
        feature_dict = {
            "time": blob_to_int_array(resp_blob),
            "remaining": blob_to_int_array(rem_blob),
            "eval": blob_to_int_array(eval_blob),
            "accuracy": blob_to_int_array(acc_blob),
            "material": blob_to_int_array(mat_blob),
            "legal": blob_to_int_array(leg_blob)
        }
        row_dict = {"row_id": row_id, "bucket": bucket_idx, "label": label, "dict": feature_dict}
        all_rows.append(row_dict)
        bucket_to_rows[bucket_idx].append(row_dict)

    max_bins = {"time":30, "remaining":300, "eval":50, "accuracy":50, "material":58, "legal":59}

    # 2. Build bucket benchmarks and counts
    bucket_benchmarks = defaultdict(lambda: {"dict": {f: None for f in max_bins}})
    bucket_counts = {}

    for bucket_idx, rows in bucket_to_rows.items():
        clean_rows = [r for r in rows if r["label"] == 0]
        if not clean_rows:
            continue
        bucket_counts[bucket_idx] = len(clean_rows)
        for feature, max_bin in max_bins.items():
            sum_hist = None
            for r in clean_rows:
                arr = r["dict"][feature]
                hist = build_histogram(arr, max_bin)
                sum_hist = hist if sum_hist is None else sum_hist + hist
            bucket_benchmarks[bucket_idx]["dict"][feature] = sum_hist

    # 3. Save averages to JSON
    averages = {}
    for bucket_idx, bench in bucket_benchmarks.items():
        count = bucket_counts.get(bucket_idx, 0)
        if count == 0:
            continue
        averages[bucket_idx] = {}
        for feature, sum_hist in bench["dict"].items():
            if sum_hist is None:
                averages[bucket_idx][feature] = None
            else:
                averages[bucket_idx][feature] = (sum_hist / count).tolist()
    with open(AVERAGES_JSON, 'w') as f:
        json.dump(averages, f, indent=2)
    print(f"Saved bucket averages to {AVERAGES_JSON}")

    # 4. Compute per-row distances and insert
    insert_q = f'''INSERT INTO {DISTANCES_TABLE} (
            row_id, time_distance, remaining_distance,
            win_chance_distance, move_accuracy_distance,
            board_material_distance, legal_moves_distance,
            bucket_index, label) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)'''

    for row in tqdm(all_rows, desc="Computing and saving distances"):
        row_id, bucket_idx, label = row["row_id"], row["bucket"], row["label"]
        if bucket_idx not in bucket_benchmarks:
            continue
        per_dists = []
        for feature, max_bin in max_bins.items():
            bench = bucket_benchmarks[bucket_idx]["dict"][feature]
            if bench is None:
                per_dists.append(np.nan)
                continue
            row_hist = build_histogram(row["dict"][feature], max_bin)
            b_cut, r_cut = remove_sparse_bins(bench, row_hist)
            if b_cut.size == 0:
                per_dists.append(np.nan)
                continue
            b_norm = normalize_histogram(b_cut)
            r_norm = normalize_histogram(r_cut)
            per_dists.append(euclidean_distance(b_norm, r_norm))

        cursor.execute(insert_q, (
            row_id, *per_dists, bucket_idx, label
        ))

    conn.commit()
    conn.close()

if __name__ == '__main__':
    main()
