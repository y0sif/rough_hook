import sqlite3
import argparse

def transform_label(original_label, is_white):
    if original_label == 0:
        return 0
    elif original_label == 1:
        return 1 if is_white else 0
    elif original_label == 2:
        return 0 if is_white else 1
    elif original_label == 3:
        return 1

def reformat_database(original_db, new_db):
    # Connect to the original database
    conn = sqlite3.connect(original_db)
    cursor = conn.cursor()
    
    # Connect to the new database
    new_conn = sqlite3.connect(new_db)
    new_cursor = new_conn.cursor()
    
    # Create the new table
    new_cursor.execute('''
        CREATE TABLE IF NOT EXISTS test (
            row_id INTEGER PRIMARY KEY,
            response_time BLOB,
            remaining_time BLOB,
            win_chance BLOB,
            move_accuracy BLOB,
            board_material BLOB,
            legal_moves BLOB,
            bucket_index INTEGER,
            label INTEGER
        )
    ''')
    
    # Fetch all rows from the original table
    cursor.execute("SELECT * FROM test")
    rows = cursor.fetchall()
    
    new_data = []
    for row in rows:
        row_id, white_time, white_remaining, white_win, white_acc, white_mat, white_moves, \
        black_time, black_remaining, black_win, black_acc, black_mat, black_moves, bucket, label = row
        
        new_data.append((white_time, white_remaining, white_win, white_acc, white_mat, white_moves, bucket, transform_label(label, True)))
        new_data.append((black_time, black_remaining, black_win, black_acc, black_mat, black_moves, bucket, transform_label(label, False)))
    
    # Insert transformed data into the new table
    new_cursor.executemany('''
        INSERT INTO test (response_time, remaining_time, win_chance, 
                              move_accuracy, board_material, legal_moves, 
                              bucket_index, label)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    ''', new_data)
    
    # Commit changes and close connections
    new_conn.commit()
    conn.close()
    new_conn.close()
    
if __name__ == "__main__":

    parser = argparse.ArgumentParser(description='Reformat chess game database to separate White and Black features')
    parser.add_argument('old_db', help='Directory containing PGN files')
    parser.add_argument('new_db', help='Database file to update')
    args = parser.parse_args()

    reformat_database(args.old_db, args.new_db)
