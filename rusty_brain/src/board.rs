use std::io::empty;

use crate::bitboards::Bitboards;

pub enum Turn {
   White,
   Black, 
}
pub struct Board{
    bitboards: Bitboards,
    turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        Board{
            bitboards: Bitboards::new(),
            turn: Turn::White,
        }
    }
    
    pub fn make_move(&self) {
        todo!()
    }
    
    fn pawn_moves(&self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();
        match self.turn {
            Turn::White => {
                // pawn push
                let empty_squares = self.bitboards.get_empty_squares();
                let mut single_push = (self.bitboards.white_pawns << 8) & empty_squares;

                let rank4 = 0x00000000FF000000;
                let mut double_push = (single_push << 8) & empty_squares & rank4; 
                
                while single_push != 0 {
                    let end_square = single_push.trailing_zeros() as u8;

                    moves.push((end_square - 8, end_square));

                    single_push &= single_push - 1;
                }
                
                while double_push != 0 {
                    let end_square = double_push.trailing_zeros() as u8;

                    moves.push((end_square - 16, end_square));
                    
                    double_push &= double_push - 1;
                }
                // pawn capture
            },
            Turn::Black => {
                // pawn push
                let empty_squares = self.bitboards.get_empty_squares();
                let mut single_push = (self.bitboards.black_pawns >> 8) & empty_squares;

                let rank5 = 0x000000FF00000000;
                let mut double_push = (single_push >> 8) & empty_squares & rank5; 
                
                while single_push != 0 {
                    let end_square = single_push.trailing_zeros() as u8;

                    moves.push((end_square + 8, end_square));

                    single_push &= single_push - 1;
                }
                
                while double_push != 0 {
                    let end_square = double_push.trailing_zeros() as u8;

                    moves.push((end_square + 16, end_square));
                    
                    double_push &= double_push - 1;
                }
                // pawn capture
            }
        }
        moves
    }
    
    fn bishop_move(&self) -> Vec<(u8, u8)> {
        todo!()
    }
    
    fn knight_moves(&self) -> Vec<(u8, u8)> {
        todo!()     
    }
    
    fn rook_moves(&self) -> Vec<(u8, u8)> {
        todo!()
    }
    
    fn queen_moves(&self) -> Vec<(u8, u8)> {
        todo!()     
    }
    
    fn king_moves(&self) -> Vec<(u8, u8)> {
        todo!()
    }
}
