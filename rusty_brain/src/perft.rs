// use burn::prelude::Backend;

// use crate::{board::Board, movement::Move};

// pub fn perft<B: Backend>(board: &mut Board<B>, depth: i32, captures: &mut i32, ep_captures: &mut i32, checks: &mut i32, checkmates: &mut i32, castling: &mut i32, promotion: &mut i32, double_checks: &mut i32) -> usize {
//     let mut nodes = 0;

//     let moves = board.generate_legal_moves();

//     if board.checkmate{
//         *checkmates += 1;
//     }

//     if depth == 0 {
//         return 1;
//     }

//     for _move in &moves {
//         match _move.get_flags() {
//             Move::CAPTURE => {*captures += 1;},
//             Move::EP_CAPTURE => {
//                 *ep_captures += 1;
//                 *captures += 1; 
//             },
//             Move::KING_CASTLE | Move::QUEEN_CASTLE => {*castling += 1},
//             Move::BISHOP_PROMOTION | Move::QUEEN_PROMOTION | Move::ROOK_PROMOTION | Move::KNIGHT_PROMOTION 
//             => {*promotion += 1}
//             Move::BISHOP_PROMO_CAPTURE | Move::ROOK_PROMO_CAPTURE | Move::QUEEN_PROMO_CAPTURE | Move::KNIGHT_PROMO_CAPTURE 
//             => {
//                 *promotion += 1;
//                 *captures += 1;
//             },
//             _ => (),
//         }
//         board.make_move(*_move);
//         let res = perft(board, depth-1, captures, ep_captures, checks, checkmates, castling, promotion, double_checks);
//         if depth == 4 {
//             println!("{}: {}", _move, res);
//         }
//         nodes += res;
//         board.undo_move();
        
//     }

//     nodes
// }

// pub fn perft_bulk<B: Backend>(board: &mut Board<B>, depth: i32, start: i32) -> usize {
//     let moves = board.generate_legal_moves();

//     if depth == 1 {
//         return moves.len();
//     }

//     let mut nodes = 0;

//     for _move in moves {
//         board.make_move(_move);
//         let res = perft_bulk(board, depth - 1, start);
//         if depth == start {
//             println!("{}: {}", _move, res);
//         }
//         nodes += res;
//         board.undo_move();
//     }
    
//     nodes
// }

// #[cfg(test)]
// mod perft {
//     use std::time::Instant;

//     use crate::board::Board;

//     use super::perft_bulk;

//     #[test]
//     fn test_pefrt() {
//         let depth_node_vec = [20, 400, 8902, 197281, 4865609, 119060324];
        
//         let mut board = Board::new();

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }
    

//     #[test]
//     fn perft_position_2() {
//         let depth_node_vec = [48, 2039, 97862, 4085603, 193690690, 8031647685];
        
//         let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }

//     #[test]
//     fn perft_position_3() {
//         let depth_node_vec = [14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393];
        
//         let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }

//     #[test]
//     fn perft_position_4() {
//         let depth_node_vec = [6, 264, 9467, 422333, 15833292, 706045033];
        
//         let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }

//         let depth_node_vec = [6, 264, 9467, 422333, 15833292, 706045033];
        
//         let mut board = Board::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }

//     #[test]
//     fn perft_position_5() {
//         let depth_node_vec = [44, 1486, 62379, 2103487, 89941194];
        
//         let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }

//     #[test]
//     fn perft_position_6() {
//         let depth_node_vec = [46, 2079, 89890, 3894594, 164075551];
        
//         let mut board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string());

//         for depth in 0..depth_node_vec.len() as i32{
//             println!("depth: {}", depth + 1);
//             let time = Instant::now();
//             let res = perft_bulk(&mut board, depth + 1, depth + 1);
//             assert_eq!(res, depth_node_vec[depth as usize]);
//             println!("elapsed time: {} ms", time.elapsed().as_millis());
//         }
//     }
// }