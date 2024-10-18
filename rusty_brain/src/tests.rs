// #[cfg(test)]
// mod tests {
//     use crate::board::Turn;
//     use crate::square::File;
//     use crate::{board::Board, square::{Rank, Square}};

//     #[test]
//     fn test_pawns() {
//         let mut board = Board::empty();
//         board.bitboards.white_pawns = 1;
//         for i in 0..64 {
//             let moves = board.pawn_moves();
//             let rank = Square::from(i).rank();

//             if rank == Rank::Second{
//                 assert_eq!(moves.len(), 2);
//             }else if rank == Rank::Eighth{
//                 assert_eq!(moves.len(), 0);
//             }else{
//                 assert_eq!(moves.len(), 1);
//             }

//             board.bitboards.white_pawns <<= 1;

//         }
        
//         board.bitboards.white_pawns = 0;

//         board.turn = Turn::Black;
//         board.bitboards.black_pawns = 1;

//         for i in 0..64 {
//             let moves = board.pawn_moves();
//             let rank = Square::from(i).rank();

//             if rank == Rank::Seventh{
//                 assert_eq!(moves.len(), 2);
//             }else if rank == Rank::First{
//                 assert_eq!(moves.len(), 0);
//             }else{
//                 assert_eq!(moves.len(), 1);
//             }

//             board.bitboards.black_pawns <<= 1;

//         }

//         // make sure no friendly captures are allowed
//         board.bitboards.black_pawns = u64::MAX;
        
//         let moves = board.pawn_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         board.bitboards.black_pawns = 0;

//         board.turn = Turn::White;

//         board.bitboards.white_pawns = u64::MAX;

//         let moves = board.pawn_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         // test en passant
//         board.turn = Turn::Black;
//         board.bitboards.white_pawns = 0x0800000000;
//         board.bitboards.black_pawns = 0x10000000000000;
//         board.make_move((Square::E7 as u8, Square::E5 as u8));

//         let moves = board.pawn_moves();
        
//         assert_eq!(moves.len(), 2);        
        
//         board.bitboards.black_pawns = 0x05000000;
//         board.bitboards.white_pawns = 0x0200;

//         board.make_move((Square::B2 as u8, Square::B4 as u8));

//         let moves = board.pawn_moves();

//         assert_eq!(moves.len(), 4);
              
//     }
    
//     #[test]
//     fn test_knights() {
//         let mut board = Board::empty();
//         board.bitboards.white_knights = 1;

//         for i in 0..64 {
//             let square = Square::from(i);
//             let rank = square.rank();
//             let file = square.file();
//             let moves = board.knight_moves();
//             if square == Square::A1 || square == Square::H1 || square == Square::A8 || square == Square::H8 {
//                 assert_eq!(moves.len(), 2);
//             }else if square == Square::B1 || square == Square::G1 || square == Square::B8 || square == Square::G8 || square == Square::A2 || square == Square::A7 || square == Square::H2 || square == Square::H7 {
//                 assert_eq!(moves.len(), 3);
//             }else if rank == Rank::First || rank == Rank::Eighth {
//                 assert_eq!(moves.len(), 4);
//             }else if square == Square::B2 || square == Square::G2 || square == Square::B7 || square == Square::G7 || file == File::A || file == File::H {
//                 assert_eq!(moves.len(), 4);
//             }else if rank == Rank::Second || rank == Rank::Seventh || file == File::B || file == File::G {
//                 assert_eq!(moves.len(), 6);
//             }else{
//                 assert_eq!(moves.len(), 8);
//             }
            
//             board.bitboards.white_knights <<= 1;
//         }
        
//         board.bitboards.white_knights = 0;

//         board.turn = Turn::Black;
//         board.bitboards.black_knights = 1;

//         for i in 0..64 {
//             let square = Square::from(i);
//             let rank = square.rank();
//             let file = square.file();
//             let moves = board.knight_moves();
//             if square == Square::A1 || square == Square::H1 || square == Square::A8 || square == Square::H8 {
//                 assert_eq!(moves.len(), 2);
//             }else if square == Square::B1 || square == Square::G1 || square == Square::B8 || square == Square::G8 || square == Square::A2 || square == Square::A7 || square == Square::H2 || square == Square::H7 {
//                 assert_eq!(moves.len(), 3);
//             }else if rank == Rank::First || rank == Rank::Eighth {
//                 assert_eq!(moves.len(), 4);
//             }else if square == Square::B2 || square == Square::G2 || square == Square::B7 || square == Square::G7 || file == File::A || file == File::H {
//                 assert_eq!(moves.len(), 4);
//             }else if rank == Rank::Second || rank == Rank::Seventh || file == File::B || file == File::G {
//                 assert_eq!(moves.len(), 6);
//             }else{
//                 assert_eq!(moves.len(), 8);
//             }
            
//             board.bitboards.black_knights <<= 1;
//         }
        
//         // make sure no friendly captures are allowed
//         board.bitboards.black_knights = u64::MAX;
        
//         let moves = board.knight_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         board.bitboards.black_knights = 0;

//         board.turn = Turn::White;

//         board.bitboards.white_knights = u64::MAX;

//         let moves = board.knight_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         // white knight on dark squares and black knights on light squares
//         board.bitboards.white_knights = 0xAA55AA55AA55AA55;

//         board.bitboards.black_knights = 0x55AA55AA55AA55AA;
        
//         let moves = board.knight_moves();

//         assert_eq!(moves.len(), 168);

//         board.turn = Turn::Black;

//         let moves = board.knight_moves();

//         assert_eq!(moves.len(), 168);
//     }

//     #[test]
// //     fn test_bishops() {
// //         let mut board = Board::empty();
// //         board.bitboards.white_bishops = 1;

// //         for i in 0..64 {
// //             let moves = board.bishop_moves();
// //             let file = Square::from(i).file();
// //             let rank = Square::from(i).rank();
// //             if rank == Rank::First || rank == Rank::Eighth{
// //                 assert_eq!(moves.len(), 7);
// //             }else if file == File::A || file == File::H {
// //                 assert_eq!(moves.len(), 7);
// //             }else if rank == Rank::Second || rank == Rank::Seventh{
// //                 assert_eq!(moves.len(), 9);
// //             }else if file == File::B || file == File::G {
// //                 assert_eq!(moves.len(), 9);
// //             }else if file == File::C || file == File::F{
// //                 assert_eq!(moves.len(), 11);
// //             }else if rank == Rank::Third || rank == Rank::Sixth{
// //                 assert_eq!(moves.len(), 11);
// //             }else if file == File::D || file == File::E {
// //                 assert_eq!(moves.len(), 13);
// //             }
// //             board.bitboards.white_bishops <<= 1;
// //         }
// //         board.bitboards.white_bishops = 0;

// //         board.turn = Turn::Black;
// //         board.bitboards.black_bishops = 1;

// //         for i in 0..64 {
// //             let moves = board.bishop_moves();
// //             let file = Square::from(i).file();
// //             let rank = Square::from(i).rank();
// //             if rank == Rank::First || rank == Rank::Eighth{
// //                 assert_eq!(moves.len(), 7);
// //             }else if file == File::A || file == File::H {
// //                 assert_eq!(moves.len(), 7);
// //             }else if rank == Rank::Second || rank == Rank::Seventh{
// //                 assert_eq!(moves.len(), 9);
// //             }else if file == File::B || file == File::G {
// //                 assert_eq!(moves.len(), 9);
// //             }else if file == File::C || file == File::F{
// //                 assert_eq!(moves.len(), 11);
// //             }else if rank == Rank::Third || rank == Rank::Sixth{
// //                 assert_eq!(moves.len(), 11);
// //             }else if file == File::D || file == File::E {
// //                 assert_eq!(moves.len(), 13);
// //             }
// //             board.bitboards.black_bishops <<= 1;
// //         }

// //         // make sure no friendly captures are allowed
// //         board.bitboards.black_bishops = u64::MAX;
        
// //         let moves = board.bishop_moves();
        
// //         assert_eq!(moves.len(), 0);
        
// //         board.bitboards.black_bishops = 0;

// //         board.turn = Turn::White;

// //         board.bitboards.white_bishops = u64::MAX;

// //         let moves = board.bishop_moves();
        
// //         assert_eq!(moves.len(), 0);
        
// //         // test captures 
// //         board.bitboards.white_bishops = 1;
// //         board.bitboards.black_bishops = 0x302;
        
// //         let moves = board.bishop_moves();

// //         assert_eq!(moves.len(), 1);
        
// // }

//     #[test]
//     fn test_rooks() {
//         let mut board = Board::empty();
//         board.bitboards.white_rooks = 1;

//         for _ in 0..64 {
//             let moves = board.rook_moves();

//             assert_eq!(moves.len(), 14);
            
//             board.bitboards.white_rooks <<= 1;
//         }
        
//         board.bitboards.white_rooks = 0;

//         board.turn = Turn::Black;
//         board.bitboards.black_rooks = 1;
        
//         for _ in 0..64 {
//             let moves = board.rook_moves();

//             assert_eq!(moves.len(), 14);
            
//            board.bitboards.black_rooks <<= 1;
//         }

//         // make sure no friendly captures are allowed
//         board.bitboards.black_rooks = u64::MAX;
        
//         let moves = board.rook_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         board.bitboards.black_rooks = 0;

//         board.turn = Turn::White;

//         board.bitboards.white_rooks = u64::MAX;

//         let moves = board.rook_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         // test captures 
//         board.bitboards.white_rooks = 1;
//         board.bitboards.black_rooks = 0x302;
        
//         let moves = board.rook_moves();

//         assert_eq!(moves.len(), 2);
//     }

//     #[test]
//     fn test_queens() {
//         let mut board = Board::empty();
//         board.bitboards.white_queens = 1;

//         for i in 0..64 {
//             let moves = board.queen_moves();
//             let file = Square::from(i).file();
//             let rank = Square::from(i).rank();
//             if rank == Rank::First || rank == Rank::Eighth{
//                 assert_eq!(moves.len(), 7 + 14);
//             }else if file == File::A || file == File::H {
//                 assert_eq!(moves.len(), 7 + 14);
//             }else if rank == Rank::Second || rank == Rank::Seventh{
//                 assert_eq!(moves.len(), 9 + 14);
//             }else if file == File::B || file == File::G {
//                 assert_eq!(moves.len(), 9 + 14);
//             }else if file == File::C || file == File::F{
//                 assert_eq!(moves.len(), 11 + 14);
//             }else if rank == Rank::Third || rank == Rank::Sixth{
//                 assert_eq!(moves.len(), 11 + 14);
//             }else if file == File::D || file == File::E {
//                 assert_eq!(moves.len(), 13 + 14);
//             }
//             board.bitboards.white_queens <<= 1;
//         }
//         board.bitboards.white_queens = 0;

//         board.turn = Turn::Black;
//         board.bitboards.black_queens = 1;


//         for i in 0..64 {
//             let moves = board.queen_moves();
//             let file = Square::from(i).file();
//             let rank = Square::from(i).rank();
//             if rank == Rank::First || rank == Rank::Eighth{
//                 assert_eq!(moves.len(), 7 + 14);
//             }else if file == File::A || file == File::H {
//                 assert_eq!(moves.len(), 7 + 14);
//             }else if rank == Rank::Second || rank == Rank::Seventh{
//                 assert_eq!(moves.len(), 9 + 14);
//             }else if file == File::B || file == File::G {
//                 assert_eq!(moves.len(), 9 + 14);
//             }else if file == File::C || file == File::F{
//                 assert_eq!(moves.len(), 11 + 14);
//             }else if rank == Rank::Third || rank == Rank::Sixth{
//                 assert_eq!(moves.len(), 11 + 14);
//             }else if file == File::D || file == File::E {
//                 assert_eq!(moves.len(), 13 + 14);
//             }
//             board.bitboards.black_queens <<= 1;
//         }

//         // make sure no friendly captures are allowed
//         board.bitboards.black_queens = u64::MAX;
        
//         let moves = board.queen_moves();
        
//         assert_eq!(moves.len(), 0);
        
//         board.bitboards.black_queens = 0;

//         board.turn = Turn::White;

//         board.bitboards.white_queens = u64::MAX;

//         let moves = board.queen_moves();
        
//         assert_eq!(moves.len(), 0);

//         // test captures
//         board.bitboards.white_queens = 1;
//         board.bitboards.black_queens = 0x302;
        
//         let moves = board.queen_moves();

//         assert_eq!(moves.len(), 3);
        
//     }
    
//     #[test]
//     fn test_king() {
//         let mut board = Board::empty();
//         board.bitboards.white_king = 1;

//         for i in 0..64 {
//             let moves = board.king_moves();
//             if i == 0 || i == 7 || i == 56 || i == 63 {
//                 assert_eq!(moves.len(), 3);
//             }else if i < 8 || i > 55{
//                 assert_eq!(moves.len(), 5);
//             }else if Square::from(i).file() == File::A || Square::from(i).file() == File::H {
//                 assert_eq!(moves.len(), 5);
//             }else{
//                 assert_eq!(moves.len(), 8);
//             } 
            
//             board.bitboards.white_king <<= 1;
//         }
        
//         board.bitboards.white_king = 0;

//         board.turn = Turn::Black;
//         board.bitboards.black_king = 1;

//         for i in 0..64 {
//             let moves = board.king_moves();
//             if i == 0 || i == 7 || i == 56 || i == 63 {
//                 assert_eq!(moves.len(), 3);
//             }else if i < 8 || i > 55{
//                 assert_eq!(moves.len(), 5);
//             }else if Square::from(i).file() == File::A || Square::from(i).file() == File::H {
//                 assert_eq!(moves.len(), 5);
//             }else{
//                 assert_eq!(moves.len(), 8);
//             } 
            
//             board.bitboards.black_king <<= 1;
//         }
        
//         board.bitboards.black_king = 0;
//         board.turn = Turn::White;
        
//         // test captures
//         board.bitboards.white_king = 1;
//         board.bitboards.black_knights = 0x302;
        
//         let moves = board.king_moves();

//         assert_eq!(moves.len(), 3);
        
//         // test castling
//         let mut board = Board::new();
//         board.bitboards.white_bishops = 0;
//         board.bitboards.white_knights = 0;
//         board.bitboards.white_queens = 0;
//         board.bitboards.black_bishops = 0;
//         board.bitboards.black_knights = 0;
//         board.bitboards.black_queens = 0;
        

//         let moves = board.king_moves();

//         assert_eq!(moves.len(), 4);
        
//         board.make_move((Square::E1 as u8, Square::G1 as u8)); 
        
//         assert_eq!(board.bitboards.white_king.trailing_zeros() as u8, Square::G1 as u8);
//         assert_eq!((board.bitboards.white_rooks & (1 << Square::F1 as u8)).trailing_zeros() as u8, Square::F1 as u8);
        
//         let moves = board.king_moves();

//         assert_eq!(moves.len(), 4);

//         board.make_move((Square::E8 as u8, Square::C8 as u8));
        
//         assert_eq!(board.bitboards.black_king.trailing_zeros() as u8, Square::C8 as u8);
//         assert_eq!((board.bitboards.black_rooks & (1 << Square::D8 as u8)).trailing_zeros() as u8, Square::D8 as u8);
        
//         let moves = board.king_moves();
        
//         assert_eq!(board.castling_rights.white_king_side, false);

//         assert_eq!(moves.len(), 1);

//         board.make_move(*moves.last().unwrap());
        
//         let moves = board.king_moves();
        
//         assert_eq!(board.castling_rights.black_queen_side, false);

//         assert_eq!(moves.len(), 1);

//         board.make_move(*moves.last().unwrap());
        

//     }
// }