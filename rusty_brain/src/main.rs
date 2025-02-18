use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
fn main() {
    // FEN string for the board
    let fen = String::from("1n1qkbnr/1P5p/1p6/1p1b1p2/1p3p2/1p3P2/1P6/1NBQKBNR b KQkq d6 0 2");
    
    // Create a Bitboards instance from the FEN string
    let board = Board::from_fen(fen);

    // Get the white pawns bitboard
    let mut white_pawns = board.bitboards.black_pawns;

    // Iterate over all white pawns
    let mut square = white_pawns.trailing_zeros() as u8;
    while square < 64 {
        // Create a bitboard for the current pawn
        let pawn_bitboard = 1u64 << square;

        // Check if the pawn is doubled isolated
        let is_doubled_isolated = board.doubled_isolated(square, pawn_bitboard);

        // Print the result
        println!("Is the white pawn on square {} doubled isolated? {}", square, is_doubled_isolated);
        // Move to the next set bit
        white_pawns ^= pawn_bitboard; // Clear the current bit
        square = white_pawns.trailing_zeros() as u8;
    }
    
    /* 
    //let white_fen = String::from("rnbqkbnr/pp1p1ppp/2p1p3/8/8/2P5/PP2PPPP/RN1QKBNR w KQkq - 0 2");
    //let black_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    //let white_board = Board::from_fen(white_fen);
    //let black_board = Board::from_fen(black_fen);
    
    //let color_flip_board = white_board.color_flip();

    //let white_sum = white_board.imbalance_total(&color_flip_board);
    // let black_sum = black_board.imbalance();

    //println!("White Sum = {}" , white_sum);
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
    */
}
