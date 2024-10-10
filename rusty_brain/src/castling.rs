use crate::board::Turn;

pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}
impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    pub fn empty() -> Self {
        CastlingRights {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

    pub fn check_king_side(&self, turn: Turn) -> bool{
        match turn{
            Turn::White => self.white_king_side,
            Turn::Black => self.black_king_side
        }
    }

    pub fn check_queen_side(&self, turn: Turn) -> bool{
        match turn{
            Turn::White => self.white_queen_side,
            Turn::Black => self.black_queen_side,
        }
    }
    
    pub fn reset_rights(&mut self, turn: Turn) {
        match turn {
            Turn::White => {
                self.white_king_side = false;
                self.white_queen_side = false;
            }, 
            Turn::Black => {
                self.black_king_side = false;
                self.black_queen_side = false;
            }
        }
    }
    
    pub fn reset_king_side_rights(&mut self, turn: Turn) {
        match turn {
            Turn::White => self.white_king_side = false,
            Turn::Black => self.black_king_side = false
        }
    }

    pub fn reset_queen_side_rights(&mut self, turn: Turn) {
        match turn {
            Turn::White => self.white_queen_side = false,
            Turn::Black => self.black_queen_side = false
        }
    }
}