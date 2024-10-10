use crate::bitboards::Bitboards;
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
}

impl Board {
    pub fn new() -> Self {
        let rook_attacks = Magic::rook_attacks();
        let bishop_attacks = Magic::bishop_attacks();
        Board{
            bitboards: Bitboards::new(),
            turn: Turn::White,
            rook_attacks,
            bishop_attacks,
            move_log: Vec::new(),
        }
    }
    
    pub fn empty() -> Self{
        let rook_attacks = Magic::rook_attacks();
        let bishop_attacks = Magic::bishop_attacks();
        Board {
            bitboards: Bitboards::empty(),
            turn: Turn::White,
            rook_attacks,
            bishop_attacks,
            move_log: Vec::new(),
        }
    }
    
    pub fn make_move(&mut self, move_to_make: (u8, u8)) {
        let start_square = 1 << move_to_make.0;
        let end_square = 1 << move_to_make.1;
        self.move_log.push((move_to_make.0, move_to_make.1));
        match self.turn {
            Turn::White => {
                if start_square & self.bitboards.white_pawns != 0 {
                    self.bitboards.white_pawns &= !start_square;      
                    self.bitboards.white_pawns |= end_square;

                }else if start_square & self.bitboards.white_knights != 0 {
                    self.bitboards.white_knights &= !start_square;
                    self.bitboards.white_knights |= end_square;

                }else if start_square & self.bitboards.white_bishops != 0 {
                    self.bitboards.white_bishops &= !start_square;
                    self.bitboards.white_bishops |= end_square;

                }else if start_square & self.bitboards.white_rooks != 0 {
                    self.bitboards.white_rooks &= !start_square;
                    self.bitboards.white_rooks |= end_square;

                }else if start_square & self.bitboards.white_queens != 0 {
                    self.bitboards.white_queens &= !start_square;
                    self.bitboards.white_queens |= end_square;

                }else if start_square & self.bitboards.white_king != 0 {
                    self.bitboards.white_king &= !start_square;
                    self.bitboards.white_king |= end_square;

                }
                self.turn = Turn::Black;
            },
            Turn::Black => {
                if start_square & self.bitboards.black_pawns != 0 {
                    self.bitboards.black_pawns &= !start_square;      
                    self.bitboards.black_pawns |= end_square;

                }else if start_square & self.bitboards.black_knights != 0 {
                    self.bitboards.black_knights &= !start_square;
                    self.bitboards.black_knights |= end_square;

                }else if start_square & self.bitboards.black_bishops != 0 {
                    self.bitboards.black_bishops &= !start_square;
                    self.bitboards.black_bishops |= end_square;

                }else if start_square & self.bitboards.black_rooks != 0 {
                    self.bitboards.black_rooks &= !start_square;
                    self.bitboards.black_rooks |= end_square;

                }else if start_square & self.bitboards.black_queens != 0 {
                    self.bitboards.black_queens &= !start_square;
                    self.bitboards.black_queens |= end_square;

                }else if start_square & self.bitboards.black_king != 0 {
                    self.bitboards.black_king &= !start_square;
                    self.bitboards.black_king |= end_square;

                }
                self.turn = Turn::White;
                
            }
        }
        
    }
    
    pub fn generate_moves(&self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();

        // let mut pawn_moves = self.pawn_moves();
        // moves.append(&mut pawn_moves);

        // let mut queen_moves = self.queen_moves();
        // moves.append(&mut queen_moves);

        // let mut rooks_moves = self.rook_moves();
        // moves.append(&mut rooks_moves);

        // let mut bishop_moves = self.bishop_moves();
        // moves.append(&mut bishop_moves);

        // let mut knight_moves = self.knight_moves();
        // moves.append(&mut knight_moves);
        
        let mut king_moves = self.king_moves();
        moves.append(&mut king_moves);

        moves
    }
    
    pub fn pawn_moves(&self) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();
        self.check_en_passant(&mut moves);
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

    pub fn check_en_passant(&self, moves: &mut Vec<(u8,u8)>){
        match self.turn{
            Turn::White=>{
                // check if move is actually a pawn double push
                if let Some(last_move) = self.move_log.last(){ 
                    if (1 << last_move.1) & self.bitboards.black_pawns != 0{
                        if Square::from(last_move.0).rank() == Rank::Seventh
                        && Square::from(last_move.1).rank() == Rank::Fifth{
                            //check for adjacent white pawns
                            let white_pawns = self.bitboards.white_pawns;
                            let east_bitboard = Bitboards::move_east(1 << last_move.1);
                            let west_bitboard = Bitboards::move_west(1 << last_move.1);
    
                            let mut ep_captures = white_pawns & (east_bitboard | west_bitboard);
                            let end_square = last_move.1 + 8;

                            while ep_captures != 0{
                                let start_square = ep_captures.trailing_zeros() as u8;
                                
                                moves.push((start_square, end_square));
    
                                ep_captures &= ep_captures - 1;
                            }
                        }      
                    } 
                }
            },
            Turn::Black=>{
                if let Some(last_move) = self.move_log.last(){
                    if (1 << last_move.1) & self.bitboards.white_pawns != 0 {
                        if  Square::from(last_move.0).rank() == Rank::Second
                        && Square::from(last_move.1).rank() == Rank::Forth{
                            let black_pawns = self.bitboards.black_pawns;
                            let east_bitboard = Bitboards::move_east(1 << last_move.1);
                            let west_bitboard = Bitboards::move_west(1 << last_move.1);

                            let mut ep_captures = black_pawns & (east_bitboard | west_bitboard);
                            let end_square = last_move.1 - 8;   

                            while ep_captures != 0{
                                let start_square = ep_captures.trailing_zeros() as u8;

                                moves.push((start_square, end_square));

                                ep_captures &= ep_captures - 1;
                            }
                        }
                    }
                }
            }
        }

    }

    pub fn knight_moves(&self) -> Vec<(u8, u8)> {
        let mut moves: Vec<(u8, u8)> = Vec::new();

        let not_ab_file: u64 = 0xFCFCFCFCFCFCFCFC;
        let not_a_file: u64= 0xfefefefefefefefe;
        let not_gh_file: u64 = 0x3F3F3F3F3F3F3F3F;
        let not_h_file: u64 = 0x7f7f7f7f7f7f7f7f;
        
        match self.turn {
            Turn::White => {
                let not_ally_squares = !self.bitboards.get_ally_pieces(self.turn);
                let mut knights: u64 = self.bitboards.white_knights;
                while knights != 0 {
                    let knight_position = knights.trailing_zeros();
                    let current_knight: u64;
                    let temp = (knights << 63 - knight_position) >> 63 - knight_position; //isolates LSB1
                    if temp != 0 {
                        current_knight = temp;
                    }else {
                        current_knight = knights;
                    }

                    let mut new_square = (current_knight << 17) & not_a_file & not_ally_squares; //noNoEa
                    new_square |= (current_knight << 10) & not_ab_file & not_ally_squares; // noEaEa
                    new_square |= (current_knight >> 6) & not_ab_file & not_ally_squares; // soEaEa
                    new_square |= (current_knight >> 15) & not_a_file & not_ally_squares; //soSoEa
                    new_square |= (current_knight << 15) & not_h_file & not_ally_squares; // noNoWe
                    new_square |= (current_knight << 6) & not_gh_file & not_ally_squares; // noWeWe
                    new_square |= (current_knight >> 10) & not_gh_file & not_ally_squares; // soWeWe
                    new_square |= (current_knight >> 17) & not_h_file & not_ally_squares; // soSoWe
                              
                    let knight_square = current_knight.trailing_zeros() as u8;
                    while new_square != 0 {
                        let to_go_sqaure = new_square.trailing_zeros() as u8;
                        moves.push((knight_square, to_go_sqaure));
                        new_square &= new_square -1;
                    }
                    if knight_position < 63 {
                        knights = (knights >> knight_position+1) << knight_position+1;
                    }else {
                        break;
                    }
                }
            }
            Turn::Black => {
                let mut knights = self.bitboards.black_knights;
                let not_ally_squares = !self.bitboards.get_ally_pieces(self.turn);
                while knights != 0 {
                    let knight_position = knights.trailing_zeros();
                    let current_knight: u64;

                    let temp = (knights << 63 - knight_position) >> 63 - knight_position;
                    if temp != 0 {
                        current_knight = temp;
                    }else {
                        current_knight = knights;
                    }

                    let mut new_square = (current_knight << 17) & not_a_file & not_ally_squares; //noNoEa
                    new_square |= (current_knight << 10) & not_ab_file & not_ally_squares; // noEaEa
                    new_square |= (current_knight >> 6) & not_ab_file & not_ally_squares; // soEaEa
                    new_square |= (current_knight >> 15) & not_a_file & not_ally_squares; //soSoEa
                    new_square |= (current_knight << 15) & not_h_file & not_ally_squares; // noNoWe
                    new_square |= (current_knight << 6) & not_gh_file & not_ally_squares; // noWeWe
                    new_square |= (current_knight >> 10) & not_gh_file & not_ally_squares; // soWeWe
                    new_square |= (current_knight >> 17) & not_h_file & not_ally_squares; // soSoWe
                    
                    let knight_square = current_knight.trailing_zeros() as u8;
                    while new_square != 0 {
                        let to_go_square = new_square.trailing_zeros() as u8;
                        moves.push((knight_square, to_go_square));
                        new_square &= new_square -1;
                    }
                    if knight_position < 63 {
                        knights = (knights >> knight_position+1) << knight_position+1;
                    }else {
                        break;
                    }
                }
            }
        }
        moves
             
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


    // Get the the bit board of all valid positions for a bishop  based on its movement directions
    // And fill the moves vector with the start and end squares for each move
    fn get_bishop_moves(&self, moves: &mut Vec<(u8,u8)>, piece_position: u64, enemy_bitboard: u64, ally_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let bishop_mask = Bitboards::bishop_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;

        let blocker = all_pieces & bishop_mask; 
        let key = ((blocker & bishop_mask).wrapping_mul(Magic::BISHOP_MAGICS[start_square as usize])) >> Magic::BISHOP_SHIFTS[start_square as usize];

        let mut moves_bitboard = self.bishop_attacks[start_square as usize][key as usize];
        moves_bitboard &= !ally_bitboard;
        
        while moves_bitboard != 0 {
            let end_square = moves_bitboard.trailing_zeros() as u8;

            moves.push((start_square, end_square));
            
            moves_bitboard &= moves_bitboard - 1;
        }
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

    // Get the the bit board of all valid positions for a rook based on its movement directions
    // And fill the moves vector with the start and end squares for each move
    fn get_rook_moves(&self, moves: &mut Vec<(u8,u8)>, piece_position: u64 ,enemy_bitboard: u64, ally_bitboard: u64) {
        let start_square = piece_position.trailing_zeros() as u8;
        let rook_mask = Bitboards::rook_mask_ex(start_square);
        let all_pieces = enemy_bitboard | ally_bitboard;

        let blocker = all_pieces & rook_mask; 
        let key = ((blocker & rook_mask).wrapping_mul(Magic::ROOK_MAGICS[start_square as usize])) >> Magic::ROOK_SHIFTS[start_square as usize];

        let mut moves_bitboard = self.rook_attacks[start_square as usize][key as usize];
        moves_bitboard &= !ally_bitboard;
        
        while moves_bitboard != 0 {
            let end_square = moves_bitboard.trailing_zeros() as u8;

            moves.push((start_square, end_square));
            
            moves_bitboard &= moves_bitboard - 1;
        }
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
    
    pub fn bishop_bitboard(piece_position: u64, blockers: u64) -> u64 {

        let mut bishop_bitboard = 0;
        let mut north_east = Bitboards::move_north_east(piece_position);
        
        while north_east & blockers == 0 && north_east != 0{
            bishop_bitboard |= north_east;
            north_east = Bitboards::move_north_east(north_east);
        }
        if north_east & blockers != 0 {
            bishop_bitboard |= north_east;
        }
        
        let mut north_west = Bitboards::move_north_west(piece_position);
        
        while north_west & blockers == 0 && north_west != 0{
            bishop_bitboard |= north_west;
            north_west = Bitboards::move_north_west(north_west);
        }
        if north_west & blockers != 0 {
            bishop_bitboard |= north_west;
        }

        let mut south_east = Bitboards::move_south_east(piece_position);
        
        while south_east & blockers == 0 && south_east != 0{
            bishop_bitboard |= south_east;
            south_east = Bitboards::move_south_east(south_east);
        }
        if south_east & blockers != 0 {
            bishop_bitboard |= south_east;
        }

        let mut south_west = Bitboards::move_south_west(piece_position);
        
        while south_west & blockers == 0 && south_west != 0{
            bishop_bitboard |= south_west;
            south_west = Bitboards::move_south_west(south_west);
        }
        if south_west & blockers != 0 {
            bishop_bitboard |= south_west;
        }

        bishop_bitboard

    }
    
    pub fn rook_bitboard(piece_position: u64, blockers: u64) -> u64 {
        let mut rook_bitboard = 0;
        let mut north = Bitboards::move_north(piece_position);
        
        while north & blockers == 0 && north != 0{
            rook_bitboard |= north;
            north = Bitboards::move_north(north);
        }
        if north & blockers != 0 {
            rook_bitboard |= north;
        }
        
        let mut east = Bitboards::move_east(piece_position);
        
        while east & blockers == 0 && east != 0{
            rook_bitboard |= east;
            east = Bitboards::move_east(east);
        }
        if east & blockers != 0 {
            rook_bitboard |= east;
        }

        let mut south = Bitboards::move_south(piece_position);
        
        while south & blockers == 0 && south != 0{
            rook_bitboard |= south;
            south = Bitboards::move_south(south);
        }
        if south & blockers != 0 {
            rook_bitboard |= south;
        }

        let mut west = Bitboards::move_west(piece_position);
        
        while west & blockers == 0 && west != 0{
            rook_bitboard |= west;
            west = Bitboards::move_west(west);
        }
        if west & blockers != 0 {
            rook_bitboard |= west;
        }

        rook_bitboard
    }
    
    pub fn king_moves(&self) -> Vec<(u8, u8)> {
        
        let mut moves: Vec<(u8, u8)> = Vec::new();
        match self.turn {
            Turn::White => {
                let ally_squares = self.bitboards.get_ally_pieces(Turn::White);
                let king_square = self.bitboards.white_king.trailing_zeros() as u8;
                let mut kingset = self.bitboards.white_king;
                
                let castling_bitboard = self.get_castling_bitboard(&kingset);

                let mut attacks = Bitboards::move_east(kingset) | Bitboards::move_west(kingset);
                kingset |= attacks;
                attacks |= Bitboards::move_north(kingset) | Bitboards::move_south(kingset);

                attacks &= !ally_squares;
                attacks |= castling_bitboard;
                
                while attacks != 0 {
                    let end_square = attacks.trailing_zeros() as u8;
                    moves.push((king_square, end_square));
                    attacks &= attacks - 1;
                }

            },
            Turn::Black => {
                let ally_squares = self.bitboards.get_ally_pieces(Turn::Black);
                let mut kingset = self.bitboards.black_king;
                let king_square = self.bitboards.black_king.trailing_zeros() as u8;

                let castling_bitboard = self.get_castling_bitboard(&kingset);

                let mut attacks = Bitboards::move_east(kingset) | Bitboards::move_west(kingset);
                kingset |= attacks;
                attacks |= Bitboards::move_north(kingset) | Bitboards::move_south(kingset);
                
                attacks &= !ally_squares;
                attacks |= castling_bitboard;
                
                while attacks != 0 {
                    let end_square = attacks.trailing_zeros() as u8;             
                    moves.push((king_square as u8, end_square));
                    attacks &= attacks - 1;
                }
            }
        }
        moves
    }
    pub fn get_castling_bitboard(&self, king_position: &u64) ->u64 {
        if true{ //  true :  will be changes after the castling rights is done 
            let occupied_bitboard = self.bitboards.get_ally_pieces(self.turn) | self.bitboards.get_enemy_pieces(self.turn);
            let king_side_castling_bitboard = Self::get_king_castling_bitboard(king_position , occupied_bitboard);
            let queen_side_castling_bitboard = Self::get_queen_castling_bitboard(king_position , occupied_bitboard);
            return king_side_castling_bitboard | queen_side_castling_bitboard;
        } 
        else {
            0
        }
        
    }
    fn get_king_castling_bitboard(king_position : &u64 , occupied_bitboard: u64) -> u64 {
        println!("king position {:b}", king_position);
        println!("f_sqaure     {:b}", king_position << 1);
        println!("s_sqaure     {:b}", king_position << 2);

        let square_between = king_position <<1 | king_position << 2;
        println!("{:b}", square_between);
        let can_castle = square_between & occupied_bitboard == 0;
        match can_castle {
            true => king_position << 2 , 
            false => 0 ,
        }
    }
    fn get_queen_castling_bitboard(king_position : &u64 , occupied_bitboard: u64) -> u64 {
        let square_between = king_position >>1 | king_position >> 2 | king_position >> 3;
        println!("{:b}", square_between);
        let can_castle = square_between & occupied_bitboard == 0;
        match can_castle {
            true => king_position >> 2 , 
            false => 0 ,
        }
    }
    // get the position for each piece using piece bitboard
    // eg : if the piece bitboard is 0000000000000000000000100000000000000000000100000000000000000001
    //      so the positions will contain : 
    //        -- 0000000000000000000000000000000000000000000000000000000000000001
    //        -- 0000000000000000000000000000000000000000000100000000000000000000
    //        -- 0000000000000000000000100000000000000000000000000000000000000000
    fn get_piece_positions_from(piece_bitboard: &u64) -> Vec<u64> {
        let mut bitboard  =  *piece_bitboard; 
        let mut positions = Vec::new();
        while bitboard != 0 {
            let rook1 = bitboard & (!bitboard + 1); 
            positions.push(rook1);
            bitboard &= bitboard - 1;
        }
        positions
    }

    pub fn print_board(&self) {

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
        for &(start, end) in &self.pawn_moves() {
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
