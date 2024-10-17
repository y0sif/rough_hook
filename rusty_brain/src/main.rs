use rusty_brain::{bitboards::Bitboards, board::Board};

fn main() {

    println!("Core Engine");
    let mut board = Board::from_fen("3r4/b5b1/8/2PRP3/r1RKR2r/2PRP3/8/b2r2b1 w - - 0 1".to_string());
    board.print_board();
}
