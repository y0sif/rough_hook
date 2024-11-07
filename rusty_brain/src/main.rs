use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};

fn main() {
    let mut val : u64 = 0 ; 
    let mask = ((1u64 << (4 + 1)) - 1) ^ ((1u64 << 6) - 1);
    val |= mask;
    println!("val = {:b}" , val); 

    // println!("Core Engine");
    let mut board = Board::from_fen("q6q/8/8/4b3/8/8/8/K6q w - - 0 1".to_string());
    let king_position = board.bitboards.white_king;
    board.checks_and_pins();    
}
