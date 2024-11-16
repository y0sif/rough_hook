use core::panic;
use std::collections::HashMap;
use crate::bitboards::Bitboards;
use crate::castling::CastlingRights;
use crate::magic::Magic;
use crate::movement::Move;
use crate::piece::Piece;
use crate::square::Square;
#[derive(Clone, Copy, PartialEq)]
pub enum Turn {
   White,
   Black, 
}
pub struct Board{
    pub bitboards: Bitboards,
    pub turn: Turn,
    pub rook_attacks: [Vec<u64>; 64],
    pub bishop_attacks: [Vec<u64>; 64],
    pub move_log: Vec<Move>,
    pub castling_rights: CastlingRights,
    pub checkmate: bool,
    pub board_hashes: HashMap<u64, u8>,
    pub stalemate: bool,
    pub draw: bool,
    pub half_move_clock: u8,
    pub capture_log: Vec<Piece>,
    pub castling_rights_log: Vec<CastlingRights>,
    pub en_passant_square: Option<Square>,
    pub best_move: Option<Move>,
}

impl Board {
    pub fn new() -> Self {
        Board{
            bitboards: Bitboards::new(),
            turn: Turn::White,
            rook_attacks: Magic::piece_attacks(true),
            bishop_attacks: Magic::piece_attacks(false),
            move_log: Vec::new(),
            castling_rights: CastlingRights::new(),
            checkmate: false,
            board_hashes: HashMap::new(),
            stalemate: false,
            draw: false,
            half_move_clock: 0,
            capture_log: Vec::new(),
            castling_rights_log: Vec::new(),
            en_passant_square: None,
            best_move: None,
        }
    }
    
    pub fn empty() -> Self{
        Board {
            bitboards: Bitboards::empty(),
            turn: Turn::White,
            rook_attacks: Magic::piece_attacks(true),
            bishop_attacks: Magic::piece_attacks(false),
            move_log: Vec::new(),
            castling_rights: CastlingRights::empty(),
            checkmate: false,
            board_hashes: HashMap::new(),
            stalemate: false,
            draw: false,
            half_move_clock:0,
            capture_log: Vec::new(),
            castling_rights_log: Vec::new(),
            en_passant_square: None,
            best_move: None,
        }
    }
    
    pub fn from_fen(fen: String) -> Self {
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
        

        let en_passant_square = match fen_vec[3] {
            "-" => None,
            _ => {
                Some(Square::from(fen_vec[3]))
            }
        };
        
        Board {
            bitboards: Bitboards::from_fen(fen_vec[0]),
            turn,
            rook_attacks: Magic::piece_attacks(true),
            bishop_attacks: Magic::piece_attacks(false),
            move_log: Vec::new(),
            castling_rights,
            checkmate: false,
            board_hashes: HashMap::new(),
            stalemate: false,
            draw: false,
            half_move_clock: fen_vec[4].parse().unwrap(),
            capture_log: Vec::new(),
            castling_rights_log: Vec::new(),
            en_passant_square,
            best_move: None
        }
    }
    
    pub fn make_move(&mut self, move_to_make: Move) {
        self.checkmate = false;
        let start_position = 1 << move_to_make.get_from();
        let end_position = 1 << move_to_make.get_to();
        let not_starting_position = !start_position;
        let flag = move_to_make.get_flags();

        self.move_log.push(move_to_make);
        self.castling_rights_log.push(self.castling_rights);
        self.en_passant_square = None;
        

        match flag {
            Move::CAPTURE | Move::QUEEN_PROMO_CAPTURE | Move::KNIGHT_PROMO_CAPTURE |
            Move::ROOK_PROMO_CAPTURE | Move::BISHOP_PROMO_CAPTURE 
             => {               
                self.board_hashes = HashMap::new();
                self.half_move_clock =0;
                self.make_capture(&move_to_make)
             },
            _ => (),
        }

        match self.turn {
            Turn::White => {
                if start_position & self.bitboards.white_pawns != 0 {
                    match flag {
                        Move::EP_CAPTURE => {
                            self.make_en_passant(end_position);
                            self.bitboards.white_pawns &= not_starting_position;      
                            self.bitboards.white_pawns |= end_position;
                        },
                        Move::QUEEN_PROMOTION | Move::QUEEN_PROMO_CAPTURE => {
                            self.bitboards.white_pawns &= not_starting_position;
                            self.bitboards.white_queens |= end_position;
                        },
                        Move::KNIGHT_PROMOTION | Move::KNIGHT_PROMO_CAPTURE => {
                            self.bitboards.white_pawns &= not_starting_position;
                            self.bitboards.white_knights |= end_position;
                        },
                        Move::ROOK_PROMOTION | Move::ROOK_PROMO_CAPTURE => {
                            self.bitboards.white_pawns &= not_starting_position;
                            self.bitboards.white_rooks |= end_position;
                        },
                        Move::BISHOP_PROMOTION | Move::BISHOP_PROMO_CAPTURE => {
                            self.bitboards.white_pawns &= not_starting_position;
                            self.bitboards.white_bishops |= end_position;
                        },
                        Move::DOUBLE_PAWN_PUSH => {
                            self.en_passant_square = Some(Square::from(move_to_make.get_to() - 8));   
                            self.bitboards.white_pawns &= not_starting_position;      
                            self.bitboards.white_pawns |= end_position;
                        },
                        _ => {
                            self.bitboards.white_pawns &= not_starting_position;      
                            self.bitboards.white_pawns |= end_position;
                        },
                    }
                    self.board_hashes = HashMap::new();
                    self.half_move_clock =0;
                }else if start_position & self.bitboards.white_knights != 0 {
                    self.bitboards.white_knights &= not_starting_position;
                    self.bitboards.white_knights |= end_position;

                }else if start_position & self.bitboards.white_bishops != 0 {
                    self.bitboards.white_bishops &= not_starting_position;
                    self.bitboards.white_bishops |= end_position;

                }else if start_position & self.bitboards.white_rooks != 0 {
                    self.check_rook(&move_to_make);
                    self.bitboards.white_rooks &= not_starting_position;
                    self.bitboards.white_rooks |= end_position;

                }else if start_position & self.bitboards.white_queens != 0 {
                    self.bitboards.white_queens &= not_starting_position;
                    self.bitboards.white_queens |= end_position;

                }else if start_position & self.bitboards.white_king != 0 {
                    match flag {
                        Move::KING_CASTLE => self.make_king_side_move(),
                        Move::QUEEN_CASTLE => self.make_queen_side_move(),
                        _ => ()
                    }
                    self.castling_rights.reset_rights(self.turn);
                    self.bitboards.white_king &= not_starting_position;
                    self.bitboards.white_king |= end_position;

                }
                self.turn = Turn::Black;
            },
            Turn::Black => {
                if start_position & self.bitboards.black_pawns != 0 {
                    match flag {
                        Move::EP_CAPTURE => {
                            self.make_en_passant(end_position);
                            self.bitboards.black_pawns &= not_starting_position;      
                            self.bitboards.black_pawns |= end_position;
                        },
                        Move::QUEEN_PROMOTION | Move::QUEEN_PROMO_CAPTURE => {
                            self.bitboards.black_pawns &= not_starting_position;
                            self.bitboards.black_queens |= end_position;
                        },
                        Move::KNIGHT_PROMOTION | Move::KNIGHT_PROMO_CAPTURE => {
                            self.bitboards.black_pawns &= not_starting_position;
                            self.bitboards.black_knights |= end_position;
                        },
                        Move::ROOK_PROMOTION | Move::ROOK_PROMO_CAPTURE => {
                            self.bitboards.black_pawns &= not_starting_position;
                            self.bitboards.black_rooks |= end_position;
                        },
                        Move::BISHOP_PROMOTION | Move::BISHOP_PROMO_CAPTURE => {
                            self.bitboards.black_pawns &= not_starting_position;
                            self.bitboards.black_bishops |= end_position;
                        },
                        Move::DOUBLE_PAWN_PUSH => {
                            self.en_passant_square = Some(Square::from(move_to_make.get_to() + 8));   
                            self.bitboards.black_pawns &= not_starting_position;      
                            self.bitboards.black_pawns |= end_position;
                        },
                        _ => {
                            self.bitboards.black_pawns &= not_starting_position;      
                            self.bitboards.black_pawns |= end_position;
                        },
                    }
                    self.board_hashes = HashMap::new();
                    self.half_move_clock =0;
                }else if start_position & self.bitboards.black_knights != 0 {
                    self.bitboards.black_knights &= not_starting_position;
                    self.bitboards.black_knights |= end_position;

                }else if start_position & self.bitboards.black_bishops != 0 {
                    self.bitboards.black_bishops &= not_starting_position;
                    self.bitboards.black_bishops |= end_position;

                }else if start_position & self.bitboards.black_rooks != 0 {
                    self.check_rook(&move_to_make);
                    self.bitboards.black_rooks &= not_starting_position;
                    self.bitboards.black_rooks |= end_position;

                }else if start_position & self.bitboards.black_queens != 0 {
                    self.bitboards.black_queens &= not_starting_position;
                    self.bitboards.black_queens |= end_position;

                }else if start_position & self.bitboards.black_king != 0 {
                    match flag {
                        Move::KING_CASTLE => self.make_king_side_move(),
                        Move::QUEEN_CASTLE => self.make_queen_side_move(),
                        _ => ()
                    }
                    self.castling_rights.reset_rights(self.turn);
                    self.bitboards.black_king &= not_starting_position;
                    self.bitboards.black_king |= end_position;

                }
                self.half_move_clock +=1;
                self.turn = Turn::White;
            }
        }

        let count = self.board_hashes.entry(self::Bitboards::hash_board(&self.bitboards)).or_insert(0);
        *count +=1; 
        if *count == 3 {
            self.draw = true; // should probably be turned into a function to stop game
        }        
    }
    
    fn make_en_passant(&mut self, end_position: u64) {
        match self.turn {
            Turn::White => {
                let black_pawn = end_position >> 8;
                self.bitboards.black_pawns &= !black_pawn;
            },
            Turn::Black => {
                let white_pawn = end_position << 8;
                self.bitboards.white_pawns &= !white_pawn;
            }
        }
    }
    pub fn get_en_passant_check(&self ,king_position: &u64, pawn_position1: &u64 , pawn_position2:&u64 , en_passant_position : u64)->bool{
        let mut mask:u64 =0;
        let king_square = king_position.trailing_zeros() as u8;
        let pawn_sqaure = pawn_position1.trailing_zeros() as u8;
        let mut pawns_to_remove = 0;
        let mut dangerous_bitboard :u64 = 0;
        if Bitboards::same_rank(king_square, pawn_sqaure){
            match self.turn {
                Turn::White=>dangerous_bitboard = self.bitboards.black_rooks | self.bitboards.black_queens,
                Turn::Black=>dangerous_bitboard = self.bitboards.white_rooks | self.bitboards.white_queens,
            } 
            mask = Bitboards::rank_mask_to_end(king_square as u8);
            pawns_to_remove = pawn_position1 | pawn_position2;
        }
        else if Bitboards::same_file(king_square, pawn_sqaure){
            match self.turn {
                Turn::White=>dangerous_bitboard = self.bitboards.black_rooks | self.bitboards.black_queens,
                Turn::Black=>dangerous_bitboard = self.bitboards.white_rooks | self.bitboards.white_queens,
            } 
            mask = Bitboards::file_mask_to_end(king_square as u8);
            pawns_to_remove = *pawn_position1;
        } 
        else if Bitboards::same_diagonal(king_square, pawn_sqaure){
            match self.turn {
                Turn::White=>{
                    if pawn_position1 < pawn_position2{
                        return false;
                    }
                    dangerous_bitboard = self.bitboards.black_bishops | self.bitboards.black_queens
                },
                Turn::Black=>{
                    if pawn_position1 > pawn_position2{
                        return false;
                    }
                    dangerous_bitboard = self.bitboards.white_bishops | self.bitboards.white_queens
                }
            } 
            mask = Bitboards::diagonal_mask(king_square as u8);
            pawns_to_remove = *pawn_position1;
        }
        else if Bitboards::same_anti_diagonal(king_square, pawn_sqaure){
            
            match self.turn {
                Turn::White=>{
                    if pawn_position1 > pawn_position2  {
                        return false;
                    }
                    dangerous_bitboard = self.bitboards.black_bishops | self.bitboards.black_queens
                },
                Turn::Black=>{
                    if pawn_position1 < pawn_position2  {
                        return false;
                    }
                    dangerous_bitboard = self.bitboards.white_bishops | self.bitboards.white_queens
                }
            } 
            mask = Bitboards::anti_diagonal_mask(king_square as u8);
            pawns_to_remove = *pawn_position1;

        }
        let all_pieces_on_rank = (self.bitboards.get_ally_pieces(self.turn) |self.bitboards.get_enemy_pieces(self.turn)) & mask;
        let queen_and_rook_bitboard_on_rank:u64 =  mask & dangerous_bitboard;
        let flag =  pawn_sqaure < king_square;
        if queen_and_rook_bitboard_on_rank == 0 {
            return false;
        }
        let related_position ;
            if flag{
                related_position= Bitboards::get_lsb(&queen_and_rook_bitboard_on_rank);
            }
            else {
                related_position= Bitboards::get_msp(&queen_and_rook_bitboard_on_rank);
            }
            
            if (flag ==true && related_position > king_square ) // for example king is in 20 and queen in 22
            || (flag == false && related_position < king_square) { // for example  knig in 22 and queen in 20
                return false;
            }
        
        
        if flag == false {
            let all_pieces_mask_right =Bitboards::get_right_mask(all_pieces_on_rank, king_square);
            let pieces_right_king = all_pieces_on_rank & all_pieces_mask_right &!king_position;

           
            let rook_queen_mask_right =  Bitboards::get_right_mask(dangerous_bitboard, king_square);
            let queen_rook_bitboard_right_of_king = queen_and_rook_bitboard_on_rank&rook_queen_mask_right;
            let closest_index = Bitboards::get_lsb(&queen_rook_bitboard_right_of_king);

            let all_pieces_mask_left = Bitboards::get_left_mask(closest_index);
            let pieces_left_closest_index = all_pieces_on_rank &all_pieces_mask_left;

            let closest_position = 1<<closest_index;
            
            let pieces_between = pieces_right_king & pieces_left_closest_index &!king_position&!closest_position;
            let pieces_between_without_pawns = pieces_between &!pawns_to_remove;
            if pieces_between_without_pawns == 0 {
                return true;
            }
            else {
                return false;
            }

        }
        if flag {
            let all_pieces_mask_left =Bitboards::get_left_mask(king_square);
            let pieces_left_king = all_pieces_on_rank & all_pieces_mask_left &!king_position;

            let rook_queen_mask_left =  Bitboards::get_left_mask(king_square);
            let queen_rook_bitboard_left_of_king = queen_and_rook_bitboard_on_rank&rook_queen_mask_left;
            let closest_index = Bitboards::get_msp(&queen_rook_bitboard_left_of_king);

            let all_pieces_mask_right = Bitboards::get_right_mask(all_pieces_on_rank , closest_index);
            let pieces_right_closest_index = all_pieces_on_rank &all_pieces_mask_right;
            let closest_position = 1<<closest_index;

            let pieces_between = pieces_left_king & pieces_right_closest_index &!king_position&!closest_position;
            let pieces_between_without_pawns = pieces_between &!pawns_to_remove;
            
            if pieces_between_without_pawns == 0 {
                return true;
            }
            else {
                return false;
            }
            
        }
     
        // if Bitboards::same_rank(king_square, pawn_sqaure){
        //     println!("king position = {}" , king_position);
        //     mask = Bitboards::rank_mask_to_end(king_square as u8);
        //     let queen_and_rook_bitboard_on_rank:u64 =  mask & rook_queen_bitboard;
        //     println!("{:b}" ,mask);
        //     println!("{:b}" ,rook_queen_bitboard);
        //     println!("{:b}" , queen_and_rook_bitboard_on_rank);
     
        //     let related_position ;
        //     if flag{
        //         related_position= Bitboards::get_lsb(&rook_queen_bitboard);
        //     }
        //     else {
        //         related_position= Bitboards::get_msp(&rook_queen_bitboard);
        //     }
        //     if queen_and_rook_bitboard_on_rank == 0 
        //     || (flag ==true && related_position > king_square ) // for example king is in 20 and queen in 22
        //     || (flag == false && related_position < king_square) { // for example  knig in 22 and queen in 20
        //         return false;
        //     }
        //     let all_pieces_on_rank = (self.bitboards.get_ally_pieces(self.turn) |self.bitboards.get_enemy_pieces(self.turn)) & mask;

        //     let pieces_after_removal = all_pieces_on_rank & !queen_and_rook_bitboard_on_rank&!pawn_position1&!pawn_position2;

        //     let res;
        //     if flag{
        //         res = Bitboards::get_lsb(&pieces_after_removal);
        //     }else {
        //         res = Bitboards::get_msp(&pieces_after_removal);
        //     }
        //     if res == king_square {
        //         return true;
        //     }
        //     else {
        //         return false
        //     }
        // }
        return false;
    }
    
    fn make_capture(&mut self, move_to_make: &Move) {
        let square_captured = 1 << move_to_make.get_to();
        match self.turn {
            Turn::White => {
                if self.bitboards.black_pawns & square_captured != 0 {
                    self.bitboards.black_pawns &= !square_captured;
                    self.capture_log.push(Piece::Pawn);
                }else if self.bitboards.black_knights & square_captured != 0 {
                    self.bitboards.black_knights &= !square_captured;
                    self.capture_log.push(Piece::Knight);
                }else if self.bitboards.black_bishops & square_captured != 0 {
                    self.bitboards.black_bishops &= !square_captured;
                    self.capture_log.push(Piece::Bishop);
                }else if self.bitboards.black_queens & square_captured != 0 {
                    self.bitboards.black_queens &= !square_captured;
                    self.capture_log.push(Piece::Queen);
                }else if self.bitboards.black_rooks & square_captured != 0 {
                    self.bitboards.black_rooks &= !square_captured;
                    self.capture_log.push(Piece::Rook);
                    self.check_captured_rook(move_to_make, self.bitboards.black_rooks);
                }
            },
            Turn::Black => {
                if self.bitboards.white_pawns & square_captured != 0 {
                    self.bitboards.white_pawns &= !square_captured;
                    self.capture_log.push(Piece::Pawn);
                }else if self.bitboards.white_knights & square_captured != 0 {
                    self.bitboards.white_knights &= !square_captured;
                    self.capture_log.push(Piece::Knight);
                }else if self.bitboards.white_bishops & square_captured != 0 {
                    self.bitboards.white_bishops &= !square_captured;
                    self.capture_log.push(Piece::Bishop);
                }else if self.bitboards.white_queens & square_captured != 0 {
                    self.bitboards.white_queens &= !square_captured;
                    self.capture_log.push(Piece::Queen);
                }else if self.bitboards.white_rooks & square_captured != 0 {
                    self.bitboards.white_rooks &= !square_captured;
                    self.capture_log.push(Piece::Rook);
                    self.check_captured_rook(move_to_make, self.bitboards.white_rooks);
                }
            }
        }
    }
    
    fn check_captured_rook(&mut self, move_to_make: &Move, rook_bitboard: u64) {
        let end_square = move_to_make.get_to();
        let square_captured = !(1 << end_square);
        
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

    fn check_rook(&mut self, move_to_make: &Move) {
        let rook_square = move_to_make.get_from();
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
    
    pub fn undo_move(&mut self) {
        let last_move = self.move_log.pop().unwrap();
        let end_position = 1 << last_move.get_to();
        let start_position = 1 << last_move.get_from();
        let flag = last_move.get_flags();
        
        match flag {
            Move::CAPTURE => self.undo_capture(end_position), 
            Move::EP_CAPTURE => self.undo_en_passant(end_position),
            Move::KING_CASTLE => self.undo_king_side_castling(start_position, end_position),
            Move::QUEEN_CASTLE => self.undo_queen_side_castling(start_position, end_position),
            Move::BISHOP_PROMOTION | Move::ROOK_PROMOTION | Move::QUEEN_PROMOTION | Move::KNIGHT_PROMOTION | 
            Move::ROOK_PROMO_CAPTURE | Move::BISHOP_PROMO_CAPTURE | Move::KNIGHT_PROMO_CAPTURE | Move::QUEEN_PROMO_CAPTURE 
            => {
                self.undo_promotion(start_position, end_position, flag);
                self.castling_rights = self.castling_rights_log.pop().unwrap();
                return; //ignore the rest of the code
            },
            _ => ()
        }
        
        match self.turn {
            Turn::Black => {
                if end_position & self.bitboards.white_pawns != 0 {
                    self.bitboards.white_pawns &= !end_position;
                    self.bitboards.white_pawns |= start_position;
                }else if end_position & self.bitboards.white_knights != 0 {
                    self.bitboards.white_knights &= !end_position;
                    self.bitboards.white_knights |= start_position;

                }else if end_position & self.bitboards.white_bishops != 0 {
                    self.bitboards.white_bishops &= !end_position;
                    self.bitboards.white_bishops |= start_position;

                }else if end_position & self.bitboards.white_rooks != 0 {
                    self.bitboards.white_rooks &= !end_position;
                    self.bitboards.white_rooks |= start_position;

                }else if end_position & self.bitboards.white_queens != 0 {
                    self.bitboards.white_queens &= !end_position;
                    self.bitboards.white_queens |= start_position;

                }else if end_position & self.bitboards.white_king != 0 {
                    self.bitboards.white_king &= !end_position;
                    self.bitboards.white_king |= start_position;

                }
                self.turn = Turn::White;
            },
            Turn::White => {
                if end_position & self.bitboards.black_pawns != 0 {
                    self.bitboards.black_pawns &= !end_position;
                    self.bitboards.black_pawns |= start_position;
                }else if end_position & self.bitboards.black_knights != 0 {
                    self.bitboards.black_knights &= !end_position;
                    self.bitboards.black_knights |= start_position;

                }else if end_position & self.bitboards.black_bishops != 0 {
                    self.bitboards.black_bishops &= !end_position;
                    self.bitboards.black_bishops |= start_position;

                }else if end_position & self.bitboards.black_rooks != 0 {
                    self.bitboards.black_rooks &= !end_position;
                    self.bitboards.black_rooks |= start_position;

                }else if end_position & self.bitboards.black_queens != 0 {
                    self.bitboards.black_queens &= !end_position;
                    self.bitboards.black_queens |= start_position;

                }else if end_position & self.bitboards.black_king != 0 {
                    self.bitboards.black_king &= !end_position;
                    self.bitboards.black_king |= start_position;

                }
                self.turn = Turn::Black;
                
            }
        }
        self.checkmate = false;
        self.stalemate = false;
        self.draw = false;
        self.castling_rights = self.castling_rights_log.pop().unwrap();
    }

    fn undo_promotion(&mut self, start_position: u64, end_position: u64, flag: u8) {
        
        //handle promo captures
        match flag {
            Move::ROOK_PROMO_CAPTURE | Move::BISHOP_PROMO_CAPTURE | 
            Move::KNIGHT_PROMO_CAPTURE | Move::QUEEN_PROMO_CAPTURE =>
             self.undo_capture(end_position),
            _ => ()
        }
        //reset board, remove the promoted piece and revert the pawns
        match self.turn {
            Turn::Black => {
                match flag {
                    Move::QUEEN_PROMOTION | Move::QUEEN_PROMO_CAPTURE => {
                        self.bitboards.white_queens &= !end_position;
                        self.bitboards.white_pawns |= start_position;
                    },
                    Move::ROOK_PROMOTION | Move::ROOK_PROMO_CAPTURE => {
                        self.bitboards.white_rooks &= !end_position;
                        self.bitboards.white_pawns |= start_position;
                    },
                    Move::KNIGHT_PROMOTION | Move::KNIGHT_PROMO_CAPTURE => {
                        self.bitboards.white_knights &= !end_position;
                        self.bitboards.white_pawns |= start_position;
                    },
                    Move::BISHOP_PROMOTION | Move::BISHOP_PROMO_CAPTURE => {
                        self.bitboards.white_bishops &= !end_position;
                        self.bitboards.white_pawns |= start_position;
                    },
                    _ => ()
                }
                self.turn = Turn::White;
            },
            Turn::White => {
                match flag {
                    Move::QUEEN_PROMOTION | Move::QUEEN_PROMO_CAPTURE => {
                        self.bitboards.black_queens &= !end_position;
                        self.bitboards.black_pawns |= start_position;
                    },
                    Move::ROOK_PROMOTION | Move::ROOK_PROMO_CAPTURE => {
                        self.bitboards.black_rooks &= !end_position;
                        self.bitboards.black_pawns |= start_position;
                    },
                    Move::KNIGHT_PROMOTION | Move::KNIGHT_PROMO_CAPTURE => {
                        self.bitboards.black_knights &= !end_position;
                        self.bitboards.black_pawns |= start_position;
                    },
                    Move::BISHOP_PROMOTION | Move::BISHOP_PROMO_CAPTURE => {
                        self.bitboards.black_bishops &= !end_position;
                        self.bitboards.black_pawns |= start_position;
                    },
                    _ => ()
                }
                self.turn = Turn::Black;
            }
        }


    }

    fn undo_capture(&mut self, end_position: u64) {
        match self.turn {
            Turn::White => {
                match self.capture_log.pop().unwrap() {
                    Piece::Pawn => self.bitboards.white_pawns |= end_position,
                    Piece::Knight => self.bitboards.white_knights |= end_position,
                    Piece::Bishop => self.bitboards.white_bishops |= end_position,
                    Piece::Rook => self.bitboards.white_rooks |= end_position,
                    Piece::Queen => self.bitboards.white_queens |= end_position,
                    _ => ()
                }
            },
            Turn::Black => {
                match self.capture_log.pop().unwrap() {
                    Piece::Pawn => self.bitboards.black_pawns |= end_position,
                    Piece::Knight => self.bitboards.black_knights |= end_position,
                    Piece::Bishop => self.bitboards.black_bishops |= end_position,
                    Piece::Rook => self.bitboards.black_rooks |= end_position,
                    Piece::Queen => self.bitboards.black_queens |= end_position,
                    _ => ()
                }
            }
        }
    }
    
    fn undo_en_passant(&mut self, end_position: u64) {
        self.en_passant_square = Some(Square::from(end_position.trailing_zeros() as u8));
        match self.turn {
            Turn::White => self.bitboards.white_pawns |= end_position << 8,
            Turn::Black => self.bitboards.black_pawns |= end_position >> 8,
        }
    }
    
    fn undo_king_side_castling(&mut self, start_position: u64, end_position: u64) {
        match self.turn {
            Turn::White => {
                self.bitboards.black_king &= !end_position;
                self.bitboards.black_king |= start_position;
                self.bitboards.black_rooks &= !(1 << Square::F8 as u8);
                self.bitboards.black_rooks |= 1 << Square::H8 as u8
            },
            Turn::Black => {
                self.bitboards.white_king &= !end_position;
                self.bitboards.white_king |= start_position;
                self.bitboards.white_rooks &= !(1 << Square::F1 as u8);
                self.bitboards.white_rooks |= 1 << Square::H1 as u8
            },
        }
    }
    
    fn undo_queen_side_castling(&mut self, start_position: u64, end_position: u64) {
        match self.turn {
            Turn::White => {
                self.bitboards.black_king &= !end_position;
                self.bitboards.black_king |= start_position;
                self.bitboards.black_rooks &= !(1 << Square::D8 as u8);
                self.bitboards.black_rooks |= 1 << Square::A8 as u8
            },
            Turn::Black => {
                self.bitboards.white_king &= !end_position;
                self.bitboards.white_king |= start_position;
                self.bitboards.white_rooks &= !(1 << Square::D1 as u8);
                self.bitboards.white_rooks |= 1 << Square::A1 as u8
            },
        }
    }
    
    pub fn generate_legal_moves(&mut self) -> Vec<Move> {
        let (checks, pins) = self.checks_and_pins();
        let moves;
        if checks.len() == 1 { // you have to block the check or capture the piece checking, keeping pins in mind
            moves = self.generate_moves(&pins, checks[0]);
            if moves.len() == 0{
                self.checkmate = true
            }
        }else if checks.len() == 2 { // double check, have to move the king
            moves = self.king_moves();
            if moves.len() == 0{
                self.checkmate =true
            }
        }else { // there is no check, you just have to take care of pins
            moves = self.generate_moves(&pins, !0);
            if moves.len() == 0{
                self.stalemate =true;
                self.draw = true;
            }    
        }
        moves
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
    
    pub fn checks_and_pins(&self) -> (Vec<u64>, Vec<u8>) {
        let (king_bitboard, rooks_bitboard, bishops_bitboard, queen_bitboard, knight_bitboard, pawn_bitboard) = match self.turn {
            Turn::White => (self.bitboards.white_king, self.bitboards.black_rooks, self.bitboards.black_bishops, self.bitboards.black_queens, self.bitboards.black_knights, self.bitboards.black_pawns),
            Turn::Black => (self.bitboards.black_king, self.bitboards.white_rooks, self.bitboards.white_bishops, self.bitboards.white_queens, self.bitboards.white_knights, self.bitboards.white_pawns)
        };

        let ally_bitboard = self.bitboards.get_ally_pieces(self.turn);

        let mut checks = Vec::new();
        let mut pins = Vec::new();

        let orth_directions = [Bitboards::move_north, Bitboards::move_east, Bitboards::move_south, Bitboards::move_west];
        let diag_directions = [Bitboards::move_north_west, Bitboards::move_north_east, Bitboards::move_south_east, Bitboards::move_south_west]; 
        
        let orth_bitboard = rooks_bitboard | queen_bitboard;
        let diag_bitboard = bishops_bitboard | queen_bitboard;
        
        for direction in orth_directions {
            self.get_checks_and_pins(&mut checks, &mut pins, king_bitboard, orth_bitboard, ally_bitboard, direction);
        }

        for direction in diag_directions {
            self.get_checks_and_pins(&mut checks, &mut pins, king_bitboard, diag_bitboard, ally_bitboard, direction);
        }
        
        // handle pawn checks
        let opp_pawn_square = match self.turn {
            Turn::Black => {
                let pawn_checks_bitboard = Bitboards::move_south_east(king_bitboard) | Bitboards::move_south_west(king_bitboard);
                pawn_checks_bitboard & pawn_bitboard 
            },
            Turn::White => {
                let pawn_checks_bitboard = Bitboards::move_north_east(king_bitboard) | Bitboards::move_north_west(king_bitboard);
                pawn_checks_bitboard & pawn_bitboard 
            }
        };
        
        if opp_pawn_square != 0 {
            checks.push(opp_pawn_square);
        }
        
        let king_as_knight = self.get_knight_attacked_squares(king_bitboard);
        let opp_knight_square = knight_bitboard & king_as_knight;
        
        if opp_knight_square != 0 {
            checks.push(opp_knight_square);
        }

        (checks, pins)
    } 
    
    fn get_checks_and_pins(&self, checks: &mut Vec<u64>, pins: &mut Vec<u8>, king_bitboard: u64, enemy_bitboard: u64, ally_bitboard: u64, move_fn: fn(u64) -> u64) {
        let non_attacking_enemy = self.bitboards.get_enemy_pieces(self.turn);
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
            }else if next_bitboard & non_attacking_enemy != 0 {
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
    
    pub fn pawn_moves(&mut self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
        let mut moves = Vec::new(); 

        let not_a_file : u64 = 0xfefefefefefefefe;
        let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;

        let empty_bitboard = self.bitboards.get_empty_squares();

        // AND with checkbitboard
        let enemy_bitboard = self.bitboards.get_enemy_pieces(self.turn) & check_bitboard;

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
                    (self.bitboards.white_pawns << 9) & not_a_file,
                    (self.bitboards.white_pawns << 7) & not_h_file        
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
                    (self.bitboards.black_pawns >> 7) & not_a_file,
                    (self.bitboards.black_pawns >> 9) & not_h_file
                )
            }
        };
        
        let promotion_rank:u64 = match push_direction {
            1 => 0x00000000000000FF, // black, so promotion is 1st rank
            _ => 0xFF00000000000000, // -1 is white so promotion is 8th rank
        };
        
        Self::get_push_moves(self, &mut moves, &mut single_push_bitboard, push_direction, pins, promotion_rank); 
        Self::get_double_push_moves(self, &mut moves, &mut double_push_bitboard, push_direction, pins); 

        if let Some(en_passant_capture) = self.en_passant_square {
            Self::get_en_passant_moves(self, &mut moves, en_passant_capture, &mut right_captures_bitboard, right_capture_mask, push_direction, check_bitboard, pins);
            Self::get_en_passant_moves(self, &mut moves, en_passant_capture, &mut left_captures_bitboard, left_capture_mask, push_direction, check_bitboard, pins);
        }
        
        right_captures_bitboard &= enemy_bitboard;
        left_captures_bitboard &= enemy_bitboard;

        Self::get_capture_moves(self,&mut moves , &mut right_captures_bitboard, right_capture_mask, push_direction, pins); 
        Self::get_capture_moves(self,&mut moves , &mut left_captures_bitboard, left_capture_mask, push_direction, pins); 
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

    fn get_push_moves(&self, moves: &mut Vec<Move>, push_bitboard: &mut u64, push_direction: i32, pins: &Vec<u8>, promotion_rank: u64) {
        while *push_bitboard != 0
        {
            let end_square = push_bitboard.trailing_zeros() as i32; 
            let start_square = end_square + (8*push_direction); 
            let curr_rank: u64 = Bitboards::rank_mask_to_end(end_square as u8);
            let valid_position = *push_bitboard & (!*push_bitboard + 1); 
            let legal_position = Self::get_legal_bitboard(self, &(start_square as u8), pins, &valid_position);
            
            if legal_position != 0 {
                //promotion
                if curr_rank == promotion_rank {
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::QUEEN_PROMOTION));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::KNIGHT_PROMOTION));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::ROOK_PROMOTION));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::BISHOP_PROMOTION));            

                }else {
                    moves.push(Move::encode(start_square as u8, end_square as u8, 0));            
                }
            }

            *push_bitboard &= *push_bitboard - 1;
        }
    }

    fn get_double_push_moves(&self, moves: &mut Vec<Move>, push_bitboard: &mut u64, push_direction: i32, pins: &Vec<u8>) {
        while *push_bitboard != 0 {
            let end_square = push_bitboard.trailing_zeros() as i32; 
            let start_square = end_square + (2*8*push_direction); 
            let valid_position = *push_bitboard & (!*push_bitboard + 1); 
            let legal_position = Self::get_legal_bitboard(self, &(start_square as u8), pins, &valid_position);
            
            if legal_position != 0 {
                moves.push(Move::encode(start_square as u8, end_square as u8, Move::DOUBLE_PAWN_PUSH));            
            }

            *push_bitboard &= *push_bitboard - 1;
        }
    }

    fn get_capture_moves(&mut self, moves: &mut Vec<Move>, capture_bitboard: &mut u64, capture_mask: i32, push_direction: i32, pins: &Vec<u8>) {
        let promotion_rank = match push_direction {
            1 => 0x00000000000000FF, // black, so promotion is 1st rank
            _ => 0xFF00000000000000, // -1 is white so promotion is 8th rank
        };

        while *capture_bitboard != 0 {
            let end_square = capture_bitboard.trailing_zeros() as i32;
            let start_square = end_square + (capture_mask*push_direction);
            let curr_rank: u64 = Bitboards::rank_mask_to_end(end_square as u8);
            let valid_position = *capture_bitboard & (!*capture_bitboard + 1); 
            let legal_position = Self::get_legal_bitboard(self, &(start_square as u8), pins, &valid_position);
            //promotion with capture
            if legal_position != 0 {
                //promotion
                if curr_rank == promotion_rank {
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::QUEEN_PROMO_CAPTURE));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::KNIGHT_PROMO_CAPTURE));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::ROOK_PROMO_CAPTURE));            
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::BISHOP_PROMO_CAPTURE));            

                }else {
                    moves.push(Move::encode(start_square as u8, end_square as u8, Move::CAPTURE));            
                }
            }
            *capture_bitboard &= *capture_bitboard - 1;
        }
    }

    fn get_en_passant_moves(&mut self, moves: &mut Vec<Move>, en_passant_capture: Square, capture_bitboard: &mut u64, capture_mask: i32, push_direction: i32, check_bitboard: u64, pins: &Vec<u8>) {
        
        let en_passant_position = 1 << en_passant_capture as u8;
        
        if en_passant_position & *capture_bitboard != 0 {
            let start_square = en_passant_capture as i32 + (capture_mask*push_direction);
            let king_position;
            let ally_pawn_position;
            let enemy_pawn_position;
            let check_bitboard = match self.turn {
                Turn::White=>{
                    king_position = self.bitboards.white_king;
                    ally_pawn_position =  1 << start_square;
                    enemy_pawn_position = en_passant_position >> 8;
                    check_bitboard << 8
                }
                Turn::Black=>{
                    king_position = self.bitboards.black_king;
                    ally_pawn_position =  1 << start_square;
                    enemy_pawn_position = en_passant_position << 8;
                    check_bitboard >> 8
                }
            };

            let valid_position = en_passant_position & (!en_passant_position + 1); 
            let legal_position = check_bitboard & Self::get_legal_bitboard(self, &(start_square as u8), pins, &valid_position);

            if legal_position != 0 {
                if self.get_en_passant_check(&king_position, &ally_pawn_position, &enemy_pawn_position , en_passant_position) == false{
                    let en_passant_move = Move::encode(start_square as u8, en_passant_capture as u8, Move::EP_CAPTURE); 
                    moves.push(en_passant_move);
                }
            }
        }
    }

    pub fn knight_moves(&self, pins: &Vec<u8>, check_bitboard: u64) -> Vec<Move> {
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
        let mut legal_bitboard = Self::get_legal_bitboard(self, &start_square, pins, &valid_bitboard) & check_bitboard;
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
        let key = (blocker.wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

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
            let key = (blocker.wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

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
        let key = (blocker.wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];
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
            let key = (blocker.wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];

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
    fn get_legal_bitboard(&self, start_square: &u8, pins: &Vec<u8>, valid_bitboard: &u64) -> u64 {
        let mut legal_bitboard = *valid_bitboard;
        let king_square = match self.turn {
            Turn::White=> self.bitboards.white_king.trailing_zeros() as u8,
            Turn::Black=> self.bitboards.black_king.trailing_zeros() as u8
        };
        if Self::is_pined_square(&start_square, pins) {
            let direction_mask = Self::get_direction_mask_ex_using(&start_square, &king_square);
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
        if next_position & enemy_bitboard != 0 {   // If the next square is occupied by an enemy piece, add it to the list of possible positions
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

        if self.castling_rights.check_king_side(self.turn){ 
            Self::get_king_side_move(self, moves, &king_position, &occupied_bitboard);
        }

        if self.castling_rights.check_queen_side(self.turn){
            Self::get_queen_side_move(self, moves, &king_position, &occupied_bitboard);
        }
    }

    fn get_king_side_move(&self, moves: &mut Vec<Move>, king_position: &u64, occupied_bitboard: &u64) {
        let square_between = king_position <<1 | king_position << 2;
        let castling_squares = (king_position | square_between) & self.get_attacked_squares();
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle && castling_squares == 0{
            let start_square = king_position.trailing_zeros() as u8;
            moves.push(Move::encode(start_square, start_square + 2, Move::KING_CASTLE));
        }
    }

    fn get_queen_side_move(&self, moves: &mut Vec<Move>, king_position: &u64, occupied_bitboard: &u64) {
        let square_between = king_position >> 1 | king_position >> 2 | king_position >> 3;
        let castling_squares = (king_position >> 1 | king_position >> 2 | king_position) & self.get_attacked_squares();
        let can_castle = square_between & occupied_bitboard == 0;

        if can_castle && castling_squares == 0{
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
    
    fn construct_moves_squares(&self, moves: &mut Vec<Move>, start_square: u8, valid_bitboard: &mut u64) {
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
        
        // println!("\nPossible moves:");
        // let x  = self.generate_legal_moves();
        // println!("x = {}" , x.len());
        // for _move in self.generate_legal_moves() {
        //     print!("({}, {}) ", Square::from(_move.get_from()), Square::from(_move.get_to()));
        // }
        // println!("\n");
    }
}

