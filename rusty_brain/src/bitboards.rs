use crate::board::Turn;
pub struct Bitboards{
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Bitboards {
    // initialize the board at the starting position
    pub fn new() -> Self {
        Bitboards{
            white_pawns: 0x000000000000FF00,
            white_bishops: 0x0000000000000024,
            white_knights: 0x0000000000000042,
            white_rooks: 0x0000000000000081,
            white_queens: 0x0000000000000008,
            white_king: 0x0000000000000010,

            black_pawns: 0x00FF000000000000,
            black_bishops: 0x2400000000000000,
            black_knights: 0x4200000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x0800000000000000,
            black_king: 0x1000000000000000,
        }
    }    

    pub fn get_ally_pieces(&self, turn: Turn) -> u64 {
        match turn{
            Turn::White => {
                self.white_bishops |
                self.white_king |
                self.white_knights |
                self.white_pawns |
                self.white_queens |
                self.white_rooks
            },
            Turn::Black => {
                self.black_bishops |
                self.black_king |
                self.black_knights |
                self.black_pawns |
                self.black_queens | 
                self.black_rooks 
            }
        }
    }
    
    pub fn get_enemy_pieces(&self, turn: Turn) -> u64 {
        match turn{
            Turn::Black => {
                self.white_bishops |
                self.white_king |
                self.white_knights |
                self.white_pawns |
                self.white_queens |
                self.white_rooks
            },
            Turn::White => {
                self.black_bishops |
                self.black_king |
                self.black_knights |
                self.black_pawns |
                self.black_queens | 
                self.black_rooks 
            }
        }
    }
    
    pub fn get_empty_squares(&self) -> u64 {
        !(self.get_ally_pieces(Turn::White) | self.get_enemy_pieces(Turn::White))
    }
    pub fn move_north(bitboard: u64) -> u64 {
        bitboard << 8
    }
    
    pub fn move_south(bitboard: u64) -> u64 {
        bitboard >> 8
    }
    
    pub fn move_east(bitboard: u64) -> u64 {
        (bitboard << 1) & 0xfefefefefefefefe  // return 0  if the move goes out of the board
    }
    
    pub fn move_west(bitboard: u64) -> u64 {
        (bitboard >> 1) & 0x7f7f7f7f7f7f7f7f  // return 0  if the move goes out of the board
    }
    
    pub fn move_north_east(bitboard: u64) -> u64 {
        (bitboard << 9) & 0xfefefefefefefe00 // return 0  if the move goes out of the board
    }
    
    pub fn move_north_west(bitboard: u64) -> u64 {
        (bitboard << 7) & 0x7f7f7f7f7f7f7f00 // return 0  if the move goes out of the board
    }
    
    pub fn move_south_east(bitboard: u64) -> u64 {
        (bitboard >> 7) & 0x00fefefefefefefe // return 0  if the move goes out of the board
    }
    
    pub fn move_south_west(bitboard: u64) -> u64 {
        (bitboard >> 9) & 0x007f7f7f7f7f7f7f // return 0  if the move goes out of the board
    }
}