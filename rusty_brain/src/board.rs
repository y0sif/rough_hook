use crate::bitboards::Bitboards;

enum Turn {
   White,
   Black, 
}
pub struct Board{
    bitboards: Bitboards,
    is_turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        Board{
            bitboards: Bitboards::new(),
            is_turn: Turn::White,
        }
    }
    
    pub fn make_move(&self) {
        todo!()
    }
    
    fn pawn_moves(&self) -> Vec<(u8, u8)> {
        todo!()
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
