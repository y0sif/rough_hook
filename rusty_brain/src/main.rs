use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
fn main() {
    // let white_fen = String::from("1n2kb1r/pp1p2pp/5p1n/2p1q3/3P2P1/1Q2P3/PP2P1PP/R1B1KB1R w KQkq - 2 2");
    // let black_fen = String::from("1n2kb1r/pp1p2pp/5p1n/2p1q3/3P2P1/1Q2P3/PP2P1PP/R1B1KB1R b KQkq - 2 2");

    // let white_board = Board::from_fen(white_fen);
    // let black_board = Board::from_fen(black_fen);
    
    // let white_sum = white_board.psqt_mg();
    // let black_sum = black_board.psqt_mg();
    // println!("White Sum = {}" , white_sum);
    // println!("Black Sum = {}" , black_sum);
    
    // let mut board: Board = Board::new();
    // let mut transposition_table: TranspositionTable = TranspositionTable::init();
    // let depth = 6;
    // let maximum_moves = 5;

    // while board.checkmate != true && board.draw != true && board.move_log.len() <= maximum_moves {
    //     let best = board.find_best_move(&mut transposition_table, depth);
    //     board.make_move(best.0);
    //     print!("{}{} ", Square::from(best.0.get_from()), Square::from(best.0.get_to()));
    // }
    // println!("\nGame ended");
}
