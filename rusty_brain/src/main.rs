use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::movement::Move;
fn main() {
    // println!("Core Engine");
    //let mut board = Board::from_fen("7k/8/PP4n1/KP1n1P2/8/2r5/8/8 w - - 1 1".to_string());
    
    //Example of stalemate, eval should give 0
    let mut board = Board::from_fen("1nb1n2k/8/1P6/KP6/2r5/8/8/8 b - - 1 1".to_string());
    board.make_move(Move::encode(58, 49, 0));
    board.print_board();
    //will find none so stalemate will be true
    _ = board.generate_legal_moves();
    println!("Evaluation of the stalemate (First example): {}", board.evaluate());
    
    //Example of white escaping from a mate in 1
    let mut board2 = Board::from_fen("7k/8/PP4n1/KP1n1P2/8/2r5/8/8 w - - 1 1".to_string());
    print!("\nBoard 2:");
    board2.print_board();
    let best = board2.find_best_move(3);
    println!("Best move in Board 2: {}{}, eval: {}", Square::from(best.0.get_from()), Square::from(best.0.get_to()), best.1);
    
    // let moves = board.generate_legal_moves();
    //for m in &moves {
    //    println!("move {}", m);
    //}
    
}
