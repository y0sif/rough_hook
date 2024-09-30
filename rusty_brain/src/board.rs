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
}
