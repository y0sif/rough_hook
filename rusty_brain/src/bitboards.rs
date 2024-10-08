use std::u64;

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

    pub fn empty() -> Self{
        Bitboards {
            white_pawns: 0,
            white_bishops: 0,
            white_knights: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,

            black_pawns: 0,
            black_bishops: 0,
            black_knights: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0 
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

    // sliding pieces masks
    pub fn east_mask_ex(square: u8) -> u64 {
        2 * ( (1 << (square|7)) - (1 << square))
    }
    
    pub fn north_mask_ex(square: u8) -> u64 {
        0x0101010101010100 << square
    }
    
    pub fn west_mask_ex(square: u8) -> u64 {
        (1 << square) - (1 << (square&56))
    }
    
    pub fn south_mask_ex(square: u8) -> u64 {
        0x0080808080808080 >> (square ^ 63)
    }
    
    pub fn rank_mask(square: u8) -> u64 {
        0xff << (square & 56)
    }
    
    pub fn file_mask(square: u8) -> u64 {
        0x0101010101010101 << (square & 7)
    }
    
    pub fn diagonal_mask(square: u8) -> u64 {
        let square = square as i32;
        let main_diagonal = 0x8040201008040201;
        let diagonal = 8*(square & 7) - (square & 56);
        let north = -diagonal & (diagonal >> 31);
        let south = diagonal & (-diagonal >> 31);
        (main_diagonal >> south) << north
    }
    
    pub fn anti_diagonal_mask(square: u8) -> u64 {
        let square = square as i32;
        let main_diagonal = 0x0102040810204080;
        let diagonal = 56 - 8*(square & 7) - (square & 56);
        let north = -diagonal & (diagonal >> 31);
        let south = diagonal & (-diagonal >> 31);
        (main_diagonal >> south) << north
    }
    
    pub fn rank_mask_ex(square: u8) -> u64 {
        (1 << square) ^ Bitboards::rank_mask(square)
    }
    
    pub fn file_mask_ex(square: u8) -> u64 {
        (1 << square) ^ Bitboards::file_mask(square)
    }

    pub fn diagonal_mask_ex(square: u8) -> u64 {
        (1 << square) ^ Bitboards::diagonal_mask(square)
    }

    pub fn anti_diagonal_mask_ex(square: u8) -> u64 {
        (1 << square) ^ Bitboards::anti_diagonal_mask(square)
    }
    
    pub fn rook_mask(square: u8) -> u64 {
        Bitboards::rank_mask(square) | Bitboards::file_mask(square)
    }
    
    pub fn bishop_mask(square: u8) -> u64 {
        Bitboards::diagonal_mask(square) | Bitboards::anti_diagonal_mask(square)
    }
    pub fn rook_mask_ex(square: u8) -> u64 {
        Bitboards::rank_mask(square) ^ Bitboards::file_mask(square)
    }
    
    pub fn bishop_mask_ex(square: u8) -> u64 {
        Bitboards::diagonal_mask(square) ^ Bitboards::anti_diagonal_mask(square)
    }
    
    pub fn queen_mask(square: u8) -> u64 {
        Bitboards::rook_mask(square) | Bitboards::bishop_mask(square)
    }
    
    pub fn queen_mask_ex(square: u8) -> u64 {
        Bitboards::rook_mask(square) ^ Bitboards::bishop_mask(square)
    }

}