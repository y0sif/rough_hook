use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};

fn main() { 
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0".to_string());
    board.print_board();
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("{m}");
    }
    board.make_move(moves[11]);
    board.print_board();
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("{m}");
    }
}
