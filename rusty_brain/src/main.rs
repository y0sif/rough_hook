use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
fn main() {
    // let white_fen = String::from("rn1qkBnr/1p1p1ppp/4bb2/1B1B1Np1/B1BB1b2/2NR1B2/PPPPPPPP/2BQK3 w KQkq - 0 1");
    // let black_fen = String::from("rn1qkBnr/1p1p1ppp/4bb2/1B1B1Np1/B1BB1b2/2NR1B2/PPPPPPPP/2BQK3 b KQkq - 0 1");

    // let white_board = Board::from_fen(white_fen);
    // let black_board = Board::from_fen(black_fen);
    
    // let white_sum = white_board.imbalance();
    // let black_sum = black_board.imbalance();

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
