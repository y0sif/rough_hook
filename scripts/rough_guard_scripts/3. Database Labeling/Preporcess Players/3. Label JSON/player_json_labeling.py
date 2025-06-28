from tenacity import retry, wait_exponential, stop_after_attempt, retry_if_exception_type
from tqdm import tqdm
import requests
import random
import json
import os
import sys
import logging
from dotenv import load_dotenv

# Configuration
load_dotenv()

REQUEST_TIMEOUT = 30
CHECKPOINT_FILE = "checkpoint.txt"
LOG_FILE = "player_checker.log"
API_KEYS = os.getenv("API_KEYS", "").split(",")

rand = random.randint(0, len(API_KEYS) - 1)

LICHESS_TOKEN = API_KEYS[rand]

# Setup logging
logging.basicConfig(
    filename=LOG_FILE,
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    filemode='a'  # Append mode for multiple runs
)

class PlayerChecker:
    def __init__(self, input_file):
        self.input_file = input_file
        self.players = None
        self.current_index = 0
        self.last_api_call = 0
        self.temp_file = f"{input_file}.tmp"
        
        # Load existing data
        self.load_data()
        self.load_checkpoint()

    def load_data(self):
        """Load player data from JSON file"""
        try:
            with open(self.input_file, 'r') as f:
                self.players = json.load(f)
            # Convert to list of tuples for ordered processing
            self.player_items = list(self.players.items())
        except Exception as e:
            logging.error(f"Failed to load input file: {str(e)}")
            raise

    def load_checkpoint(self):
        """Load processing position from checkpoint"""
        try:
            if os.path.exists(CHECKPOINT_FILE):
                with open(CHECKPOINT_FILE, 'r') as f:
                    self.current_index = int(f.read().strip())
                    logging.info(f"Resuming from index {self.current_index}")
            print(f"Starting from index {self.current_index}\nPlayer: {self.player_items[self.current_index]}")
        except Exception as e:
            logging.warning(f"Checkpoint load failed: {str(e)} - starting from 0")
            self.current_index = 0

    def save_checkpoint(self):
        """Save current processing position"""
        try:
            with open(CHECKPOINT_FILE, 'w') as f:
                f.write(str(self.current_index))
        except Exception as e:
            logging.error(f"Checkpoint save failed: {str(e)}")

    def log_retry_attempt(retry_state):
        """Log retries with specific error type"""
        exc = retry_state.outcome.exception()
        exc_type = type(exc).__name__ if exc else "UnknownError"
        logging.info(f"Retrying API call due to {exc_type}...")

    @retry(
        wait=wait_exponential(multiplier=1, min=1, max=10),
        stop=stop_after_attempt(5),
        retry=retry_if_exception_type((requests.exceptions.RequestException,)),
        before_sleep=log_retry_attempt
    )
    def check_player(self, username):
        """Check player status with rate limiting"""
        if not username:
            return 1
    
        try:
            headers = {"Authorization": f"Bearer {LICHESS_TOKEN}"}
            response = requests.get(
                f"https://lichess.org/api/user/{username}",
                headers=headers,
                timeout=REQUEST_TIMEOUT
            )

            if response.status_code == 200:
                data = response.json()
                return 1 if data.get("disabled") or data.get("tosViolation") else 0
            elif response.status_code == 404:
                return 1  # Account doesn't exist
            else:
                response.raise_for_status()

        except requests.exceptions.HTTPError as e:
            if e.response.status_code == 429:
                logging.warning("Rate limit exceeded")
                self.save_progress()
                self.save_checkpoint()
                print("\nRate Limit Reached. Progress saved. Resume with another IP.")
                sys.exit(1)
            raise

    def save_progress(self):
        """Atomically save progress to temporary file"""
        try:
            # Write to temporary file first
            with open(self.temp_file, 'w') as f:
                json.dump(dict(self.player_items), f, indent=2)

            # Replace original file
            os.replace(self.temp_file, self.input_file)
            logging.info(f"Saved progress up to index {self.current_index}")

        except Exception as e:
            logging.error(f"Failed to save progress: {str(e)}")
            raise

    def process_players(self):
        """Main processing loop with progress tracking"""
        try:
            with tqdm(total=len(self.player_items), initial=self.current_index, 
                     desc="Processing players") as pbar:
                while self.current_index < len(self.player_items):
                    username, current_status = self.player_items[self.current_index]

                    try:
                        logging.info(f"Checking {username}...")
                        new_status = self.check_player(username)
                        if new_status != current_status:
                            self.player_items[self.current_index] = (username, new_status)
                            logging.info(f"Updated {username}: {current_status} -> {new_status}")

                        # Save progress every 100 players
                        if self.current_index % 100 == 0:
                            self.save_progress()
                            self.save_checkpoint()

                    except Exception as e:
                        logging.error(f"Failed to process {username}: {str(e)}")
                        # Skip to next player after retries exhausted
                        self.player_items[self.current_index] = (username, 2)

                    self.current_index += 1
                    pbar.update(1)

            # Final save
            self.save_progress()
            os.remove(CHECKPOINT_FILE)  # Clean up checkpoint
            logging.info("Processing completed successfully")

        except KeyboardInterrupt:
            logging.info("Processing interrupted by user")
            self.save_progress()
            self.save_checkpoint()
            print("\nProcess interrupted. Progress saved. Resume with same command.")

        except Exception as e:
            logging.error(f"Fatal error: {str(e)}")
            self.save_progress()
            self.save_checkpoint()
            raise

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description='Check player statuses on Lichess')
    parser.add_argument('input_file', help='JSON file containing player data')
    args = parser.parse_args()

    if not os.path.exists(args.input_file):
        print(f"Error: Input file {args.input_file} not found!")
        sys.exit(2)

    checker = PlayerChecker(args.input_file)
    checker.process_players()