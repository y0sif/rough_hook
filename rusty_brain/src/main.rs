use rusty_brain::board::{self, Board};

fn main() {
    println!("Core Engine");
    
    let mut board = Board::from_fen("8/8/3p4/1Pp4r/1K5k/5p2/4P1P1/1R6 w - c6 0 1".to_string());
    board.print_board();
    for m in board.generate_legal_moves() {
        println!("move {}", m);
    }
}
