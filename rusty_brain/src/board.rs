use crate::bitboards::Bitboards;
use crate::castling::CastlingRights;
use crate::magic::Magic;
use crate::square::{Rank, Square};
#[derive(Clone, Copy)]
pub enum Turn {
   White,
   Black, 
}
pub struct Board{
    pub bitboards: Bitboards,
    pub turn: Turn,
    pub rook_attacks: [Vec<u64>; 64],
    pub bishop_attacks: [Vec<u64>; 64],
    pub move_log: Vec<(u8, u8)>,
    pub is_en_passant: bool,
    pub castling_rights: CastlingRights,

}

impl Board {
    pub fn new() -> Self {
        let rook_attacks = Magic::piece_attacks(true);
        let bishop_attacks = Magic::piece_attacks(false);
        Board{
            bitboards: Bitboards::new(),
            turn: Turn::White,
            rook_attacks,
            bishop_attacks,
            move_log: Vec::new(),
            is_en_passant: false,
            castling_rights: CastlingRights::new(),
        }
    }
    
    pub fn empty() -> Self{
        let rook_attacks = Magic::piece_attacks(true);
        let bishop_attacks = Magic::piece_attacks(false);
        Board {
            bitboards: Bitboards::empty(),
            turn: Turn::White,
            rook_attacks,
            bishop_attacks,
            move_log: Vec::new(),
            is_en_passant: false, 
            castling_rights: CastlingRights::empty()
        }
    }
    
    pub fn from_fen(fen: String) -> Self {
        let rook_attacks = Magic::piece_attacks(true);
        let bishop_attacks = Magic::piece_attacks(false);
        let fen_vec: Vec<&str> = fen.split_whitespace().collect();
        
        let turn = match fen_vec[1] {
            "w" => Turn::White,
            _ => Turn::Black,
        };
        
        let mut castling_rights = CastlingRights::empty();
        
        for right in fen_vec[2].chars() {
            match right {
                'K' => castling_rights.white_king_side = true,
                'Q' => castling_rights.white_queen_side = true,
                'k' => castling_rights.black_king_side = true,
                'q' => castling_rights.black_queen_side = true,
                _ => ()
            }
        }
        
        let mut move_log = Vec::new();

        let is_en_passant = match fen_vec[3] {
            "-" => false,
            _ => {
                let square = Square::from(fen_vec[3]) as u8;
                let last_move = match turn {
                    Turn::White => {
                        let start_square = square + 8;
                        let end_square = square - 8;
                        (start_square, end_square)
                    },
                    Turn::Black => {
                        let start_square = square - 8;
                        let end_square = square + 8;
                        (start_square, end_square)                    
                    }
                };
                move_log.push(last_move);
                true
            }
        };
        
        // add fifty move rule later

        Board {
            bitboards: Bitboards::from_fen(fen_vec[0]),
            turn,
            rook_attacks,
            bishop_attacks,
            move_log,
            is_en_passant, 
            castling_rights
        }
    }
    
    pub fn make_move(&mut self, move_to_make: (u8, u8)) {
        let start_square = 1 << move_to_make.0;
        let end_square = 1 << move_to_make.1;
        self.make_capture(move_to_make);
        match self.turn {
            Turn::White => {
                if start_square & self.bitboards.white_pawns != 0 {
                    if self.is_en_passant {
                        self.make_en_passant(move_to_make);
                    }
                    self.bitboards.white_pawns &= !start_square;      
                    self.bitboards.white_pawns |= end_square;

                }else if start_square & self.bitboards.white_knights != 0 {
                    self.bitboards.white_knights &= !start_square;
                    self.bitboards.white_knights |= end_square;

                }else if start_square & self.bitboards.white_bishops != 0 {
                    self.bitboards.white_bishops &= !start_square;
                    self.bitboards.white_bishops |= end_square;

                }else if start_square & self.bitboards.white_rooks != 0 {
                    self.check_rook(move_to_make);
                    self.bitboards.white_rooks &= !start_square;
                    self.bitboards.white_rooks |= end_square;

                }else if start_square & self.bitboards.white_queens != 0 {
                    self.bitboards.white_queens &= !start_square;
                    self.bitboards.white_queens |= end_square;

                }else if start_square & self.bitboards.white_king != 0 {
                    self.make_castling_move(move_to_make);
                    self.castling_rights.reset_rights(self.turn);
                    self.bitboards.white_king &= !start_square;
                    self.bitboards.white_king |= end_square;

                }
                self.turn = Turn::Black;
            },
            Turn::Black => {
                if start_square & self.bitboards.black_pawns != 0 {
                    if self.is_en_passant {
                        self.make_en_passant(move_to_make);
                    }
                    self.bitboards.black_pawns &= !start_square;      
                    self.bitboards.black_pawns |= end_square;

                }else if start_square & self.bitboards.black_knights != 0 {
                    self.bitboards.black_knights &= !start_square;
                    self.bitboards.black_knights |= end_square;

                }else if start_square & self.bitboards.black_bishops != 0 {
                    self.bitboards.black_bishops &= !start_square;
                    self.bitboards.black_bishops |= end_square;

                }else if start_square & self.bitboards.black_rooks != 0 {
                    self.check_rook(move_to_make);
                    self.bitboards.black_rooks &= !start_square;
                    self.bitboards.black_rooks |= end_square;

                }else if start_square & self.bitboards.black_queens != 0 {
                    self.bitboards.black_queens &= !start_square;
                    self.bitboards.black_queens |= end_square;

                }else if start_square & self.bitboards.black_king != 0 {
                    self.make_castling_move(move_to_make);
                    self.castling_rights.reset_rights(self.turn);
                    self.bitboards.black_king &= !start_square;
                    self.bitboards.black_king |= end_square;

                }
                self.turn = Turn::White;
                
            }
        }
        self.move_log.push((move_to_make.0, move_to_make.1));
        self.is_en_passant = false;
        
    }
    
    fn make_en_passant(&mut self, move_to_make: (u8, u8)) {
        let last_move = self.move_log.last().unwrap();
        match self.turn {
            Turn::White => {
                if move_to_make.1 - 8 == last_move.1 {
                    let black_pawn = 1 << last_move.1;
                    self.bitboards.black_pawns &= !black_pawn;
                } 
            },
            Turn::Black => {
                if move_to_make.1 + 8 == last_move.1 {
                    let white_pawn = 1 << last_move.1;
                    self.bitboards.white_pawns &= !white_pawn;
                } 
            }
        }
    }
    
    fn make_capture(&mut self, move_to_make: (u8, u8)) {
        let square_captured = !(1 << move_to_make.1);
        
        match self.turn {
            Turn::White => {
                self.bitboards.black_bishops &= square_captured;
                self.bitboards.black_knights &= square_captured;
                self.bitboards.black_pawns &= square_captured;
                self.bitboards.black_queens &= square_captured;
                self.bitboards.black_rooks &= square_captured;
                self.check_captured_rook(move_to_make, self.bitboards.black_rooks);
            },
            Turn::Black => {
                self.bitboards.white_bishops &= square_captured;
                self.bitboards.white_knights &= square_captured;
                self.bitboards.white_pawns &= square_captured;
                self.bitboards.white_queens &= square_captured;
                self.bitboards.white_rooks &= square_captured;
                self.check_captured_rook(move_to_make, self.bitboards.white_rooks);
            }
        }
    }
    
    fn check_captured_rook(&mut self, move_to_make: (u8, u8), rook_bitboard: u64) {
        let square_captured = !(1 << move_to_make.1);
        let end_square = move_to_make.1;
        
        if square_captured & rook_bitboard != 0 {
            match Square::from(end_square) {
                Square::A1 => self.castling_rights.white_queen_side = false,
                Square::H1 => self.castling_rights.white_king_side = false,
                Square::A8 => self.castling_rights.black_queen_side = false,
                Square::H8 => self.castling_rights.black_king_side = false,
                _ => ()
            }
        }
        
    }

    fn check_rook(&mut self, move_to_make: (u8, u8)) {
        let rook_square = move_to_make.0;
        match self.turn {
            Turn::White => {
                if rook_square == Square::H1 as u8 {
                    self.castling_rights.reset_king_side_rights(self.turn);
                }else if rook_square == Square::A1 as u8{
                    self.castling_rights.reset_queen_side_rights(self.turn);
                }
            },
            Turn::Black => {
                if rook_square == Square::H8 as u8 {
                    self.castling_rights.reset_king_side_rights(self.turn);
                }else if rook_square == Square::A8 as u8{
                    self.castling_rights.reset_queen_side_rights(self.turn);
                }
            }
        }
    }

    fn make_castling_move(&mut self, move_to_make: (u8, u8)) {
        // king side
        if move_to_make.0 == move_to_make.1 - 2 {
            self.make_king_side_move();
        }else if move_to_make.0 == move_to_make.1 + 2 { // queen side
            self.make_queen_side_move();
        }
    }

    fn make_king_side_move(&mut self) {
        match self.turn {
            Turn::White => {
                self.bitboards.white_rooks &= !(1 << Square::H1 as u8);
                self.bitboards.white_rooks |= 1 << Square::F1 as u8;
            },
            Turn::Black => {
                self.bitboards.black_rooks &= !(1 << Square::H8 as u8);
                self.bitboards.black_rooks |= 1 << Square::F8 as u8;
            }
        }
    }

    fn make_queen_side_move(&mut self) {
        match self.turn {
            Turn::White => {
                self.bitboards.white_rooks &= !(1 << Square::A1 as u8);
                self.bitboards.white_rooks |= 1 << Square::D1 as u8;
            },
            Turn::Black => {
                self.bitboards.black_rooks &= !(1 << Square::A8 as u8);
                self.bitboards.black_rooks |= 1 << Square::D8 as u8;
            }
        }
    }
    
    pub fn generate_moves(&mut self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();

        let mut pawn_moves = self.pawn_moves();
        moves.append(&mut pawn_moves);

        let mut queen_moves = self.queen_moves();
        moves.append(&mut queen_moves);

        let mut rooks_moves = self.rook_moves();
        moves.append(&mut rooks_moves);

        let mut bishop_moves = self.bishop_moves();
        moves.append(&mut bishop_moves);

        let mut knight_moves = self.knight_moves();
        moves.append(&mut knight_moves);
        
        let mut king_moves = self.king_moves();
        moves.append(&mut king_moves);

        moves
    }
    
    pub fn pawn_moves(&mut self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new(); 

        let not_a_file : u64 = 0xfefefefefefefefe;
        let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;

        let empty_bitboard = self.bitboards.get_empty_squares();
        let enemy_bitboard = self.bitboards.get_enemy_pieces(self.turn);

        let (mut single_push_bitboard, mut double_push_bitboard, push_direction, right_capture_mask, left_capture_mask, mut right_captures_bitboard, mut left_captures_bitboard) =
        match self.turn {
            Turn::White => {
                let rank = 0x00000000FF000000;
                let single_push_bitboard = (self.bitboards.white_pawns << 8) & empty_bitboard; 
                (
                    single_push_bitboard,
                    (single_push_bitboard << 8) & empty_bitboard & rank,
                    -1,
                    9,
                    7,
                    (self.bitboards.white_pawns << 9) & not_a_file & enemy_bitboard,
                    (self.bitboards.white_pawns << 7) & not_h_file & enemy_bitboard          
                )
            },
            Turn::Black => {
                let rank = 0x000000FF00000000; 
                let single_push_bitboard = (self.bitboards.black_pawns >> 8) & empty_bitboard; 
                (
                    single_push_bitboard,
                    (single_push_bitboard >> 8) & empty_bitboard & rank,
                    1,
                    7,
                    9,
                    (self.bitboards.black_pawns >> 7) & not_a_file & enemy_bitboard,
                    (self.bitboards.black_pawns >> 9) & not_h_file & enemy_bitboard          
                )
            }
        };
        
        Self::get_push_moves(&mut moves ,  &mut single_push_bitboard , 1 , push_direction); 
        Self::get_push_moves(&mut moves ,  &mut double_push_bitboard , 2,  push_direction); 
        Self::get_capture_moves(&mut moves , &mut right_captures_bitboard , right_capture_mask , push_direction); 
        Self::get_capture_moves(&mut moves , &mut left_captures_bitboard  , left_capture_mask  , push_direction); 
        Self::check_en_passant(self, &mut moves);
        moves
    }

    fn get_push_moves(moves : &mut Vec<(u8, u8)> , push_bitboard  : & mut u64 ,  steps : i32 , push_direction : i32) {
        while *push_bitboard != 0
        {
            let end_square = push_bitboard.trailing_zeros() as i32 ;
            let start_square = end_square + (steps*8*push_direction);  
            moves.push((start_square as u8 , end_square as u8));            
            *push_bitboard &= *push_bitboard - 1;
        }
    }

    fn get_capture_moves(moves : &mut Vec<(u8, u8)> , capture_bitboard : &mut u64 , capture_mask : i32 , push_direction : i32) {
        while *capture_bitboard != 0 {
            let end_square = capture_bitboard.trailing_zeros() as i32;
            let start_square = end_square + (capture_mask*push_direction);

            moves.push((start_square as u8, end_square as u8));
            *capture_bitboard &= *capture_bitboard - 1;
        }
    }
    
    fn get_en_passant_moves(moves: &mut Vec<(u8, u8)>, capture_bitboard : &mut u64, end_square: u8) {
        while *capture_bitboard != 0 {
            let start_square = capture_bitboard.trailing_zeros() as u8;

            moves.push((start_square, end_square));

            *capture_bitboard &= *capture_bitboard - 1;
        }
    }

    pub fn check_en_passant(&mut self, moves: &mut Vec<(u8,u8)>){
        if let Some(last_move) = self.move_log.last() {
            let (pawn_bitboard, start_rank, end_rank) = match self.turn {
                Turn::White => (self.bitboards.black_pawns, Rank::Seventh, Rank::Fifth),
                Turn::Black => (self.bitboards.white_pawns, Rank::Second, Rank::Forth)
            };
            
            if (1 << last_move.1) & pawn_bitboard != 0 {
                if Square::from(last_move.0).rank() == start_rank && Square::from(last_move.1).rank() == end_rank {
                    let east_bitboard = Bitboards::move_east(1 << last_move.1);
                    let west_bitboard = Bitboards::move_west(1 << last_move.1);
                    let (pawns, end_square) = match self.turn {
                        Turn::White => (self.bitboards.white_pawns, last_move.1 + 8),
                        Turn::Black => (self.bitboards.black_pawns, last_move.1 - 8)
                    };
                    let mut ep_captures = pawns & (east_bitboard | west_bitboard);
                    
                    Self::get_en_passant_moves(moves, &mut ep_captures, end_square);

                    self.is_en_passant = true
                }
            }
        }
    }

    pub fn knight_moves(&self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();

        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);
        let piece_bitboard = match self.turn {
            Turn::White => self.bitboards.white_knights,
            Turn::Black => self.bitboards.black_knights,
        };
        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_knight_moves(&mut moves, piece_position, ally_bitboard);
        }
        moves 
    }

    fn get_knight_moves(moves: &mut Vec<(u8,u8)>, piece_position: u64, ally_bitboard: u64) {
        let not_ab_file = 0xFCFCFCFCFCFCFCFC;
        let not_a_file = 0xfefefefefefefefe;
        let not_gh_file = 0x3F3F3F3F3F3F3F3F;
        let not_h_file = 0x7f7f7f7f7f7f7f7f;

        let mut valid_bitboard = (piece_position << 17) & not_a_file & !ally_bitboard; //noNoEa
        valid_bitboard |= (piece_position << 10) & not_ab_file & !ally_bitboard; // noEaEa
        valid_bitboard |= (piece_position >> 6)  & not_ab_file & !ally_bitboard; // soEaEa
        valid_bitboard |= (piece_position >> 15) & not_a_file  & !ally_bitboard; //soSoEa
        valid_bitboard |= (piece_position << 15) & not_h_file  & !ally_bitboard; // noNoWe
        valid_bitboard |= (piece_position << 6)  & not_gh_file & !ally_bitboard; // noWeWe
        valid_bitboard |= (piece_position >> 10) & not_gh_file & !ally_bitboard; // soWeWe
        valid_bitboard |= (piece_position >> 17) & not_h_file  & !ally_bitboard; // soSoWe

        let start_square = piece_position.trailing_zeros() as u8;    
        Self::construct_moves_squares(moves, start_square, &mut valid_bitboard); 
    }
    
    pub fn bishop_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_bishops,
            Turn::Black=>self.bitboards.black_bishops
        };
        
        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_bishop_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard);
        }

        return moves;
    }

    fn get_bishop_moves(&self, moves: &mut Vec<(u8,u8)>, piece_position: u64, enemy_bitboard: u64, ally_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let bishop_mask = Bitboards::bishop_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;

        let blocker = all_pieces & bishop_mask; 
        let key = ((blocker & bishop_mask).wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

        let mut moves_bitboard = self.bishop_attacks[start_square as usize][key as usize];
        moves_bitboard &= !ally_bitboard;
        
        Self::construct_moves_squares(moves, start_square, &mut moves_bitboard); 
    }
    
    pub fn rook_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_rooks,
            Turn::Black => self.bitboards.black_rooks
        };

        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_rook_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard);
        }
        return  moves;
    }

    fn get_rook_moves(&self, moves: &mut Vec<(u8,u8)>, piece_position: u64 ,enemy_bitboard: u64, ally_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let rook_mask = Bitboards::rook_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;

        let blocker = all_pieces & rook_mask; 
        let key = ((blocker & rook_mask).wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];

        let mut moves_bitboard = self.rook_attacks[start_square as usize][key as usize];
        moves_bitboard &= !ally_bitboard;
        
        Self::construct_moves_squares(moves, start_square, &mut moves_bitboard); 
    }

    pub fn queen_moves(&self) -> Vec<(u8, u8)> {
        let mut moves : Vec<(u8 ,u8)> = Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_queens,
            Turn::Black => self.bitboards.black_queens
        };

        // The queen moves is Combination of bishop and rook moves
        if piece_bitboard != 0 {
            Self::get_bishop_moves(&self, &mut moves, piece_bitboard, enemy_bitboard, ally_bitboard);
            Self::get_rook_moves(&self, &mut moves, piece_bitboard, enemy_bitboard, ally_bitboard);
        }

        return moves;
    }

    // get the bit board of valid positions that the piece can move to (in specific direction)
    // the move_fn is a function that determines movement direction 
    pub fn get_sliding_bitboard(current_position: u64, occupied_bitboard: u64, enemy_bitboard: u64, move_fn: fn(u64) -> u64) -> u64 {
        let mut bitboard = 0;             
        let mut next_position = move_fn(current_position); 
    
        // move until you hit a piece or the edge of the board
        while next_position != 0 && (next_position & occupied_bitboard) == 0 {
            bitboard |= next_position;               // Add the current square to the list of possible positions
            next_position = move_fn(next_position);   // Keep moving in the same direction
        }
        if next_position&enemy_bitboard != 0 {   // If the next square is occupied by an enemy piece, add it to the list of possible positions
            bitboard |= next_position;
        }
        bitboard
    }
    
    pub fn king_moves(&self) -> Vec<(u8, u8)> {
        let mut moves: Vec<(u8, u8)> = Vec::new();
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);
        let piece_bitboard = match self.turn { 
            Turn::White => self.bitboards.white_king,
            Turn::Black => self.bitboards.black_king,
        };
        Self::get_king_moves(&mut moves, piece_bitboard, ally_bitboard);
        Self::get_castling_moves(self ,&mut moves ,&piece_bitboard);    
        moves
    }

    fn get_king_moves(moves: &mut Vec<(u8,u8)>, piece_position: u64, ally_bitboard: u64){
        let mut king_bitboard = piece_position;
        
        let mut valid_bitboard = Bitboards::move_east(king_bitboard) | Bitboards::move_west(king_bitboard);
        king_bitboard |= valid_bitboard;
        valid_bitboard |= Bitboards::move_north(king_bitboard) | Bitboards::move_south(king_bitboard);

        valid_bitboard &= !ally_bitboard;

        let start_square = piece_position.trailing_zeros() as u8;
        Self::construct_moves_squares(moves, start_square, &mut valid_bitboard);
    }

    pub fn get_castling_moves(&self, moves : &mut Vec<(u8,u8)> , king_position: &u64) {
        let occupied_bitboard = self.bitboards.get_ally_pieces(self.turn) | self.bitboards.get_enemy_pieces(self.turn);
        if self.castling_rights.check_king_side(self.turn){ //  true :  will be changes after the castling rights is done 
            Self::get_king_side_move(moves ,&king_position , &occupied_bitboard);
        }
        if self.castling_rights.check_queen_side(self.turn){
            Self::get_queen_side_move(moves ,&king_position , &occupied_bitboard);
        }
    }

    fn get_king_side_move(moves : &mut Vec<(u8,u8)>, king_position : &u64 , occupied_bitboard: &u64){
        let square_between = king_position <<1 | king_position << 2;
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle {
            let start_square = king_position.trailing_zeros() as u8;
            moves.push((start_square, start_square + 2));
        }
    }

    fn get_queen_side_move(moves : &mut Vec<(u8,u8)>, king_position : &u64 , occupied_bitboard: &u64){
        let square_between = king_position >>1 | king_position >> 2 | king_position >> 3;
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle {
            let start_square = king_position.trailing_zeros() as u8;
            moves.push((start_square, start_square - 2));
        }
    }

    // get the position for each piece from the piece bitboard
    // eg : if the piece bitboard is 0000000000000000000000100000000000000000000100000000000000000001
    //      so the positions will contain : 
    //        -- 0000000000000000000000000000000000000000000000000000000000000001
    //        -- 0000000000000000000000000000000000000000000100000000000000000000
    //        -- 0000000000000000000000100000000000000000000000000000000000000000
    fn get_piece_positions_from(piece_bitboard: &u64) -> Vec<u64> {
        let mut bitboard  =  *piece_bitboard; 
        let mut positions = Vec::new();
        while bitboard != 0 {
            let position = bitboard & (!bitboard + 1); 
            positions.push(position);
            bitboard &= bitboard - 1;
        }
        positions
    }
    
    fn construct_moves_squares(moves : &mut Vec<(u8,u8)>  , start_square : u8 , valid_bitboard : &mut u64){
        while *valid_bitboard != 0 {
            let end_squares = valid_bitboard.trailing_zeros() as u8;
            moves.push((start_square  , end_squares));        
            *valid_bitboard &= *valid_bitboard - 1;
        }
    }

    pub fn print_board(&mut self) {

        println!("\nWhite:♚ - Black:♔\n");

        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square_index = rank * 8 + file;

                if self.bitboards.white_pawns & (1 << square_index) != 0 {
                    print!("♟ ");
                } 
                else if self.bitboards.white_rooks & (1 << square_index) != 0 {
                    print!("♜ "); 
                } 
                else if self.bitboards.white_knights & (1 << square_index) != 0 {
                    print!("♞ "); 
                } 
                else if self.bitboards.white_bishops & (1 << square_index) != 0 {
                    print!("♝ "); 
                } 
                else if self.bitboards.white_queens & (1 << square_index) != 0 {
                    print!("♛ ");
                } 
                else if self.bitboards.white_king & (1 << square_index) != 0 {
                    print!("♚ ");
                } 
                else if self.bitboards.black_pawns & (1 << square_index) != 0 {
                    print!("♙ ");
                } 
                else if self.bitboards.black_rooks & (1 << square_index) != 0 {
                    print!("♖ ");
                } 
                else if self.bitboards.black_knights & (1 << square_index) != 0 {
                    print!("♘ ");
                } 
                else if self.bitboards.black_bishops & (1 << square_index) != 0 {
                    print!("♗ ");
                } 
                else if self.bitboards.black_queens & (1 << square_index) != 0 {
                    print!("♕ ");
                } 
                else if self.bitboards.black_king & (1 << square_index) != 0 {
                    print!("♔ ");
                } 
                else {
                    print!(". ");
                }
            }
            println!(); 
        }
        
        println!("  a b c d e f g h");

        match self.turn {
            Turn::White => println!("\nTurn: White"),
            Turn::Black => println!("\nTurn: Black",),
        };
        
        println!("\nPossible moves:");
        print!("Pawns: ");
        for (start, end) in self.pawn_moves() {
            print!("({}, {}) ", Square::from(start), Square::from(end));
        }
        print!("\nKing: ");
        for &(start, end) in &self.king_moves() {
            print!("({}, {}) ", Square::from(start), Square::from(end));
        }
        print!("\nQueens: ");
        for &(start, end) in &self.queen_moves() {
            print!("({}, {}) ", Square::from(start), Square::from(end));
        }
        print!("\nBishops: ");
        for &(start, end) in &self.bishop_moves() {
            print!("({}, {}) ", Square::from(start), Square::from(end));
        }
        print!("\nKnights: ");
        for &(start, end) in &self.knight_moves() {
           print!("({}, {}) ", Square::from(start), Square::from(end));
        }
        print!("\nRooks: ");
        for &(start, end) in &self.rook_moves() {
            print!("({}, {}) ", Square::from(start), Square::from(end));
        }
    }
}
