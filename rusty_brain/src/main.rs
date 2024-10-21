use rusty_brain::{board::{self, Board}, movement::Move, square::Square};

fn main() {
    println!("Core Engine");
    let mut board = Board::new();
    board.print_board();
    board.make_move(Move::encode(Square::E2 as u8, Square::E4 as u8, 0));
    board.print_board();
    board.make_move(Move::encode(Square::D7 as u8, Square::D5 as u8, 0));
    board.print_board();
    board.make_move(Move::encode(Square::E4 as u8, Square::D5 as u8, Move::CAPTURE));
    board.print_board();
    board.undo_move();
    board.print_board();
}
