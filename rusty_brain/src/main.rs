use rusty_brain::{board::{self, Board}, square::Square};

fn main() {
    println!("Core Engine");
    let mut board = Board::from_fen("2r4k/8/8/5p2/3B4/3K1p2/8/2b5 w - - 0 1".to_string());
    board.print_board();
}
