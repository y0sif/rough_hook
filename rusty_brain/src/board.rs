use core::panic;

use crate::bitboards::Bitboards;
use crate::castling::CastlingRights;
use crate::magic::Magic;
use crate::movement::Move;
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
    
    pub fn generate_legal_moves(&mut self) -> Vec<Move> {
        let (checks, pins) = self.checks_and_pins();
        if checks.len() == 1 { // you have to block the check or capture the piece checking, keeping pins in mind
            self.generate_moves(&pins, checks[0])
        }else if checks.len() == 2 { // double check, have to move the king
            self.king_moves()
        }else { // there is no check, you just have to take care of pins
            self.generate_moves(&pins, !0)
        }
    }
    
    pub fn generate_moves(&mut self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let mut pawn_moves = self.pawn_moves(pins, check_bitboard);
        moves.append(&mut pawn_moves);

        let mut queen_moves = self.queen_moves(pins, check_bitboard);
        moves.append(&mut queen_moves);

        let mut rooks_moves = self.rook_moves(pins, check_bitboard);
        moves.append(&mut rooks_moves);

        let mut bishop_moves = self.bishop_moves(pins, check_bitboard);
        moves.append(&mut bishop_moves);

        let mut knight_moves = self.knight_moves(pins, check_bitboard);
        moves.append(&mut knight_moves);
        let mut king_moves = self.king_moves();
        moves.append(&mut king_moves);

        moves
    }
    
    pub fn checks_and_pins(&self) -> (Vec<u64>, Vec<u8>){
        let (king_bitboard, rooks_bitboard, bishops_bitboard, queen_bitboard, knight_bitboard) = match self.turn {
            Turn::White => (self.bitboards.white_king, self.bitboards.black_rooks, self.bitboards.black_bishops, self.bitboards.black_queens, self.bitboards.black_knights),
            Turn::Black => (self.bitboards.black_king, self.bitboards.white_rooks, self.bitboards.white_bishops, self.bitboards.white_queens, self.bitboards.white_knights)
        };

        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let mut checks = Vec::new();
        let mut pins = Vec::new();

        let orth_directions = [Bitboards::move_north, Bitboards::move_east, Bitboards::move_south, Bitboards::move_west];
        let diag_directions = [Bitboards::move_north_west, Bitboards::move_north_east, Bitboards::move_south_east, Bitboards::move_south_west]; 
        
        for direction in orth_directions {
            self.get_checks_and_pins(&mut checks, &mut pins, king_bitboard, rooks_bitboard | queen_bitboard, ally_bitboard, direction);
        }

        for direction in diag_directions {
            self.get_checks_and_pins(&mut checks, &mut pins, king_bitboard, bishops_bitboard | queen_bitboard, ally_bitboard, direction);
        }
        
        let king_as_knight = self.get_knight_attacked_squares(king_bitboard);
        let opp_knight_square = knight_bitboard & king_as_knight;
        
        if opp_knight_square != 0 {
            checks.push(opp_knight_square);
        }
        (checks, pins)
    } 
    
    fn get_checks_and_pins(&self, checks: &mut Vec<u64>, pins: &mut Vec<u8>, king_bitboard: u64, enemy_bitboard: u64, ally_bitboard: u64, move_fn: fn(u64) -> u64 , ) {
        let mut next_bitboard = move_fn(king_bitboard);
        let mut possible_check = next_bitboard;
        let mut pins_ctr = 0 ;
        let mut flag = false ;
        while next_bitboard != 0 {
            if next_bitboard & ally_bitboard != 0 {
                flag = true;
                if pins_ctr == 0 {
                    pins.push(next_bitboard.trailing_zeros() as u8);
                    pins_ctr +=1;
                }
                else {
                    break;
                }
            }else if next_bitboard & enemy_bitboard != 0 {
                flag = false;
                if pins_ctr == 0 {
                    checks.push(possible_check);
                }
                break;
            }
            next_bitboard = move_fn(next_bitboard);
            possible_check |= next_bitboard;
        }
        if flag{
            pins.pop();
        }
    }

    fn get_attacked_squares(&self) -> u64 {
        match self.turn {
            Turn::White => {
                self.get_pawns_attacked_squares() |
                self.get_knight_attacked_squares(self.bitboards.black_knights) |
                self.get_bishop_attacked_squares(&self.bitboards.black_bishops) |
                self.get_rook_attacked_squares(&self.bitboards.black_rooks) |
                self.get_queen_attacked_squares(&self.bitboards.black_queens) |
                self.get_king_attacked_squares(self.bitboards.black_king)
            },
            Turn::Black => {
                self.get_pawns_attacked_squares() |
                self.get_knight_attacked_squares(self.bitboards.white_knights) |
                self.get_bishop_attacked_squares(&self.bitboards.white_bishops) |
                self.get_rook_attacked_squares(&self.bitboards.white_rooks) |
                self.get_queen_attacked_squares(&self.bitboards.white_queens) |
                self.get_king_attacked_squares(self.bitboards.white_king)
            }
        }
    }
    
    pub fn pawn_moves(&mut self ,pins :&Vec<u8>, check_bitboard: u64) -> Vec<Move> {
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
                    single_push_bitboard & check_bitboard,
                    (single_push_bitboard << 8) & empty_bitboard & rank & check_bitboard,
                    -1,
                    9,
                    7,
                    (self.bitboards.white_pawns << 9) & not_a_file & enemy_bitboard & check_bitboard,
                    (self.bitboards.white_pawns << 7) & not_h_file & enemy_bitboard & check_bitboard         
                )
            },
            Turn::Black => {
                let rank = 0x000000FF00000000; 
                let single_push_bitboard = (self.bitboards.black_pawns >> 8) & empty_bitboard; 
                (
                    single_push_bitboard & check_bitboard,
                    (single_push_bitboard >> 8) & empty_bitboard & rank & check_bitboard,
                    1,
                    7,
                    9,
                    (self.bitboards.black_pawns >> 7) & not_a_file & enemy_bitboard & check_bitboard,
                    (self.bitboards.black_pawns >> 9) & not_h_file & enemy_bitboard & check_bitboard 
                )
            }
        };
        
        Self::get_push_moves(self, &mut moves ,  &mut single_push_bitboard , 1 , push_direction ,pins); 
        Self::get_push_moves(self, &mut moves ,  &mut double_push_bitboard , 2,  push_direction ,pins); 
        Self::get_capture_moves(self,&mut moves , &mut right_captures_bitboard , right_capture_mask , push_direction,pins); 
        Self::get_capture_moves(self,&mut moves , &mut left_captures_bitboard  , left_capture_mask  , push_direction ,pins); 
        Self::check_en_passant(self, &mut moves);
        moves
    }

    fn get_pawns_attacked_squares(&self) -> u64 {
        let not_a_file : u64 = 0xfefefefefefefefe;
        let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;
        
        match self.turn {
            Turn::Black => {
                let right_capture = (self.bitboards.white_pawns << 9) & not_a_file;
                let left_capture = (self.bitboards.white_pawns << 7) & not_h_file;                
                right_capture | left_capture
            },
            Turn::White => {
                let right_capture = (self.bitboards.black_pawns >> 7) & not_a_file;
                let left_capture = (self.bitboards.black_pawns >> 9) & not_h_file;                
                right_capture | left_capture
            }
        }

    }

    fn get_push_moves(&self, moves: &mut Vec<Move>, push_bitboard: &mut u64, steps: i32, push_direction: i32, pins: &Vec<u8>) {
        let flag = match steps {
            1 => 0,
            _ => Move::DOUBLE_PAWN_PUSH,
        };
        while *push_bitboard != 0
        {
            let end_square = push_bitboard.trailing_zeros() as i32; 
            let start_square = end_square + (steps*8*push_direction); 
            let valid_position = *push_bitboard & (!*push_bitboard + 1); 
            let legal_position = Self::get_legal_bitboard(self, &(start_square as u8), pins, &valid_position);
            if legal_position!=0 {
                moves.push(Move::encode(start_square as u8, end_square as u8, flag));            
            }
            *push_bitboard &= *push_bitboard - 1;
        }
    }

    fn get_capture_moves(&self, moves: &mut Vec<Move>, capture_bitboard: &mut u64, capture_mask: i32, push_direction: i32 ,pins: &Vec<u8>) {
        while *capture_bitboard != 0 {
            let end_square = capture_bitboard.trailing_zeros() as u8;
            let start_square = end_square + (capture_mask*push_direction) as u8;
            let valid_position = *capture_bitboard & (!*capture_bitboard + 1); 
            let legal_position = Self::get_legal_bitboard(self, &start_square, pins, &valid_position);
            if legal_position!=0 {
                moves.push(Move::encode(start_square, end_square, Move::CAPTURE));            
            }
            *capture_bitboard &= *capture_bitboard - 1;
        }
    }
    
    fn get_en_passant_moves(moves: &mut Vec<Move>, capture_bitboard : &mut u64, end_square: u8) {
        while *capture_bitboard != 0 {
            let start_square = capture_bitboard.trailing_zeros() as u8;

            moves.push(Move::encode(start_square, end_square, Move::EP_CAPTURE));

            *capture_bitboard &= *capture_bitboard - 1;
        }
    }

    pub fn check_en_passant(&mut self, moves: &mut Vec<Move>){
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

    pub fn knight_moves(&self ,pins :&Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);
        let piece_bitboard = match self.turn {
            Turn::White => self.bitboards.white_knights,
            Turn::Black => self.bitboards.black_knights,
        };

        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_knight_moves(self, &mut moves, piece_position, ally_bitboard ,pins, check_bitboard);
        }
        moves 
    }

    fn get_knight_moves(&self, moves: &mut Vec<Move>, piece_position: u64, ally_bitboard: u64, pins: &Vec<u8>, check_bitboard: u64) {
        let mut valid_bitboard = self.get_knight_attacked_squares(piece_position);
        valid_bitboard &= !ally_bitboard;
        let start_square = piece_position.trailing_zeros() as u8;
        let mut legal_bitboard = Self::get_legal_bitboard(self ,&start_square ,pins , &valid_bitboard) & check_bitboard;
        Self::construct_moves_squares(self, moves, start_square, &mut legal_bitboard); 
    }
    
    fn get_knight_attacked_squares(&self, piece_position: u64) -> u64 {
        let not_ab_file = 0xFCFCFCFCFCFCFCFC;
        let not_a_file = 0xfefefefefefefefe;
        let not_gh_file = 0x3F3F3F3F3F3F3F3F;
        let not_h_file = 0x7f7f7f7f7f7f7f7f;

        let mut valid_bitboard = (piece_position << 17) & not_a_file; //noNoEa
        valid_bitboard |= (piece_position << 10) & not_ab_file; // noEaEa
        valid_bitboard |= (piece_position >> 6)  & not_ab_file; // soEaEa
        valid_bitboard |= (piece_position >> 15) & not_a_file; //soSoEa
        valid_bitboard |= (piece_position << 15) & not_h_file; // noNoWe
        valid_bitboard |= (piece_position << 6)  & not_gh_file; // noWeWe
        valid_bitboard |= (piece_position >> 10) & not_gh_file; // soWeWe
        valid_bitboard |= (piece_position >> 17) & not_h_file; // soSoWe
            
        valid_bitboard
    }
    
    pub fn bishop_moves(&self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves= Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_bishops,
            Turn::Black=>self.bitboards.black_bishops
        };
        
        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_bishop_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard, pins, check_bitboard);
        }
        
        return moves;
    }

    fn get_bishop_moves(&self, moves: &mut Vec<Move>, piece_position: u64, enemy_bitboard: u64, ally_bitboard: u64 , pins: &Vec<u8>, check_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let bishop_mask = Bitboards::bishop_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;


        let blocker = all_pieces & bishop_mask; 
        let key = ((blocker & bishop_mask).wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

        let mut valid_bitboard = self.bishop_attacks[start_square as usize][key as usize];
        valid_bitboard &= !ally_bitboard;
        let mut legal_bitboard = Self::get_legal_bitboard(self, &start_square, pins, &valid_bitboard) & check_bitboard;

        Self::construct_moves_squares(&self, moves, start_square, &mut legal_bitboard); 
    }
    
    fn get_bishop_attacked_squares(&self, piece_bitboard: &u64) -> u64 {
        let mut moves_bitboard = 0;
        let all_pieces = !self.bitboards.get_empty_squares();
        let king_bitboard = match self.turn {
            Turn::White => self.bitboards.white_king,
            Turn::Black => self.bitboards.black_king
        };

        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            let start_square = piece_position.trailing_zeros() as u8;
            let bishop_mask = Bitboards::bishop_mask_ex(start_square);

            let blocker = all_pieces & bishop_mask & !king_bitboard; 
            let key = ((blocker & bishop_mask).wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

            moves_bitboard |= self.bishop_attacks[start_square as usize][key as usize];
            
        }
        
        moves_bitboard       
    }
    
    pub fn rook_moves(&self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves = Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_rooks,
            Turn::Black => self.bitboards.black_rooks
        };

        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_rook_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard, pins, check_bitboard);
        }
        return  moves;
    }

    fn get_rook_moves(&self, moves: &mut Vec<Move>, piece_position: u64 ,enemy_bitboard: u64, ally_bitboard: u64, pins: &Vec<u8>, check_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let rook_mask = Bitboards::rook_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;

        let blocker = all_pieces & rook_mask; 
        let key = ((blocker & rook_mask).wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];
        let mut valid_bitboard = self.rook_attacks[start_square as usize][key as usize];
        valid_bitboard &= !ally_bitboard;
        let mut legal_bitboard = Self::get_legal_bitboard(self, &start_square, pins, &valid_bitboard) & check_bitboard;
        Self::construct_moves_squares(self, moves, start_square, &mut legal_bitboard); 
    }

    fn get_rook_attacked_squares(&self, piece_bitboard: &u64) -> u64 {
        let mut moves_bitboard = 0;
        let all_pieces = !self.bitboards.get_empty_squares();
        let king_bitboard = match self.turn {
            Turn::White => self.bitboards.white_king,
            Turn::Black => self.bitboards.black_king
        };

        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            let start_square = piece_position.trailing_zeros() as u8;
            let rook_mask = Bitboards::rook_mask_ex(start_square);

            let blocker = all_pieces & rook_mask & !king_bitboard; 
            let key = ((blocker & rook_mask).wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];

            moves_bitboard |= self.rook_attacks[start_square as usize][key as usize];
            
        }
        
        moves_bitboard       
    }

    pub fn queen_moves(&self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves = Vec::new();
    
        let enemy_bitboard= self.bitboards.get_enemy_pieces(self.turn);
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let piece_bitboard   = match self.turn {
            Turn::White => self.bitboards.white_queens,
            Turn::Black => self.bitboards.black_queens
        };

        // The queen moves is Combination of bishop and rook moves
        let all_piece_positions = Self::get_piece_positions_from(&piece_bitboard);
        for piece_position in all_piece_positions {
            Self::get_rook_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard, pins, check_bitboard);
            Self::get_bishop_moves(&self, &mut moves, piece_position, enemy_bitboard, ally_bitboard, pins, check_bitboard);
        }

        return moves;
    }
    
    fn get_queen_attacked_squares(&self, piece_bitboard: &u64) -> u64 {
        self.get_bishop_attacked_squares(piece_bitboard) | self.get_rook_attacked_squares(piece_bitboard)
    }

    fn is_pined_square(sqaure : &u8 , pins :&Vec<u8>)-> bool{
        for pinned_sqaure in pins {
            if *sqaure == *pinned_sqaure {
                return true;
            }
        }
        return false;
    }
    pub fn get_direction_mask_ex_using (first_square : &u8 , second_sqaure : &u8)->u64{
        let res = (*first_square as i16 - *second_sqaure as i16).abs();
        if res < 7{
            return Bitboards::rank_mask_to_end_ex(*first_square)
        }
        else if res % 8 == 0{
            return Bitboards::file_mask_to_end_ex(*first_square)
        }
        else if res % 9 == 0{
            return Bitboards::diagonal_mask_ex(*first_square)
        }
        else if res % 7 == 0 { 
            return Bitboards::anti_diagonal_mask_ex(*first_square)
        }
        panic!("Impossible")
    }
    fn get_legal_bitboard(&self , start_square: &u8 , pins: &Vec<u8> ,valid_bitboard : &u64)->u64{
        let mut legal_bitboard = *valid_bitboard;
        let king_square = match self.turn {
            Turn::White=> self.bitboards.white_king.trailing_zeros() as u8,
            Turn::Black=> self.bitboards.black_king.trailing_zeros() as u8
        };
        if Self::is_pined_square(&start_square , pins) {
            let direction_mask = Self::get_direction_mask_ex_using(&start_square , &king_square);
            legal_bitboard &= direction_mask;
        }
        return legal_bitboard;
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
    
    pub fn king_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);
        let piece_bitboard = match self.turn { 
            Turn::White => self.bitboards.white_king,
            Turn::Black => self.bitboards.black_king,
        };
        Self::get_king_moves(self, &mut moves, piece_bitboard, ally_bitboard);
        Self::get_castling_moves(self ,&mut moves ,&piece_bitboard);    
        moves
    }

    fn get_king_moves(&self, moves: &mut Vec<Move>, piece_position: u64, ally_bitboard: u64){
        let mut king_bitboard= piece_position;
        
        let mut valid_bitboard = Bitboards::move_east(king_bitboard) | Bitboards::move_west(king_bitboard);
        king_bitboard |= valid_bitboard;
        valid_bitboard |= Bitboards::move_north(king_bitboard) | Bitboards::move_south(king_bitboard);

        valid_bitboard &= !ally_bitboard;
        valid_bitboard &= !self.get_attacked_squares();

        let start_square = piece_position.trailing_zeros() as u8;
        Self::construct_moves_squares(self, moves, start_square, &mut valid_bitboard);
    }

    pub fn get_castling_moves(&self, moves : &mut Vec<Move> , king_position: &u64) {
        let occupied_bitboard = self.bitboards.get_ally_pieces(self.turn) | self.bitboards.get_enemy_pieces(self.turn);
        if self.castling_rights.check_king_side(self.turn){ //  true :  will be changes after the castling rights is done 
            Self::get_king_side_move(moves ,&king_position , &occupied_bitboard);
        }
        if self.castling_rights.check_queen_side(self.turn){
            Self::get_queen_side_move(moves ,&king_position , &occupied_bitboard);
        }
    }

    fn get_king_side_move(moves : &mut Vec<Move>, king_position : &u64 , occupied_bitboard: &u64){
        let square_between = king_position <<1 | king_position << 2;
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle {
            let start_square = king_position.trailing_zeros() as u8;
            moves.push(Move::encode(start_square, start_square + 2, Move::KING_CASTLE));
        }
    }

    fn get_queen_side_move(moves : &mut Vec<Move>, king_position : &u64 , occupied_bitboard: &u64){
        let square_between = king_position >>1 | king_position >> 2 | king_position >> 3;
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle {
            let start_square = king_position.trailing_zeros() as u8;
            moves.push(Move::encode(start_square, start_square - 2, Move::QUEEN_CASTLE));
        }
    }
    
    fn get_king_attacked_squares(&self, piece_bitboard: u64) -> u64 {
        let mut king_bitboard= piece_bitboard;
        
        let mut valid_bitboard = Bitboards::move_east(king_bitboard) | Bitboards::move_west(king_bitboard);
        king_bitboard |= valid_bitboard;
        valid_bitboard |= Bitboards::move_north(king_bitboard) | Bitboards::move_south(king_bitboard);

        valid_bitboard
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
    
    fn construct_moves_squares(&self, moves : &mut Vec<Move>  , start_square : u8 , valid_bitboard : &mut u64){
        let enemy_bitboard = self.bitboards.get_enemy_pieces(self.turn);
        while *valid_bitboard != 0 {
            let end_squares = valid_bitboard.trailing_zeros() as u8;
            let end_bitboard = 1 << end_squares;
            let flag = match end_bitboard & enemy_bitboard {
                0 => 0,
                _ => Move::CAPTURE,
            };
            moves.push(Move::encode(start_square, end_squares, flag));        
            *valid_bitboard &= *valid_bitboard - 1;
        }
    }

    pub fn print_board(&mut self) {
        let(checks , pins)  = Self::checks_and_pins(self);
        println!("pins : {:?}" , pins);
        println!("checks : {:?}" , checks);
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
        for _move in self.generate_legal_moves() {
            print!("({}, {}) ", Square::from(_move.get_from()), Square::from(_move.get_to()));
        }
    }
}
