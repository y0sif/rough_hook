use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
fn main() {
    // FEN string for the board
    //let fen = String::from("rnbqkbnr/ppp2ppp/2PpP3/P1P3P1/8/P5P1/8/RNBQKBNR w KQkq - 0 2");
    

    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
    // Create a Bitboards instance from the FEN string
    let board = Board::from_fen(fen);

    // Get the white pawns bitboard
    let mut white_pawns = board.bitboards.white_pawns;

    // Iterate over all white pawns
    while white_pawns != 0 {
        let square = white_pawns.trailing_zeros() as u8;

        // Check if the pawn is doubled isolated
        let is_doubled = board.doubled(square);

        // Print the result
        println!("Is the white pawn on square {} doubled? {}", square, is_doubled);
        
        white_pawns &= white_pawns - 1;

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
