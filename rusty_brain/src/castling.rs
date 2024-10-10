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
    fn check_king_castle(&self, turn : Turn) -> bool{
        match turn{
            Turn::White => self.white_king_side,
            Turn::Black => self.black_king_side
        }
    }
    fn check_queen_castle(&self, turn : Turn) -> bool{
        match turn{
            Turn::White => self.white_queen_castle,
            Turn::Black => self.black_queen_castle,
        }
    }
}