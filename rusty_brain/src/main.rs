use rusty_brain::{bitboards::{self, Bitboards}, board::{self, Board}, square, piece};
use rusty_brain::square::Square;
use rusty_brain::transposition::TranspositionTable;
use rusty_brain::uci;
use crate::piece::Piece;

use std::{io, result};
fn print_bitboard(bb: u64) {
    println!("\nBitboard visualization (LSB=a1, MSB=h8):");
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            print!("{} ", (bb >> square) & 1);
        }
        println!();
    }
    println!("  a b c d e f g h");
}

fn main() {

    loop {
        println!("\nEnter FEN string (or 'quit' to exit):");
        
        let mut fen = String::new();
        io::stdin()
            .read_line(&mut fen)
            .expect("Failed to read input");
            
        let fen = fen.trim();
        if fen.eq_ignore_ascii_case("quit") {
            break;
        }
        
        let board = Board::from_fen(fen.to_string());
        
        //println!("{}",board.pawnless_flank());
        println!("{}", board.king_ring(false).count_ones())
        // let flipped_board = board.color_flip();
        // //let (_, pins) = board.checks_and_pins();
        // //let (_, filp_pins) = flipped_board.checks_and_pins();
        // // // // Get the white pawns bitboard
        
        // let mut white_pawns = board.bitboards.white_pawns;
        // let mut black_pawns = flipped_board.bitboards.white_pawns;

        // // // // Test Flipping Color
        // let sum_white = board.pawns_mg();
        // let sum_black = flipped_board.pawns_mg();

        // println!("Sum White = {}", sum_white);
        
        // println!("Sum Black = {}", sum_black);
        // let mut sum_white = 0;
        // let mut sum_black = 0;


        // // Iterate over all white pawns
        //  while white_pawns != 0 {
        //     let white_square = white_pawns.trailing_zeros() as u8;
        //     let square_position = 1 << white_square;
        //     let result = board.backward(square_position, white_square);
        //     // Check if the pawn is doubled isolated
        //     sum_white += result;

        //     println!("Is the white pawn on square {} Backward? {}", white_square, result);
        //     white_pawns &= white_pawns - 1;
        // }
        // // Iterate over all Black pawns
        // while black_pawns != 0 {
        //     let black_square = black_pawns.trailing_zeros() as u8;
        //     let square_position = 1 << black_square;
        //     let result = flipped_board.backward(square_position, black_square);
        //     // Check if the pawn is doubled isolated
        //     sum_black += result;

        //     println!("Is the Black pawn on square {} Backward? {}", black_square, result);
        //     black_pawns &= black_pawns - 1;
        // }

        // //Create bitboard of all pinned pieces
        // println!("White");
        // for &square in &pins {
        //     println!("{}", square);
        // }
        // println!("Black");
        // for &square in &filp_pins {
        //     println!("{}",square);
        // }
        //println!("Mobility_MG For White = {} ",board.mobility_mg(&pins));
        //println!("Mobility_MG For Black = {} ",flipped_board.mobility_mg(&filp_pins));

        // board = flipped_board;        
        // //Test Mobility Complete Code
        //     // Loop through each piece type
        //     let piece_types = [
        //         (board.bitboards.white_knights, 0, Piece::Knight),
        //         (board.bitboards.white_bishops, 1, Piece::Bishop),
        //         (board.bitboards.white_rooks, 2, Piece::Rook),
        //         (board.bitboards.white_queens, 3, Piece::Queen),
        //     ];
        //     let mobility_area = board.mobility_area(&filp_pins);
        //     for (bitboard, piece_index, piece_type) in piece_types.iter() {
        //         let mut pieces = *bitboard;
        //         while pieces != 0 {
        //             let square = pieces.trailing_zeros() as u64; // Get square position
        //             let square_position = 1 << square;

        //             let mobility_index = board.mobility(*piece_type, square_position, mobility_area, &filp_pins); // Call mobility function
                    
        //             println!("Piece is {} and Mobility = {}",piece_index, mobility_index);
        //             pieces &= pieces - 1; // Remove piece from bitboard
        //         }
        //     }
         
        // let mobility = board.mobility_area();
        // // Print the raw hex value
        // println!("Mobility bitboard: {}", mobility.count_ones());
            
        // // Print the visual 8x8 representation
        // print_bitboard(mobility);

        // let mobility_flip = flipped_board.mobility_area();
        // // Print the raw hex value
        // println!("Flipped Mobility bitboard: {}", mobility_flip.count_ones());
            
        // // Print the visual 8x8 representation
        // print_bitboard(mobility_flip);
        
        /*
        let queen_bitboard = board.bitboards.white_queens;
        let queen_attacked_squares = board.get_queen_attacked_squares_for_eval(&queen_bitboard);
        let mut remaining = queen_attacked_squares;
        println!("Number of squares = {}", queen_attacked_squares.count_ones());
        while remaining != 0 {
            let square = remaining.trailing_zeros() as u8;
            let file = (square % 8) as u8 + b'a';
            let rank = (square / 8) as u8 + b'1';
            println!("{}{}", file as char, rank as char); // e.g., "e4"
            remaining ^= 1 << square;
        }
        let queen_square = queen_bitboard.trailing_zeros() as u8;
        let legal_queen_bitboard = board.get_legal_bitboard(&queen_square, &pins, &queen_attacked_squares);
        println!("\nNumber of Leagal accessible squares: {}", legal_queen_bitboard.count_ones());
        */
        /*
            let squares = board.get_bishop_xray_attacked_squares(&board.bitboards.white_bishops);
            println!("\nNumber of accessible squares: {}", count_ones(squares));

            let bishop_square = board.bitboards.white_bishops.trailing_zeros() as u8;

            let legal_bishop_bitboard = board.get_legal_bitboard(&bishop_square, &pins, &squares);
            println!("\nNumber of accessible squares: {}", count_ones(legal_bishop_bitboard));
         */
        
        //       
        // println!("{}", queen_moves.len());
        // for &queen_move in &queen_moves{
        //     println!("{}", queen_move);
        // }
        
        
    }

//     let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
//     // // // Create a Bitboards instance from the FEN string
//     let board = Board::from_fen(fen);

//     let mobility = board.mobility_area();
// // Print the raw hex value
// println!("Mobility bitboard: 0x{:016x}", mobility);
    
// // Print the visual 8x8 representation
// print_bitboard(mobility);   
    
        
        
    // // board.print_board();
    // // println!("-------------------------");
    
    // // color_flip_board.print_board();

    // // println!("-------------------------");

    // // // Get the white pawns bitboard
    // let mut white_pawns = board.bitboards.white_pawns;
    // let mut black_pawns = color_flip_board.bitboards.white_pawns;

    // // // Test Flipping Color
    // let sum_white = board.pawns_mg();
    // let sum_black = color_flip_board.pawns_mg();

    // // let mut sum_white = 0;
    // // let mut sum_black = 0;


    // // // Iterate over all white pawns
    // //  while white_pawns != 0 {
    // //      let white_square = white_pawns.trailing_zeros() as u8;
    // //      let square_position = 1 << white_square;

    // //      // Check if the pawn is doubled isolated
    // //      sum_white += board.blocked(square_position, white_square);

    // //      // Print the result
    // //      //println!("Is the white pawn on square {} doubled? {}", square, is_doubled);
        
    // //      white_pawns &= white_pawns - 1;


    // // }

    // // // Iterate over all white pawns
    // // while black_pawns != 0 {
    // //     let black_square = black_pawns.trailing_zeros() as u8;
    // //     let square_position = 1 << black_square;

    // //     // Check if the pawn is doubled isolated
    // //     sum_black += color_flip_board.blocked(square_position, black_square);

    // //     // Print the result
    // //     //println!("Is the white pawn on square {} doubled? {}", square, is_doubled);
        
    // //     black_pawns &= black_pawns - 1;

    // // }

    // println!("White Sum = {}", sum_white);
    // println!("Black Sum = {}", sum_black);
    // println!("Total Sum = {}", sum_white - sum_black);
    
    // /* 
    // //let white_fen = String::from("rnbqkbnr/pp1p1ppp/2p1p3/8/8/2P5/PP2PPPP/RN1QKBNR w KQkq - 0 2");
    // //let black_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    // //let white_board = Board::from_fen(white_fen);
    // //let black_board = Board::from_fen(black_fen);
    
    // //let color_flip_board = white_board.color_flip();

    // //let white_sum = white_board.imbalance_total(&color_flip_board);
    // // let black_sum = black_board.imbalance();

    // //println!("White Sum = {}" , white_sum);
    // // println!("Black Sum = {}" , black_sum);

    
    // // let mut board: Board = Board::new();
    // // let mut transposition_table: TranspositionTable = TranspositionTable::init();
    // // let depth = 6;
    // // let maximum_moves = 5;

    // // while board.checkmate != true && board.draw != true && board.move_log.len() <= maximum_moves {
    // //     let best = board.find_best_move(&mut transposition_table, depth);
    // //     board.make_move(best.0);
    // //     print!("{}{} ", Square::from(best.0.get_from()), Square::from(best.0.get_to()));
    // // }
    // // println!("\nGame ended");
    // */


    // let mut uci = uci::Uci::new();
    // uci.listen();
}