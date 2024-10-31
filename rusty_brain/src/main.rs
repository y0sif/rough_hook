use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};

fn main() {
    // println!("Core Engine");
     let mut board = Board::from_fen("7k/8/1K1r2n1/5P2/8/8/8/8 w - - 0 1".to_string());
    board.print_board();
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("move {}", m);
    }
    
}
