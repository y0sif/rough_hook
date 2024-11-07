use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};

fn main() { 
    let mut board = Board::from_fen("8/2p5/3p4/Kr6/1R3p1k/4P3/6P1/8 w - - 0 1".to_string());
    board.checks_and_pins();
}
