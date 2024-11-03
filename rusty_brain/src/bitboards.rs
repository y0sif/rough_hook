use std::hash::{DefaultHasher, Hash, Hasher};
use std::u64;
use std::collections::HashMap;
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
    
    pub fn from_fen(fen: &str) -> Self {
        let mut map = HashMap::from([
            ('P', 0u64),
            ('B', 0u64),
            ('N', 0u64),
            ('R', 0u64),
            ('Q', 0u64),
            ('K', 0u64),
            ('p', 0u64),
            ('b', 0u64),
            ('n', 0u64),
            ('r', 0u64),
            ('q', 0u64),
            ('k', 0u64),
        ]);
        
        let mut counter = 56;
        for piece in fen.chars() {
            if piece == '/' {
                counter -= 16;
                continue;
            }
            
            if piece.is_numeric() {
                counter += piece.to_digit(10).unwrap();
                continue;
            }

            if let Some(bitboard) = map.get_mut(&piece) {
                *bitboard |= 1 << counter;
            }
            counter += 1;            
        }

        Bitboards {
            white_pawns: *map.get(&'P').unwrap(),
            white_bishops: *map.get(&'B').unwrap(),
            white_knights: *map.get(&'N').unwrap(),
            white_rooks: *map.get(&'R').unwrap(),
            white_queens: *map.get(&'Q').unwrap(),
            white_king: *map.get(&'K').unwrap(),

            black_pawns: *map.get(&'p').unwrap(),
            black_bishops: *map.get(&'b').unwrap(),
            black_knights: *map.get(&'n').unwrap(),
            black_rooks: *map.get(&'r').unwrap(),
            black_queens: *map.get(&'q').unwrap(),
            black_king: *map.get(&'k').unwrap(),
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

    pub fn hash_board(&self) -> u64{
        let mut hasher = DefaultHasher::new();
        self.white_bishops.hash(&mut hasher);
        self.white_king.hash(&mut hasher);
        self.white_knights.hash(&mut hasher);
        self.white_pawns.hash(&mut hasher);
        self.white_queens.hash(&mut hasher);
        self.white_rooks.hash(&mut hasher);

        self.black_bishops.hash(&mut hasher);
        self.black_king.hash(&mut hasher);
        self.black_knights.hash(&mut hasher);
        self.black_pawns.hash(&mut hasher);
        self.black_queens.hash(&mut hasher);
        self.black_rooks.hash(&mut hasher);
       
        hasher.finish()
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
    
    pub fn rank_mask_to_end(square: u8) -> u64 {
        0xFF << ((square / 8) * 8)
    }

    pub fn rank_mask(square: u8) -> u64 {
        0x7E  << (square & 56)
    }

    pub fn file_mask(square: u8) -> u64 {
        0x0001010101010100  << (square & 7)
    }
    
    pub fn rank_mask_to_end_ex(square: u8) -> u64 {
        (1 << square) ^ (0xFF << (square & 56))
    }
    
    pub fn file_mask_to_end_ex(square: u8) -> u64 {
        (1 << square) ^ (0x0101010101010101 << (square & 7))
    }
    pub fn file_mask_to_end(square: u8) -> u64 {
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
        (Bitboards::rank_mask(square) ^ Bitboards::file_mask(square)) & !(1 << square) 
    }
    
    pub fn bishop_mask_ex(square: u8) -> u64 {
        (Bitboards::diagonal_mask(square) ^ Bitboards::anti_diagonal_mask(square)) & 0x7E7E7E7E7E7E00
    }
    
    pub fn queen_mask(square: u8) -> u64 {
        Bitboards::rook_mask(square) | Bitboards::bishop_mask(square)
    }
    
    pub fn queen_mask_ex(square: u8) -> u64 {
        Bitboards::rook_mask(square) ^ Bitboards::bishop_mask(square)
    }
    pub fn same_rank(pos1: u8, pos2: u8) -> bool {
        // Same rank if division by 8 gives the same result
        pos1 / 8 == pos2 / 8
    }
    
    pub fn same_file(pos1: u8, pos2: u8) -> bool {
        // Same file if remainder when divided by 8 gives the same result
        pos1 % 8 == pos2 % 8
    }
    
    pub fn same_diagonal(pos1: u8, pos2: u8) -> bool {
        // Same diagonal if (row - col) is the same
        (pos1 / 8) as i8 - (pos1 % 8) as i8 == (pos2 / 8) as i8 - (pos2 % 8) as i8
    }
    
    pub fn same_anti_diagonal(pos1: u8, pos2: u8) -> bool {
        // Same anti-diagonal if (row + col) is the same
        (pos1 / 8) as i8 + (pos1 % 8) as i8 == (pos2 / 8) as i8 + (pos2 % 8) as i8
    }
    pub fn get_lsb(number : &u64)->u8{
        let lsb_index = number.trailing_zeros() as u8; // Counts zeros from the right
        lsb_index
    }
    pub fn get_msp(number : &u64)->u8{
        let msb_index = 63 - number.leading_zeros(); // Leading zeros from the left
        return msb_index as u8;
    }
    pub fn get_right_mask(bitboard : u64 , square  :u8)->u64{
        bitboard & !((1 << (square + 1)) - 1)
    }
    pub fn get_left_mask(sqaure : u8)->u64{
        (1 << (sqaure + 1)) - 1
    }

}