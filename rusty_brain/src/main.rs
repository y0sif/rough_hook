use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
fn main() {

    let mut board: Board = Board::new();
    let mut transposition_table: TranspositionTable = TranspositionTable::init();
    let depth = 6;
    let maximum_moves = 5;

    while board.checkmate != true && board.draw != true && board.move_log.len() <= maximum_moves {
        let best = board.find_best_move(&mut transposition_table, depth);
        board.make_move(best.0);
        print!("{}{} ", Square::from(best.0.get_from()), Square::from(best.0.get_to()));
    }
    println!("\nGame ended");
}
