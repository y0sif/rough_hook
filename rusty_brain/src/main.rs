use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
use rusty_brain::uci;
fn main() {

    let fen = String::from("r1b1k1nr/6p1/2q1Pp2/pb1N2P1/1PPpP2P/1B1p4/6P1/R1Q2K2 b kq - 3 7");
    
    // // Create a Bitboards instance from the FEN string
    let mut board = Board::from_fen(fen);
    let mut color_flip_board = board.color_flip();

    // board.print_board();
    // println!("-------------------------");
    
    // color_flip_board.print_board();

    // println!("-------------------------");

    // // Get the white pawns bitboard
    // let mut white_pawns = board.bitboards.white_pawns;
    // let mut black_pawns = color_flip_board.bitboards.white_pawns;

    // // Test Flipping Color
    // let sum_white = board.pawns_mg();
    // let sum_black = color_flip_board.pawns_mg();

    // let mut sum_white = 0;
    // let mut sum_black = 0;

    let white_space = board.space(true);
    println!("White space: {}", white_space);

    let black_space = color_flip_board.space(true);
    println!("black_space: {}", black_space);

    // // Iterate over all white pawns
    //  while white_pawns != 0 {
    //      let white_square = white_pawns.trailing_zeros() as u8;
    //      let square_position = 1 << white_square;

    //      // Check if the pawn is doubled isolated
    //      sum_white += board.blocked(square_position, white_square);

    //      // Print the result
    //      //println!("Is the white pawn on square {} doubled? {}", square, is_doubled);
        
    //      white_pawns &= white_pawns - 1;


    // }

    // // Iterate over all white pawns
    // while black_pawns != 0 {
    //     let black_square = black_pawns.trailing_zeros() as u8;
    //     let square_position = 1 << black_square;

    //     // Check if the pawn is doubled isolated
    //     sum_black += color_flip_board.blocked(square_position, black_square);

    //     // Print the result
    //     //println!("Is the white pawn on square {} doubled? {}", square, is_doubled);
        
    //     black_pawns &= black_pawns - 1;

    // }

    // println!("White Sum = {}", sum_white);
    // println!("Black Sum = {}", sum_black);
    // println!("Total Sum = {}", sum_white - sum_black);
    
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


    // let mut uci = uci::Uci::new();
    // uci.listen();
}