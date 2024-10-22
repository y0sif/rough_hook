use crate::{board::Board, castling, movement::Move};

pub fn perft(board: &mut Board, depth: i32, captures: &mut i32, ep_captures: &mut i32, checks: &mut i32, checkmates: &mut i32, castling: &mut i32, promotion: &mut i32) -> usize {
    let mut nodes = 0;

    let moves = board.generate_legal_moves();

    if board.checkmate{
        *checkmates += 1;
    }
    if board.check {
        *checks += 1;
    }

    if depth == 0 {
        return 1;
    }

    for _move in &moves {
        match _move.get_flags() {
            Move::CAPTURE => {*captures += 1},
            Move::EP_CAPTURE => {
                *ep_captures += 1;
            },
            Move::KING_CASTLE | Move::QUEEN_CASTLE => {*castling += 1},
            Move::BISHOP_PROMOTION | Move::QUEEN_PROMOTION | Move::ROOK_PROMOTION | Move::KNIGHT_PROMOTION 
            => {*promotion += 1}
            Move::BISHOP_PROMO_CAPTURE | Move::ROOK_PROMO_CAPTURE | Move::QUEEN_PROMO_CAPTURE | Move::KNIGHT_PROMO_CAPTURE 
            => {
                *promotion += 1;
                *captures += 1;
            },
            _ => (),
        }
        board.make_move(*_move);
        let res = perft(board, depth-1, captures, ep_captures, checks, checkmates, castling, promotion);
        nodes += res;
        board.undo_move();
        
    }

    nodes
}

#[cfg(test)]
mod perft {
    use std::time::Instant;

    use crate::{board::{self, Board}, castling, perft::perft};

    // #[test]
    fn test_pefrt() {
        let mut board = Board::new();

        let mut captures = 0;
        let mut ep_captures = 0;
        let mut checksmates = 0;
        let mut checks = 0;
        let mut castling = 0;
        let mut promotions = 0;

        let res = perft(&mut board, 5, &mut captures, &mut ep_captures, &mut checks, &mut checksmates, &mut castling, &mut promotions);
        println!("cap {}, ep {}, checks {}, checkmates {}, castling {}, promotions {}", captures, ep_captures, checks, checksmates, castling, promotions);
        assert_eq!(res, 6);

        // print!("depth 2 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 2);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 400);

        // print!("depth 3 \t");
        // let now = Instant::now();
        // let mut captures = 0;
        // let mut ep_captures = 0;
        // let res = perft(&mut board, 3, &mut captures, &mut ep_captures);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // println!("captures {}, ep_captures {}", captures, ep_captures);
        // assert_eq!(res, 8902);

        // print!("depth 4 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 4);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 197281);

        // print!("depth 5 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 5);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 4865609);

        // print!("depth 6 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 6);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 119060324);

        // print!("depth 7 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 7);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 3195901860);

        // print!("depth 8 \t");
        // let now = Instant::now();
        // let res = perft(&mut board, 8);
        // println!("time: {} milliseconds", now.elapsed().as_millis());
        // assert_eq!(res, 84998978956);
    }
    

    #[test]
    fn perft_position_3() {
        let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string());

        let mut captures = 0;
        let mut ep_captures = 0;
        let mut checksmates = 0;
        let mut checks = 0;
        let mut castling = 0;
        let mut promotions = 0;

        let res = perft(&mut board, 2, &mut captures, &mut ep_captures, &mut checks, &mut checksmates, &mut castling, &mut promotions);
        println!("cap {}, ep {}, checks {}, checkmates {}, castling {}, promotions {}", captures, ep_captures, checks, checksmates, castling, promotions);
        assert_eq!(res, 6);
    }
}