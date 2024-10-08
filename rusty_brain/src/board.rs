use crate::bitboards::Bitboards;
use crate::magic::Magic;
use crate::square::Square;

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
        }
    }
    
    pub fn empty() -> Self{
        let rook_attacks = Magic::rook_attacks();
        let bishop_attacks = Magic::bishop_attacks();
        Board {
            bitboards: Bitboards::empty(),
            turn: Turn::White,
            rook_attacks,
            bishop_attacks
        }
    }
    
    pub fn make_move(&self) {
        todo!()
    }
    
    pub fn generate_moves(&self) -> Vec<(u8, u8)> {
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

        moves
    }
    
    pub fn pawn_moves(&self) -> Vec<(u8, u8)> {
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
        moves_bitboard &= !all_pieces;
        
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
        moves_bitboard &= !all_pieces;
        
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
        Self::get_bishop_moves(&self, &mut moves, piece_bitboard, enemy_bitboard, ally_bitboard);
        Self::get_rook_moves(&self, &mut moves, piece_bitboard, enemy_bitboard, ally_bitboard);

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
                    moves.push((king_square as u8, end_square));
                    attacks &= attacks - 1;
                }
            }
        }

        moves
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
            Turn::Black => println!("\nTurn: Balck",),
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
