use rusty_brain::{board::{self, Board}, movement::Move, square::Square};

fn main() {
    println!("Core Engine");
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0".to_string());
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("{}",m);
    }
    board.print_board();
    

    board.make_move(moves[0]);
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("{}",m);
    }
    board.print_board();

    board.make_move(moves[12]);
    let moves = board.generate_legal_moves();
    for m in &moves {
        println!("{}",m);
    }
    board.print_board();
    
    board.undo_move();
    board.print_board();
}
