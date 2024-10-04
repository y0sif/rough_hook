use std::io::empty;

use crate::bitboards::Bitboards;
use crate::helper::Masks;
#[derive(Clone, Copy)]
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
    
    pub fn generate_moves(&self) -> Vec<(u8, u8)> {
        // let mut moves = Vec::new();
        // let mut pawn_moves = self.pawn_moves();
        
        // moves.append(&mut pawn_moves);
        // moves

        // let mut moves = Vec::new();
        // let mut queen_moves = self.queen_moves();
        
        // moves.append(&mut queen_moves);
        // moves

        let mut moves = Vec::new();
        let mut rooks_moves = self.rook_moves();
        
        moves.append(&mut rooks_moves);
        moves

        //   let mut moves = Vec::new();
        //   let mut bishop_moves = self.bishop_moves();
        
        //   moves.append(&mut bishop_moves);
        //   moves
        
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
                let not_a_file = 0xfefefefefefefefe;
                let not_h_file = 0x7f7f7f7f7f7f7f7f;
                
                let enemy_pieces = self.bitboards.get_enemy_pieces(self.turn);

                let mut right_captures = (self.bitboards.white_pawns << 9) & not_a_file & enemy_pieces;
                let mut left_captures = (self.bitboards.white_pawns << 7) & not_h_file & enemy_pieces;
                
                while right_captures != 0 {
                    let end_square = right_captures.trailing_zeros() as u8;

                    moves.push((end_square - 9, end_square));
                    
                    right_captures &= right_captures - 1;
                }
                
                while left_captures != 0 {
                    let end_squares = left_captures.trailing_zeros() as u8;

                    moves.push((end_squares - 7, end_squares));     
                    
                    left_captures &= left_captures - 1;
                }
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
                let not_a_file = 0xfefefefefefefefe;
                let not_h_file = 0x7f7f7f7f7f7f7f7f;
                
                let enemy_pieces = self.bitboards.get_enemy_pieces(self.turn);

                let mut right_captures = (self.bitboards.black_pawns >> 7) & not_a_file & enemy_pieces;
                let mut left_captures = (self.bitboards.black_pawns >> 9) & not_h_file & enemy_pieces;
                
                while right_captures != 0 {
                    let end_square = right_captures.trailing_zeros() as u8;

                    moves.push((end_square + 7, end_square));
                    
                    right_captures &= right_captures - 1;
                }
                
                while left_captures != 0 {
                    let end_squares = left_captures.trailing_zeros() as u8;

                    moves.push((end_squares + 9, end_squares));     
                    
                    left_captures &= left_captures - 1;
                }
            }
        }
        moves
    }
    
    fn bishop_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let empty_positions= self.bitboards.get_empty_squares();
        let enemy_positions= self.bitboards.get_enemy_pieces(self.turn);
        let piece_position   = match self.turn {
            Turn::White => self.bitboards.white_bishops,
            Turn::Black=>self.bitboards.black_bishops
        };
        
        // Since it is impossible for two bishops to occupy the same color square (one must be on a light square and the other on a dark square),
        // we can leverage masks to isolate the bitboards for the light-square bishop and the dark-square bishop.

        let light_square_bishop = piece_position & Masks::LIGHT_SQUARE_MASK;
        // Get the dark-square bishop bit board
        let dark_square_bishop = piece_position & Masks::DARK_SQUARE_MASK;

        Self::get_bishop_moves(&mut moves, light_square_bishop,empty_positions, enemy_positions);
        Self::get_bishop_moves(&mut moves, dark_square_bishop,empty_positions, enemy_positions);

        return moves;
    }
    // Get the the bit board of all valid positions for a bishop  based on its movement directions
    // And fill the moves vector with the start and end squares for each move
    fn get_bishop_moves(moves : &mut Vec<(u8,u8)> , piece_positions : u64 , empty_positions : u64, enemy_positions : u64)
    {
        let mut valid_positions:u64 = Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_north_east)
                                     |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_north_west)
                                     |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions ,Bitboards::move_south_east)
                                     |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_south_west);

        let start_square = piece_positions.trailing_zeros() as u8;    
        Self::construct_moves_squares(moves, start_square, &mut valid_positions);                         
    }

    
    fn knight_moves(&self) -> Vec<(u8, u8)> {
        todo!()     
    }
    
    fn rook_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let empty_positions= self.bitboards.get_empty_squares();
        let enemy_positions= self.bitboards.get_enemy_pieces(self.turn);
        let piece_position   = match self.turn {
            Turn::White => self.bitboards.white_rooks,
            Turn::Black => self.bitboards.black_rooks
        };

        // We cannot apply the same masking technique as we did for the bishops, 
        // as both pieces can occupy squares of the same color. 
        // Therefore, we will use a different method to isolate the bitboards for the rooks. 
        // 
        // Steps of this method:
        //   1. Isolate the least significant set bit (rightmost 1) in `piece_position`.
        //   2. Clear the least significant set bit in `piece_position`.
        // 
        // Example:
        // If the rook's position is represented as 
        // 0000000000000000000001000000000000000000000000100000000000000000, 
        // then:
        //   rook1 = 0000000000000000000000000000000000000000000000100000000000000000
        //   rook2 = 0000000000000000000001000000000000000000000000000000000000000000
        //
        // Note: This technique is designed to handle boards that contain only one rook.
        match piece_position {
            0 => return moves,
            _ => {
                let rook1 = piece_position & (!piece_position + 1); 
                let rook2 = piece_position & (piece_position - 1);
                Self::get_rook_moves(&mut moves, rook1,empty_positions, enemy_positions);
                Self::get_rook_moves(&mut moves, rook2,empty_positions, enemy_positions);
                return moves
            }
        }
    }
    // Get the the bit board of all valid positions for a rook based on its movement directions
    // And fill the moves vector with the start and end squares for each move
    fn get_rook_moves(moves : &mut Vec<(u8,u8)> , piece_positions : u64 , empty_positions : u64, enemy_positions : u64)
    {
        let mut valid_positions :u64 = Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_north)
                                      |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_south)
                                      |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions ,Bitboards::move_east)
                                      |Self::get_sliding_positions(piece_positions, !empty_positions,enemy_positions, Bitboards::move_west);

        let start_square = piece_positions.trailing_zeros() as u8;
        Self::construct_moves_squares(moves, start_square, &mut valid_positions);
    }

    fn queen_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let empty_positions= self.bitboards.get_empty_squares();
        let enemy_positions= self.bitboards.get_enemy_pieces(self.turn);
        let piece_position   = match self.turn {
            Turn::White => self.bitboards.white_queens,
            Turn::Black => self.bitboards.black_queens
        };
        // The queen moves is Combination of bishop and rook moves
        Self::get_bishop_moves(&mut moves, piece_position, empty_positions, enemy_positions);
        Self::get_rook_moves(&mut moves, piece_position, empty_positions, enemy_positions);
        return moves;
    }
    
    fn king_moves(&self) -> Vec<(u8, u8)> {

        let mut moves: Vec<(u8, u8)> = Vec::new();
        let empty_squares = self.bitboards.get_empty_squares();

        match self.turn {
            Turn::White => {
                let king_square = self.bitboards.white_king.trailing_zeros() as u8;
                let mut kingset = self.bitboards.white_king;
                
                let mut attacks = Bitboards::move_east(kingset) | Bitboards::move_west(kingset);
                kingset |= attacks;
                attacks |= Bitboards::move_north(kingset) | Bitboards::move_south(kingset);

                attacks &= empty_squares;

                while attacks != 0 {
                    let end_square = attacks.trailing_zeros() as u8;

                    if end_square == king_square {
                        attacks &= attacks - 1;
                        continue;
                    }

                    moves.push((king_square, end_square));
                    attacks &= attacks - 1;
                }

            },
            Turn::Black => {
                let mut kingset = self.bitboards.black_king;
                let king_square = self.bitboards.black_king.trailing_zeros() as u8;

                let mut attacks = Bitboards::move_east(kingset) | Bitboards::move_west(kingset);
                kingset |= attacks;
                attacks |= Bitboards::move_north(kingset) | Bitboards::move_south(kingset);

                attacks &= empty_squares;

                while attacks != 0 {
                    let end_square = attacks.trailing_zeros() as u8;

                    if end_square == king_square as u8 {
                        continue;
                    }
                    
                    moves.push((king_square as u8, end_square));
                    attacks &= attacks - 1;
                }
            }
        }

        moves
    }

    // get the bit board of valid positions that the piece can move to (in specific direction)
    // the move_fn is a function that determines movement direction 
    fn get_sliding_positions(current_position: u64, occupied_positions: u64,enemy_positions : u64 , move_fn: fn(u64) -> u64) -> u64 {
        let mut positions = 0;             
        let mut next_position = move_fn(current_position); 
    
        // move until you hit a piece or the edge of the board
        while next_position != 0 && (next_position & occupied_positions) == 0 {
            positions |= next_position;               // Add the current square to the list of possible positions
            next_position = move_fn(next_position);   // Keep moving in the same direction
        }
        if next_position&enemy_positions != 0 {   // If the next square is occupied by an enemy piece, add it to the list of possible positions
            positions |= next_position;
        }
        positions
    }
    // construct start square and end square of the each move using the valid_positions bit board
    fn construct_moves_squares(moves : &mut Vec<(u8,u8)>  , start_square : u8 , valid_positions : &mut u64){
        while *valid_positions != 0 {
            let end_squares = valid_positions.trailing_zeros() as u8;
            moves.push((start_square  , end_squares));        
            *valid_positions &= *valid_positions - 1;
        }
    }
}
