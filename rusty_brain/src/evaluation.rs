use std::process::id;
use std::vec;
use crate::board;
use crate::piece::Piece;
use std::cmp;

use crate::{bitboards::{self, Bitboards}, board::{Board, Turn}, square::{self, Rank, Square}};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mg = self.middle_game_evaluation(true); //should figure out what to do with these bools
        let mut eg = self.end_game_evaluation(true);
        let p = self.phase(true);
        let rule50= self.rule50();
        //eg = eg * self.scale_factor() / 64; //not implemented
        let mut v = (mg *p + (eg *(138 -p)))/128;
        // if (arguments.length == 1) v = ((v / 16) << 0) * 16; //genuinely what
        v += self.tempo();
        v  = (v * (100 - rule50) / 100);

        v
    }
    
    fn middle_game_evaluation(&self, nowinnable: bool) -> i32 {
        let mut v: i32 = 0;
        let color_flip_board = self.color_flip();
        let (_, pins) = self.checks_and_pins();
        let (_, flip_pins) = color_flip_board.checks_and_pins();

        v += self.piece_value_mg() - color_flip_board.piece_value_mg();
        v += self.psqt_mg() - color_flip_board.psqt_mg();
        v += self.imbalance_total(&color_flip_board); // need to check this cuz in the wiki it doesn't pass flipped board
        v += self.pawns_mg() - color_flip_board.pawns_mg(); 
        v += self.pieces_mg(&pins) - color_flip_board.pieces_mg(&flip_pins);
        v += self.mobility_mg(&pins) - color_flip_board.mobility_mg(&flip_pins);
        v += self.threats_mg() - color_flip_board.threats_mg();
        v += self.passed_mg() - color_flip_board.passed_mg();
        v += self.space(true) - color_flip_board.space(true); // needs to see what tdo with middle_game var
        v += self.king_mg(&pins, &flip_pins) - color_flip_board.king_mg(&flip_pins, &pins);
        
        if !nowinnable {
            v += self.winnable_total_mg(Some(v));
        }

        v
    }

    fn end_game_evaluation(&self, nowinnable: bool) -> i32{
        let mut v: i32 =0;
        let color_flip_board = self.color_flip();
        let (_, pins) = self.checks_and_pins();
        let (_, flip_pins) = color_flip_board.checks_and_pins();

        v += self.piece_value_eg() - color_flip_board.piece_value_eg();
        v += self.psqt_eq() - color_flip_board.psqt_eq();
        v += self.imbalance_total(&color_flip_board); // same thing as mg
        v += self.pawns_eg() - color_flip_board.pawns_eg();
        v += self.pieces_eg(&pins) - color_flip_board.pieces_eg(&flip_pins);
        v += self.mobility_eg(&pins) - color_flip_board.mobility_eg(&flip_pins);
        v += self.threats_eg() - color_flip_board.threats_eg();
        v += self.passed_eg() - color_flip_board.passed_eg();
        v += self.king_eg(&pins, &flip_pins) - color_flip_board.king_eg(&flip_pins, &pins);

        if !nowinnable {
            v += self.winnable_total_eg(Some(v));
        }

    v
    }

    fn rule50(&self) -> i32 {

        self.half_move_clock as i32
    }

    fn tempo(&self) -> i32 {
        let mut v: i32 =0;
        let turn = {
            if self.turn == Turn::White {
                1
            } else {
                -1
            }
        };
        28 * turn
    }
    
    // PIECE VALUE MIDDLE GAME
    
    fn piece_value_mg(&self) -> i32 {
        // self.piece_value_bonus(true)
        let mut sum = 0;

        sum += self.bitboards.white_pawns.count_ones() as i32 * self.piece_value_bonus(Piece::Pawn, true);
        sum += self.bitboards.white_knights.count_ones() as i32 * self.piece_value_bonus(Piece::Knight, true);
        sum += self.bitboards.white_bishops.count_ones() as i32 * self.piece_value_bonus(Piece::Bishop, true);
        sum += self.bitboards.white_rooks.count_ones() as i32 * self.piece_value_bonus(Piece::Rook, true);
        sum += self.bitboards.white_queens.count_ones() as i32 * self.piece_value_bonus(Piece::Queen, true);

        sum
    }

    fn piece_value_eg(&self) -> i32 {
        // self.piece_value_bonus(false)
        let mut sum = 0;

        sum += self.bitboards.white_pawns.count_ones() as i32 * self.piece_value_bonus(Piece::Pawn, false);
        sum += self.bitboards.white_knights.count_ones() as i32 * self.piece_value_bonus(Piece::Knight, false);
        sum += self.bitboards.white_bishops.count_ones() as i32 * self.piece_value_bonus(Piece::Bishop, false);
        sum += self.bitboards.white_rooks.count_ones() as i32 * self.piece_value_bonus(Piece::Rook, false);
        sum += self.bitboards.white_queens.count_ones() as i32 * self.piece_value_bonus(Piece::Queen, false);

        sum
    }

    fn piece_value_bonus(&self ,piece: Piece, is_middle_game: bool) -> i32 {
        // pawn, knight, bishop, rook, queen
        let a = if is_middle_game {
            [124, 781, 825, 1276, 2538]
        } else {
            [206, 854, 915, 1380, 2682]
        };

        match piece {
            Piece::Pawn => a[0],            
            Piece::Knight =>a[1],
            Piece::Bishop =>a[2],
            Piece::Rook =>a[3],
            Piece::Queen =>a[4],
            _ => 0
        }
    }

    fn non_pawn_material(&self, is_middle_game: bool) -> i32 {
        let mut sum = 0;

        sum += self.bitboards.white_knights.count_ones() as i32 * self.piece_value_bonus(Piece::Knight, is_middle_game);
        sum += self.bitboards.white_bishops.count_ones() as i32 * self.piece_value_bonus(Piece::Bishop, is_middle_game);
        sum += self.bitboards.white_rooks.count_ones() as i32 * self.piece_value_bonus(Piece::Rook, is_middle_game);
        sum += self.bitboards.white_queens.count_ones() as i32 * self.piece_value_bonus(Piece::Queen, is_middle_game);

        sum
    }
    
    // PSQT MIDDLE GAME

    fn psqt_mg(&self) -> i32 {
        self.psqt_bonus(true)
    }

    fn psqt_eq(&self) -> i32 {
        self.psqt_bonus(false)
    }
    
    fn psqt_bonus(&self, is_middle_game: bool) -> i32 {
        // knight, bishop, rook, queen, king
        let bonus = if is_middle_game {
            [
                [[-175,-92,-74,-73],[-77,-41,-27,-15],[-61,-17,6,12],[-35,8,40,49],[-34,13,44,51],[-9,22,58,53],[-67,-27,4,37],[-201,-83,-56,-26]],
                [[-53,-5,-8,-23],[-15,8,19,4],[-7,21,-5,17],[-5,11,25,39],[-12,29,22,31],[-16,6,1,11],[-17,-14,5,0],[-48,1,-14,-23]],
                [[-31,-20,-14,-5],[-21,-13,-8,6],[-25,-11,-1,3],[-13,-5,-4,-6],[-27,-15,-4,3],[-22,-2,6,12],[-2,12,16,18],[-17,-19,-1,9]],
                [[3,-5,-5,4],[-3,5,8,12],[-3,6,13,7],[4,5,9,8],[0,14,12,5],[-4,10,6,8],[-5,6,10,8],[-2,-2,1,-2]],
                [[271,327,271,198],[278,303,234,179],[195,258,169,120],[164,190,138,98],[154,179,105,70],[123,145,81,31],[88,120,65,33],[59,89,45,-1]]
            ]
        }else {
            [
                [[-96,-65,-49,-21],[-67,-54,-18,8],[-40,-27,-8,29],[-35,-2,13,28],[-45,-16,9,39],[-51,-44,-16,17],[-69,-50,-51,12],[-100,-88,-56,-17]],
                [[-57,-30,-37,-12],[-37,-13,-17,1],[-16,-1,-2,10],[-20,-6,0,17],[-17,-1,-14,15],[-30,6,4,6],[-31,-20,-1,1],[-46,-42,-37,-24]],
                [[-9,-13,-10,-9],[-12,-9,-1,-2],[6,-8,-2,-6],[-6,1,-9,7],[-5,8,7,-6],[6,1,-7,10],[4,5,20,-5],[18,0,19,13]],
                [[-69,-57,-47,-26],[-55,-31,-22,-4],[-39,-18,-9,3],[-23,-3,13,24],[-29,-6,9,21],[-38,-18,-12,1],[-50,-27,-24,-8],[-75,-52,-43,-36]],
                [[1,45,85,76],[53,100,133,135],[88,130,169,175],[103,156,172,172],[96,166,199,199],[92,172,184,191],[47,121,116,131],[11,59,73,78]]
            ]
        };
        
        let p_bonus = if is_middle_game {
            [
                [0,0,0,0,0,0,0,0],
                [3,3,10,19,16,19,7,-5],
                [-9,-15,11,15,32,22,5,-22],
                [-4,-23,6,20,40,17,4,-8],
                [13,0,-13,1,11,-2,-13,5],
                [5,-12,-7,22,-8,-5,-15,-8],
                [-7,7,-3,-13,5,-16,10,-8],
                [0,0,0,0,0,0,0,0]
            ]
        }else {
            [
                [0,0,0,0,0,0,0,0],
                [-10,-6,10,0,14,7,-5,-19],
                [-10,-10,-10,4,4,3,-6,-4],
                [6,-2,-8,-4,-13,-12,-10,-9],
                [10,5,4,-5,-5,-5,14,9],
                [28,20,21,28,30,7,6,13],
                [0,-11,12,21,25,19,4,7],
                [0,0,0,0,0,0,0,0]
            ]
        };
        
        let mut sum = 0;

        // Helper function to calculate piece bonuses
        let calculate_bonus = |bitboard: u64, table: &[[_; 4]; 8], multiplier| {
            let mut sum = 0;
            let mut bb = bitboard;
            while bb != 0 {
                let square = bb.trailing_zeros() as u8;
                let rank = Square::from(square).rank() as usize;
                let file = Square::from(square).file() as usize;
                let table_rank = if multiplier == 1 {rank as usize } else {7 - rank as usize };
                sum += table[table_rank][usize::min(file, 7 - file)];
                bb &= bb - 1; // Clear the least significant bit
            }
            sum
        };
        

        let mut pawn_bitboard = self.bitboards.white_pawns;
            
        while pawn_bitboard != 0 {
            let square = pawn_bitboard.trailing_zeros() as u8;
            let rank = Square::from(square).rank() as usize;
            let file = Square::from(square).file() as usize;
            
            sum += p_bonus[rank][file];
            
            pawn_bitboard &= pawn_bitboard - 1;
        }

        sum += calculate_bonus(self.bitboards.white_knights, &bonus[0], 1);
        sum += calculate_bonus(self.bitboards.white_bishops, &bonus[1], 1);
        sum += calculate_bonus(self.bitboards.white_rooks, &bonus[2], 1);
        sum += calculate_bonus(self.bitboards.white_queens, &bonus[3], 1);
        sum += calculate_bonus(self.bitboards.white_king, &bonus[4], 1);

        sum
    }

    // IMBALANCE TOTAL
    
    fn imbalance_total(&self, flip: &Board) -> i32 {
        let mut v = 0;
        v += self.imbalance() - flip.imbalance();
        v += self.bishop_pair() - flip.bishop_pair();

        v / 16
    }

    fn imbalance(&self) -> i32 {
        let qo: Vec<Vec<i32>> = vec![
            vec![0],
            vec![40, 38],
            vec![32, 255, -62],
            vec![0, 104, 4, 0],
            vec![-26, -2, 47, 105, -208],
            vec![-189, 24, 117, 133, -134, -6],
        ];

        let qt: Vec<Vec<i32>> = vec![
            vec![0],
            vec![36, 0],
            vec![9, 63, 0],
            vec![59, 65, 42, 0],
            vec![46, 39, 24, -24, 0],
            vec![97, 100, -42, 137, 268, 0],
        ];

        let mut bishop = [0, 0];
        
        let mut v = 0;
        
        let through_piece = |bitboard: u64, table: &Vec<i32>, i: usize, j: usize| {            
            if i % 6 > j{
                return 0;
            }
            else {
                let sum;
                let number_of_pieces = bitboard.count_ones() as i32;
                sum = number_of_pieces * table[i];
                return sum;
            }
        };

        // Helper function to calculate piece bonuses
        let calculate_bonus = |bishop: &[u32; 2], bitboard: u64, ally_table: &Vec<Vec<i32>>, enemy_table: &Vec<Vec<i32>>, idx: usize| {
            let mut sum = 0;
            let mut bb = bitboard;
            while bb != 0 {
                sum += through_piece(self.bitboards.white_pawns, &ally_table[idx], 1, idx);
                sum += through_piece(self.bitboards.white_knights, &ally_table[idx], 2, idx);
                sum += through_piece(self.bitboards.white_bishops, &ally_table[idx], 3, idx);
                sum += through_piece(self.bitboards.white_rooks, &ally_table[idx], 4, idx);
                sum += through_piece(self.bitboards.white_queens, &ally_table[idx], 5, idx);

                sum += through_piece(self.bitboards.black_pawns, &enemy_table[idx], 1, idx);
                sum += through_piece(self.bitboards.black_knights, &enemy_table[idx], 2, idx);
                sum += through_piece(self.bitboards.black_bishops, &enemy_table[idx], 3, idx);
                sum += through_piece(self.bitboards.black_rooks, &enemy_table[idx], 4, idx);
                sum += through_piece(self.bitboards.black_queens, &enemy_table[idx], 5, idx);
                
                        if bishop[0] > 1 {
                            sum += enemy_table[idx][0];
                            //println!("Here1");
                        }
                        if bishop[1] > 1 {
                            //println!("Here2");
                            sum += ally_table[idx][0]
                        }

                bb &= bb - 1;
            }
            
            sum
        };

        bishop[0] = self.bitboards.black_bishops.count_ones();
        bishop[1] = self.bitboards.white_bishops.count_ones();
        
        v += calculate_bonus(&bishop, self.bitboards.white_pawns, &qo, &qt, 1);
        v += calculate_bonus(&bishop, self.bitboards.white_knights, &qo, &qt, 2);
        v += calculate_bonus(&bishop, self.bitboards.white_bishops, &qo, &qt, 3);
        v += calculate_bonus(&bishop, self.bitboards.white_rooks, &qo, &qt,4);
        v += calculate_bonus(&bishop, self.bitboards.white_queens, &qo, &qt,5);

        v
    }
    
    fn bishop_pair(&self) -> i32 {

        if self.bitboards.white_bishops.count_ones() < 2 {
            return 0;
        }else{
            return 1438;
        }
   
    }
    
    // PAWNS MIDDLE GAME
    
    fn pawns_mg(&self) -> i32 {
        // sum function
        
        let mut v = 0;
        
        let mut pawn_bitboard = self.bitboards.white_pawns; 
        
        while pawn_bitboard != 0 {
            let square = pawn_bitboard.trailing_zeros() as u8;
            let square_position = 1 << square;

            if self.doubled_isolated(square_position, square) == 1{
                v -= 11;
            }else if self.isolated(square_position,square) == 1{
                v -= 5;
            }else if self.backward(square_position,square) == 1{
                v -= 9;
            }

            v -= self.doubled(square) * 11;
        
            if self.connected(square_position,square) == 1{
                v += self.connected_bonus(square_position,square);
            }
            
            v -= 13 * self.weak_unopposeed_pawn(square_position, square);
            
            let arr = [0, -11, -3];
            
            v += arr[self.blocked(square_position, square) as usize]; 

            pawn_bitboard &= pawn_bitboard - 1;
        }
    
        v
    }

    fn pawns_eg(&self) -> i32 {

        let mut v: i32 = 0;
        
        let mut pawn_bitboard = self.bitboards.white_pawns;

        while pawn_bitboard != 0 {
            let square = pawn_bitboard.trailing_zeros() as u8;
            let square_position = 1 << square;

            if self.doubled_isolated(square_position, square) == 1{
                v -= 56;
            }else if self.isolated(square_position,square) == 1{
                v -= 15;
            }else if self.backward(square_position,square) == 1{
                v -= 24;
            }

            v -= self.doubled(square) * 56;

                    
            if self.connected(square_position,square) == 1{
                v += (self.connected_bonus(square_position,square) * (Square::from(square).rank() as i32 - 3)/ 4);
            }

            v -= 27 * self.weak_unopposeed_pawn(square_position, square);

            v -= 56 * self.weak_lever(square_position, square);

            let arr = [0, -4, 4];

            v += arr[self.blocked(square_position, square) as usize];            

            pawn_bitboard &= pawn_bitboard - 1;
        }
        v 
    }
    
    fn weak_lever(&self, square_position: u64, square: u8) -> i32 {
        let not_a_file = 0xFEFEFEFEFEFEFEFE;
        let not_h_file = 0x7F7F7F7F7F7F7F7F;

        let enemy_pawms = self.bitboards.black_pawns;
        let ally_pawns = self.bitboards.white_pawns;
        let attacked_twice =  (square_position << 7) & not_a_file & enemy_pawms != 0 && 
            (square_position << 9) & not_h_file & enemy_pawms != 0;

        let not_defended = (square_position >> 7) & not_h_file & ally_pawns == 0 &&
            (square_position >> 9) & not_a_file & ally_pawns == 0;
        
        if attacked_twice && not_defended {
            1
        } else {
            0
        }
    }

    // return if current pawn is double isolated or not
    // return two values only 0 - 1
    fn doubled_isolated(&self,square_position: u64, square: u8) -> i32 {

        // Check if the pawn is isolated
        if self.isolated(square_position, square) == 1 {
            let mut friendly_pawns_below = 0; // Friendly pawns below
            let mut enemy_pawns_above = 0;    // Enemy pawns above
            let mut enemy_pawns_adjacent = 0; // Enemy pawns on adjacent files

            // Get the rank of the square (0 = rank 1, 7 = rank 8)
            let rank = square / 8;

            // If the pawn is on the 8th rank, it cannot be doubled isolated
            if rank == 7 {
                return 0;
            }

            // Count friendly pawns below the current pawn
            friendly_pawns_below += (Bitboards::south_mask_ex(square) & self.bitboards.white_pawns).count_ones();

            // Count enemy pawns above the current pawn
            enemy_pawns_above += (Bitboards::north_mask_ex(square) & self.bitboards.black_pawns).count_ones();

            // Count enemy pawns on adjacent files
            if square % 8 > 0 {
                // Check the left file (x - 1), but only if not on the a-file
                enemy_pawns_adjacent += (Bitboards::file_mask(square - 1) & self.bitboards.black_pawns).count_ones();
            }
            if square % 8 < 7 {
                // Check the right file (x + 1), but only if not on the h-file
                enemy_pawns_adjacent += (Bitboards::file_mask(square + 1) & self.bitboards.black_pawns).count_ones();
            }

            // Check for doubled isolated pawns
            if friendly_pawns_below > 0 && enemy_pawns_above > 0 && enemy_pawns_adjacent == 0 {
                return 1; // Doubled isolated
            }
        }

        0 // Not doubled isolated
    }

    // return if current pawn is isolated or not
    // return two values only 0 - 1
    fn isolated(&self,square_position: u64, square: u8) -> i32 {
        let file = square % 8;
        let mut neighbor_pawns = 0u64;
        if file < 7 {
            neighbor_pawns |= Bitboards::file_mask_to_end(Bitboards::move_east(square_position).trailing_zeros() as u8);
        }
        if file > 0 {
            neighbor_pawns |= Bitboards::file_mask_to_end(Bitboards::move_west(square_position).trailing_zeros() as u8);
        }

        if neighbor_pawns & self.bitboards.white_pawns != 0 {
            return 0;
        }

        1
    }

    // return if current pawn is backward or not
    // return two values only 0 - 1
    pub fn backward(&self,square_position: u64, square: u8) -> i32 {
        let file = square % 8;
        let rank = square / 8;
        let mut neighbor_pawns = 0u64;
        
        if file < 7 {
            neighbor_pawns |= Bitboards::file_mask_to_end(Bitboards::move_east(square_position).trailing_zeros() as u8);
        }
        if file > 0 {
            neighbor_pawns |= Bitboards::file_mask_to_end(Bitboards::move_west(square_position).trailing_zeros() as u8);
        }

        // in the conditions of bacjward pawn: no friendly pawns on adjacent files, but if the friendly
        // pawns above the desired pawn no problem
        let number_of_adjacent_pawns = (neighbor_pawns & self.bitboards.white_pawns).count_ones();
        if  number_of_adjacent_pawns != 0 {
            // We will calculate number of friendly pawns above me and if it equal to number_of_adjacent_pawns so there is no problem

            let mut friendly_pawns_above = 0;

            if file < 7 {
                friendly_pawns_above |= Bitboards::north_mask_ex(Bitboards::move_east(square_position).trailing_zeros() as u8);
            }
            if file > 0 {
                friendly_pawns_above |= Bitboards::north_mask_ex(Bitboards::move_west(square_position).trailing_zeros() as u8);
            }

            let number_of_friendly_pawns_above = (friendly_pawns_above & self.bitboards.white_pawns).count_ones();

            // if number_of_friendly_pawns_above = number_of_adjacent_pawns so all pawns are above the desired one
            // so it is valid, other that not valid
            if number_of_adjacent_pawns != number_of_friendly_pawns_above{
                return 0;
            }    
        }
        // now, for the enemy pawns: 
        // directly is very easy just go step north
        let mut enemy_pawns = 0;
        if rank < 7
        {
            enemy_pawns |= Bitboards::move_north(square_position);
        }

        // not directly, We will go two steps north then step right and step left
        if rank < 6 
        {
            // move two steps above, we want square number, then square position

            let new_square = square + 8 + 8;
            // check on left most file and right most file
            if file < 7 {
                let new_position = 1 << (new_square + 1);

                enemy_pawns |= new_position;
            }
            if file > 0 {
                let new_position = 1 << (new_square - 1);

                enemy_pawns |= new_position;
            }
        }
        
        if enemy_pawns & self.bitboards.black_pawns != 0 {
            return 1;
        }
        
    
        0
    }
    
    // return if current pawn is doubled or not
    // return two values only 0 - 1
    fn doubled(&self, square: u8) -> i32 {
        let file = square % 8;
        let rank = square / 8;        
        /*
        Stockfish evaluates doubled pawns more specifically. It applies a penalty only if:

        1- Another Friendly Pawn is Directly Behind:

            There is a friendly pawn on the square directly behind the current pawn on the same file.

            For example, if White has pawns on c3 and c2, the pawn on c3 is considered doubled because the pawn on c2 is directly behind it.

        2- The Doubled Pawn is Not Supported:

            The doubled pawn is not supported by friendly pawns on adjacent files.

            For example, if the pawn on c3 has no friendly pawns on the b or d files, it is considered unsupported.
        */

        // check for pawn which is directly behind
        if rank > 0
        {
            let new_square = square - 8;
            let new_position = 1 << new_square;
            if new_position & self.bitboards.white_pawns == 0 // means no pawns directly behind me
            {
                return 0;
            }
            // know check for supoorted pawn, we will move one step down then one step right and left
            let mut supported_pawns = 0;
            if file < 7 {
                let east_square = new_square + 1;
                supported_pawns |= 1 << east_square;
            }
            if file > 0 {
                let weast_square = new_square - 1;
                supported_pawns |= 1 << weast_square;

            }
            // if there is supported pawns so not doubled
            if supported_pawns & self.bitboards.white_pawns != 0
            {
                return 0;
            }
            return 1;
        }
        else {
            return 0;
        }
              
    }

    // return 1 if the pawn connected or phalanx
    fn connected(&self, square_position: u64, square: u8) -> i32 {
        if self.supported(square) != 0 || self.phalanx(square_position, square) == 1{
            return 1;
        }   

        0
    }
    
    // return number of pawns support the current pawn
    // it can return only 0 - 1 - 2
    fn supported(&self, square: u8) -> i32 {
        let file = square % 8;
        let rank = square / 8;

        // check for pawn which is directly behind
        if rank > 0
        {
            let new_square = square - 8;
            // know check for supoorted pawn, we will move one step down then one step right and left
            let mut supported_pawns = 0;
            if file < 7 {
                let east_square = new_square + 1;
                supported_pawns |= 1 << east_square;
            }
            if file > 0 {
                let weast_square = new_square - 1;
                supported_pawns |= 1 << weast_square;

            }
            // number of supported pawns
            return (supported_pawns & self.bitboards.white_pawns).count_ones() as i32;
            
        }
        else {
            return 0;
        } 
    }
    
    // check if the current pawn is phalanx or not
    // return only two values 0 - 1
    fn phalanx(&self,square_position: u64, square: u8) -> i32 {
        let file = square % 8;
        let mut phalan = 0;
        if file < 7
        {
            phalan |= Bitboards::move_east(square_position);
        }
        if file > 0
        {
            phalan |= Bitboards::move_west(square_position);
        }

        if phalan & self.bitboards.white_pawns != 0 {
            return 1;
        }

        0
    }

    fn connected_bonus(&self,square_position: u64, square: u8) -> i32 {

        if self.connected(square_position, square) == 0{
            return 0;
        }

        let seed = [0, 7, 8, 12, 29, 48, 86];
        
        let op = self.opposed(square);
        let ph = self.phalanx(square_position, square);
        let su = self.supported(square);
        
        // unusable variable
        // let bl = match self.turn {
        //     Turn::White => {
        //         if Bitboards::move_north(square_position) & self.bitboards.black_pawns != 0 {
        //             return 1;
        //         }
        //         0
        //     },
        //     Turn::Black => {
        //         if Bitboards::move_south(square_position) & self.bitboards.white_pawns != 0 {
        //             return 1;
        //         }
        //         0
        //     },
        // };
        
        let r = (Square::from(square).rank() as usize) + 1;
        
        if r < 2 || r > 7 {
            return 0;
        }

        seed[r - 1] * (2 + ph - op) + 21 * su
         
    }

    fn weak_unopposeed_pawn(&self, square_position: u64, square: u8) -> i32 {
        if self.opposed(square) == 1{
            return 0;
        }

        let mut v = 0;

        if self.isolated(square_position,square) == 1{
            v += 1;
        }else if self.backward(square_position, square) == 1{
            v += 1;
        }

        v
    }

    fn opposed(&self,square: u8) -> i32 {
        let op = Bitboards::north_mask_ex(square) & self.bitboards.black_pawns;
        
        if op != 0 {
            return 1;
        }

        0
    }
    
    // Only considers white pawns on ranks 5 and 6 (1-based).

    fn blocked(&self, square_position: u64, square: u8) -> i32 {

        let rank = (Square::from(square).rank() as usize) + 1;
        
        if rank != 5 && rank != 6 {
            return 0;
        }
        
        if Bitboards::move_north(square_position) & self.bitboards.black_pawns == 0 {
            return 0;
        }
        
        // based on understanding stock fish logic
        // if the pawn at rank 6 (1 based) the function must return 2
        // if the pawn at rank 5 (1 based) the function must return 1
        if rank == 5{
            return 1;
        }
        else{ // must be rank 6
            return 2;
        }
    }

    // PIECES MIDDLES GAME

    fn pieces_mg(&self, pins: &Vec<u8>) -> i32 {
        let mut v = 0;
        v += [0, 31, -7, 30, 56][self.outpost_total() as usize];
        v += 18 * self.minor_behind_pawn();
        v -= 3 * self.bishop_pawns();
        v -= 4 * self.bishop_xray_pawns();
        v += 6 * self.rook_on_queen_file();
        v += 16 * self.rook_on_king_ring(pins);
        v += self.rook_on_file();
        v -= self.trapped_rook(pins) * 55 ; //idk incomplete for now
        v -= 56 * self.weak_queen();
        v -= 2 * self.queen_infiltration();
        //king protector line idk
        v += 45 * self.long_diagonal_bishop();
        return v;
    }

    fn pieces_eg(&self, pins: &Vec<u8>) -> i32 {
        let mut v: i32 =0;

        v
    }

    fn outpost_total(&self) -> i32 {
        let mut v = Vec::new();

        let left_board_mask: u64 = 0xf0f0f0f0f0f0f0f0;
        let right_board_mask: u64 = 0x0f0f0f0f0f0f0f0f;

        let outposts = self.outpost();
        let reachable_outposts  =self.reachable_outposts(); //only knights, no bishops

        let bishops_outposts = self.bitboards.white_bishops & outposts;
        // while bishops_outposts != 0 {
        let count = bishops_outposts.count_ones();
        for n in 0..count {
            v.push(3);// outpost bishops worth 3
        }
        // }
        let mut knight_outposts = self.bitboards.white_knights & outposts;
        let only_reachable_knights = reachable_outposts & ! knight_outposts;
        let count = only_reachable_knights.count_ones();
        for n in 0..count {
            v.push(1); //reachable outpost knights worth 1
        }

        while knight_outposts != 0 {
            let knight_idx = knight_outposts.trailing_zeros();
            let knight = 1u64 << knight_idx;
            if (Square::from(knight_idx as u8).file() as usize) < (2 as usize) || (Square::from(knight_idx as u8).file() as usize) > (5 as usize) {

            }
            knight_outposts &= knight_outposts -1;
        }


        0
    }

    fn outpost(&self) -> u64 {
        let outpost_squares: u64 = self.outpost_squares();
        let knights_and_bishops: u64 = self.bitboards.white_knights | self.bitboards.white_bishops;
        let outposts = outpost_squares & knights_and_bishops;
        outposts
    } 

    fn reachable_outposts(&self) -> u64 {
        let mut reachable_outposts : u64 =0;
        let mut bishops = self.bitboards.white_bishops;
        let mut knights = self.bitboards.white_knights;
        let outpost_squares = self.outpost_squares();
        while knights != 0 {
            let knight_idx = knights.trailing_zeros() as u64;
            let knight = 1u64 << knight_idx;
            let knight_moves = self.get_knight_attacked_squares(knight);
            if knight_moves & outpost_squares !=0{
                reachable_outposts |= knight;
            }
            knights &= knights-1;
        }
        // Original function checks for reachable outpost bishops, but they are never actually used in the parent functions
        // So I removed them 

        // while bishops != 0{
        //     let bishop_idx = bishops.trailing_zeros() as u64;
        //     let bishop = 1u64 << bishop_idx;
        //     let bishop_moves = self.bishop_xray_attack(pins, bishop);
        //     if bishop_moves & outpost_squares != 0 {
        //         reachable_outposts |= bishop;
        //     }
        //     bishops &= bishops -1;
        // }
        reachable_outposts
    }

    pub fn outpost_squares(&self) -> u64 {
        let not_a_file : u64 = 0xfefefefefefefefe;
        let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;

        let ranks_456_mask : u64 = 0x0000FFFFFF000000;
        let enemy_pawn_attack_span: u64 = self.pawn_attacks_span();
        let pawns = self.bitboards.white_pawns;
        let pawn_diagonals =  (pawns >> 7) & not_a_file | (pawns >> 9) & not_h_file;
        let outpost_squares_bitboard = ranks_456_mask & !enemy_pawn_attack_span & pawn_diagonals;

        outpost_squares_bitboard
    }

    pub fn pawn_attacks_span(&self) -> u64 {
        // let mut enemy_pawns = self.bitboards.black_pawns;
        let color_flipped_board = self.color_flip();

        let mut my_pawns =  color_flipped_board.bitboards.white_pawns; // blackpawns, but now flipped as white
        let mut pawn_attack_span_bitboard: u64 =0;

        let not_a_file : u64 = 0xfefefefefefefefe;
        let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;

        // while enemy_pawns != 0 {
        //     let mut pawn_idx = enemy_pawns.trailing_zeros() as i64;
        //     let pawn = (1u64 << pawn_idx);
        //     let mut attack_diagonals = (pawn>> 7 & not_a_file) | (pawn >> 9 & not_h_file); 
        //     pawn_attack_span_bitboard |= attack_diagonals;
        //     if !((pawn >> 8 ) & self.bitboards.white_pawns != 0 || //blocked right ifnront of it
        //     //(pawn >>15 ) & self.bitboards.white_pawns != 0 || //blocked right diagonal under
        //     //(pawn >> 17 ) & self.bitboards.white_pawns != 0 || //blocked left idagonal under
        //     self.backward(pawn, pawn_idx as u8) == 1){
        //         while attack_diagonals != 0 {
        //                 attack_diagonals = attack_diagonals >> 8;
        //                 pawn_attack_span_bitboard |= attack_diagonals;
        //                 // pawn_idx -=  8;
        //             }
        //         }
        //     if self.backward(pawn, pawn_idx as u8) == 1 {
        //         println!("backward pawn at idx: {}", pawn_idx);
        //     }
        //     else {
        //         println!("not backward pawn at idx: {}", pawn_idx);
        //     }
        //     enemy_pawns &= enemy_pawns -1;
        // }
        while my_pawns != 0 {
            let mut pawn_idx = my_pawns.trailing_zeros() as i64;
            let pawn = (1u64 << pawn_idx);
            let mut attack_diagonals = (pawn << 9 & not_a_file) | (pawn << 7 & not_h_file);
            pawn_attack_span_bitboard |= attack_diagonals;
            if !((pawn << 8 ) & self.bitboards.black_pawns != 0 || //blocked right ifnront of it
            //(pawn >>15 ) & self.bitboards.white_pawns != 0 || //blocked right diagonal under
            //(pawn >> 17 ) & self.bitboards.white_pawns != 0 || //blocked left idagonal under
            self.backward(pawn, pawn_idx as u8) == 1){
                while attack_diagonals != 0 {
                        attack_diagonals = attack_diagonals << 8;
                        pawn_attack_span_bitboard |= attack_diagonals;
                        // pawn_idx -=  8;
                    }
                }
                // if self.backward(pawn, pawn_idx as u8) == 1 {
                //     println!("backward pawn at idx: {}", pawn_idx);
                // }
                // else {
                //     println!("not backward pawn at idx: {}", pawn_idx);
                // }
            my_pawns &= my_pawns -1;
        }

        self.flip_vertical(pawn_attack_span_bitboard)
    }

    fn minor_behind_pawn(&self) -> i32 {
        let mut sum = 0;
        let mut knights_and_bishops_bitboard = self.bitboards.white_knights | self.bitboards.white_bishops;
        while knights_and_bishops_bitboard != 0{
            let square: u8 = knights_and_bishops_bitboard.trailing_zeros() as u8;
            let above_square = square + 8;
            if above_square < 63 {
                if self.bitboards.white_pawns & (1 << above_square) !=0{
                    sum +=1;
                } 
            }
            knights_and_bishops_bitboard &= knights_and_bishops_bitboard -1;
        }
        sum
    }

    fn bishop_pawns(&self) -> i32 { //untested
        // Light squares (a1, c1, e1, g1, b2, d2, f2, h2, etc)
        let light_squares: u64 = 0x55AA55AA55AA55AA;
        // Dark squares (b1, d1, f1, h1, a2, c2, e2, g2, etc)
        let dark_squares : u64 = 0xAA55AA55AA55AA55;
        let center_files_mask: u64 = 0x3C3C3C3C3C3C3C3C;
        let mut bishops_bitboard = self.bitboards.white_bishops;
        let pawns_bitboard = self.bitboards.white_pawns;
        let all_pieces_bitboard = !self.bitboards.get_empty_squares();
        let mut sum : i32 = 0;

        let mut blocked = 0;
        let mut center_pawns = pawns_bitboard & center_files_mask;
        let infront_pawns = center_pawns << 8;
        let blocked_pawns = infront_pawns & all_pieces_bitboard;
        blocked = blocked_pawns.count_ones() as i32;
        // if a pawn shifted off the board, it was on the last rank, therefore it is also considered blocked
        blocked += (center_pawns.count_ones() - infront_pawns.count_ones()) as i32;

        while bishops_bitboard != 0 {
            let bishop_idx = bishops_bitboard.trailing_zeros() as u8;
            let bishop = (1u64 << bishop_idx);
            let is_light = (bishop & light_squares) != 0;

            let same_color_pawns = if is_light {
                pawns_bitboard & light_squares
            } else {
                pawns_bitboard & dark_squares
            };

            let same_color_pawns_count = same_color_pawns.count_ones() as i32;
            sum += same_color_pawns_count * (blocked + 1);
            bishops_bitboard &= bishops_bitboard - 1;
        }

        sum
    }


    // I will not use this function and replace it with another one
    // fn pawn_attack(&self) -> i32 { //might be able to remove and replace

    //     0
    // }

    fn bishop_xray_pawns(&self) -> i32 { //untested
        let mut sum: i32 = 0;
        let mut bishop_bitboard = self.bitboards.white_bishops;
        let enemy_pawns = self.bitboards.black_pawns;
        while bishop_bitboard != 0{
            let bishop_square = bishop_bitboard.trailing_zeros() as u8;
            let bishop_mask = Bitboards::bishop_mask(bishop_square);
            let bishop_xray_pawns = bishop_mask & enemy_pawns;
            sum += bishop_xray_pawns.count_ones() as i32;
            bishop_bitboard &= bishop_bitboard-1;
        }
        sum
    }

    fn rook_on_queen_file(&self) -> i32 {
        let mut sum =0;
        let mut rook_bitboard = self.bitboards.white_rooks;
        let queen = self.bitboards.white_queens.trailing_zeros() as u8;
        let queen_file = Square::from(queen).file();
        while rook_bitboard != 0 {
            let rook = rook_bitboard.trailing_zeros() as u8;
            let rook_file = Square::from(rook).file();
            if queen_file == rook_file{
                sum +=1
            }
            rook_bitboard &= rook_bitboard -1;
        }
        sum
    }

    fn rook_on_king_ring(&self,pins : &Vec<u8>) -> i32 {
        let mut sum = 0;
        let mut rook_bitboard = self.bitboards.white_rooks;
        let king_ring = self.king_ring(false);
        while rook_bitboard != 0 {
            let rook_idx = rook_bitboard.trailing_zeros() as u8;
            let rook = (1u64 << rook_idx);
            
            //
            let king_ring_for_pawns = self.king_ring(true);
            let normal_king_ring = self.king_ring(false);
            //

            let rook_king_attacker_count = self.king_attackers_count( pins, king_ring_for_pawns, normal_king_ring);
            if !(rook_king_attacker_count.0 > 0) {
                let rook_file_mask = Bitboards::file_mask_to_end(rook_idx);
                if rook_file_mask & king_ring != 0 {
                    sum += 1;
                }
            }

            rook_bitboard &= rook_bitboard -1;
        }

        sum
        
    }

        // i will check if it is in the bitboard or not

// here we return number of pieces for Knight, Queen, Bishop, Rook
    // but for Pawns we return number of attacked squares
    // There is a dunction called King attackers weight:
    // is the sum of the "weights" of the pieces of the given color which attack a square in the kingRing of the enemy king.
    // i will update this function to return it also
    pub fn king_attackers_count(&self, pins: &Vec<u8>, king_ring_for_pawns:u64,normal_king_ring:u64 ) -> (i32, i32) {
        /*
            King attackers count is the number of pieces of the given color
            which attack a square in the kingRing of the enemy king. 
            For pawns we count number of attacked squares in kingRing.
         */
        let mut c = 0;
        let white_pieces = self.bitboards.get_ally_pieces(Turn::White) & !self.bitboards.white_king;
        if white_pieces == 0 {
            return (0,0);
        }
        
        // Special Case For Pawns is duo pawns, if two pawns attacked the same enemy square,
        // each one of them will count as 0.5 rather than 1, i will try another way,
        // i will make bitboard for pawns attaked and before adding another 1 -refer for attack-
        // i will check if it is in the bitboard or not

        
        let mut white_pawns = self.bitboards.white_pawns;

        let mut pawns_attackers:u64 = 0;

        while white_pawns != 0 {
            let square = white_pawns.trailing_zeros() as u8;
            let pawn = 1 << square;
            let pawn_attack = Bitboards::move_north_east(pawn) |Bitboards::move_north_west(pawn);
            let temp = king_ring_for_pawns & pawn_attack;
            pawns_attackers |= temp;
            white_pawns &= white_pawns - 1;
        }
        c += pawns_attackers.count_ones() as i32;

        let mut weights = 0; // For King attackers weight function

        let mut knights = self.bitboards.white_knights;
        let mut bishops = self.bitboards.white_bishops;
        let mut queens = self.bitboards.white_queens;
        let mut rooks = self.bitboards.white_rooks;
        
        while knights != 0 { // For Kinght weight = 81
            let square = knights.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.knight_attack(square_position, pins);
            if attacked_squares & normal_king_ring != 0{
                c += 1;
                weights += 81;
            }
            knights &= knights - 1;
        }
        while bishops != 0 { // For Bishop w = 52
            let square = bishops.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.bishop_xray_attack(pins, square_position);
            if attacked_squares & normal_king_ring != 0{
                c += 1;
                weights += 52;
            }
            bishops &= bishops - 1;
        }
        while rooks != 0 { // 44
            let square = rooks.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.rook_xray_attack(pins, square_position);
            if attacked_squares & normal_king_ring != 0{
                c += 1;
                weights += 44;
            }
            rooks &= rooks - 1;
        }
        while queens != 0 { //10
            let square = queens.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.queen_attack(pins, square_position);
            if attacked_squares & normal_king_ring != 0{
                c += 1;
                weights += 10;
            }
            queens &= queens - 1;
        }
        return (c, weights);
    }

    // Note in King Ring Function
    // it calculate the king area for the other side
    // mean we should calculate the King area square for Blck King
    pub fn king_ring(&self, full: bool) -> u64 { //untested, should probably be called at the very start and stored as a struct fields because it's used multiple times
        let a_file_mask : u64 = 0x0101010101010101;
        let h_file_mask : u64 = 0x8080808080808080;
        let rank_1_mask : u64 = 0x00000000000000ff;
        let rank_8_mask : u64 = 0xff00000000000000;
        
        let mut king_ring_bitboard: u64 = 0;
        let mut king_bitboard = self.bitboards.black_king;

        // the logic should be make area of king ring wit 1 in bitboard then based on full condition i will rempve protected by pawns

        if king_bitboard & rank_1_mask != 0{
            king_bitboard = king_bitboard << 8;
        }
        if king_bitboard & rank_8_mask != 0 {
            king_bitboard = king_bitboard >> 8;
        }
        if king_bitboard & a_file_mask != 0{
            king_bitboard = king_bitboard << 1;
        }
        if king_bitboard & h_file_mask != 0 {
            king_bitboard = king_bitboard >> 1;
        }

        king_ring_bitboard |= king_bitboard; //assuming there's only one king
        king_ring_bitboard |= Bitboards::move_east(king_bitboard);
        king_ring_bitboard |= Bitboards::move_west(king_bitboard);
        king_ring_bitboard |= Bitboards::move_north(king_bitboard);
        king_ring_bitboard |= Bitboards::move_south(king_bitboard);
        king_ring_bitboard |= Bitboards::move_north_east(king_bitboard);
        king_ring_bitboard |= Bitboards::move_north_west(king_bitboard);
        king_ring_bitboard |= Bitboards::move_south_east(king_bitboard);
        king_ring_bitboard |= Bitboards::move_south_west(king_bitboard);
        
        if full == false { // When full=false, checks if the square is defended by two black pawns diagonally below it
            let not_g_h_file : u64 = 0x3F3F3F3F3F3F3F3F;

            let mut pawn_bitboard = self.bitboards.black_pawns;
            let black_pawns = pawn_bitboard;
            let mut protected_by_2_pawns_bitboard = 0 as u64;
            while pawn_bitboard != 0 {
                let square = pawn_bitboard.trailing_zeros() as u8;
                let pawn = 1 << square;
                if pawn & not_g_h_file != 0 {
                    let neighbour_pawn = pawn << 2; // check if there's a pawn to the right of the pawn by 2
                    if black_pawns & neighbour_pawn !=0 { // as i from black prespective i will move south east 
                        protected_by_2_pawns_bitboard |= pawn >> 7;
                    }
                }
                pawn_bitboard &= pawn_bitboard -1;
            }
            king_ring_bitboard = king_ring_bitboard & !protected_by_2_pawns_bitboard; // removes positions protected by 2 pawns , notand operation -> !&

            return king_ring_bitboard // still haven't decided how i'll use the bitboard
        }else{
            return king_ring_bitboard;
        }
        
    }
    fn rook_xray_attack(&self, pins : &Vec<u8>, rook_bitboard:u64) -> u64 {
        let squares = self.get_rook_xray_attacked_squares(&rook_bitboard);
        let rook_square = rook_bitboard.trailing_zeros() as u8;
        let legal_rook_bitboard = self.get_legal_bitboard(&rook_square, pins, &squares);
        return legal_rook_bitboard;
    }

    fn bishop_xray_attack(&self, pins : &Vec<u8>, bishop_bitboard:u64) -> u64 {
        let squares = self.get_bishop_xray_attacked_squares(&bishop_bitboard);
        let bishop_square = bishop_bitboard.trailing_zeros() as u8;
        let legal_bishop_bitboard = self.get_legal_bitboard(&bishop_square, pins, &squares);
        return legal_bishop_bitboard;
    }

    
    fn queen_attack(&self,pins : &Vec<u8>, queen_bitboard:u64) -> u64 {
        let queen_attacked_squares = self.get_queen_attacked_squares_for_eval(&queen_bitboard);
        let queen_square = queen_bitboard.trailing_zeros() as u8;
        let legal_queen_bitboard = self.get_legal_bitboard(&queen_square, &pins, &queen_attacked_squares);
        return legal_queen_bitboard;    
    }

    fn bishop_on_king_ring(&self) ->i32 {
        
        0
    }

    fn rook_on_file(&self) -> i32 { //not tested
        let mut sum =0;
        let all_pawn_bitboard = self.bitboards.white_pawns | self.bitboards.black_pawns;
        let mut rook_bitboard = self.bitboards.white_rooks;
        while rook_bitboard != 0 {
            let square = rook_bitboard.trailing_zeros() as u8;
            let file_mask = Bitboards::file_mask_to_end(square);
            let num_of_pawns_masked = file_mask & all_pawn_bitboard;
            sum += 2 - (num_of_pawns_masked.count_ones() as i32);
            rook_bitboard &= rook_bitboard -1;
        }
        sum
    }

    fn trapped_rook(&self, pins : &Vec<u8>) -> i32 { //untested, probably doesn't work
        
        0
    }

    fn weak_queen(&self) -> i32 { // ask farouk
        let queen_bitboard = self.bitboards.white_queens;
        let mut sum = 0;
        while queen_bitboard != 0 {
            let queen_idx = queen_bitboard.trailing_zeros() as u8;
            let queen: u64 = 1u64 << queen_idx;
            //?????????? pins or something
        }
        sum
    }

    fn queen_infiltration(&self) -> i32 {
        let queen_bitboard = self.bitboards.white_queens;
        let mut sum = 0;
        let upper_board_half_mask : u64 = 0xFFFFFFFF00000000;
        let enemy_attack_span = self.pawn_attacks_span();
        while queen_bitboard != 0 {
            let queen_idx = queen_bitboard.trailing_zeros() as u8;
            let queen: u64 = 1u64 << queen_idx;
            if ((queen & upper_board_half_mask) & !(enemy_attack_span)) != 0 {
                sum += 1;
            }
        }
        sum
    }

    fn king_protector(&self) -> i32 {
        // let mut knights_bishops_bitboard = self.bitboards.white_bishops | self.bitboards.white_knights;
        let mut knights_bitboard = self.bitboards.white_knights;
        let mut bishops_bitboard = self.bitboards.white_bishops;
        let mut sum = 0;
        let king_bitboard = self.bitboards.white_king;
        let king_idx = king_bitboard.trailing_zeros() as u8;
        let king_file = Square::from(king_idx).file();
        let king_rank = Square::from(king_idx).rank();

        while knights_bitboard != 0 {
            let knight_idx = knights_bitboard.trailing_zeros() as u8;
            // let square = 1u64 << knight_idx;
            let square_rank = Square::from(knight_idx).rank();
            let square_file = Square::from(knight_idx).file();
            let file_dist = (king_file as i32 - square_file as i32).abs();
            let rank_dist = (king_rank as i32 - square_rank as i32).abs();
            if rank_dist > file_dist {
                sum += 8 * rank_dist
            } else {
                sum += 8 * file_dist
            }
            knights_bitboard &= knights_bitboard -1;
        }

        while bishops_bitboard != 0 {
            let bishop_idx = bishops_bitboard.trailing_zeros() as u8;
            // let square = 1u64 << knight_idx;
            let square_rank = Square::from(bishop_idx).rank();
            let square_file = Square::from(bishop_idx).file();
            let file_dist = (king_file as i32 - square_file as i32).abs();
            let rank_dist = (king_rank as i32 - square_rank as i32).abs();
            if rank_dist > file_dist {
                sum += 6 * rank_dist
            } else {
                sum += 6 * file_dist
            }
            bishops_bitboard &= bishops_bitboard -1;
        }

        sum
    }

    fn king_distance(&self) -> i32 {
        
        0
    }

    fn long_diagonal_bishop(&self) -> i32 { //untested
        let mut sum = 0;
        let center_squares_diagonal_mask = 8142240000244281 as u64;
        let long_diagonal_bishops = center_squares_diagonal_mask & self.bitboards.white_bishops;
        sum = long_diagonal_bishops.count_ones();
        sum as i32
    }

    // MOBILITY MIDDLE GAME

    pub fn mobility_mg(&self, pins: &Vec<u8>) -> i32 {
        // sum function
        
        self.mobility_bonus(true, pins)
    }

    fn mobility_eg(&self, pins: &Vec<u8>) -> i32 {
        self.mobility_bonus(false, pins)
    }
    
    fn mobility_bonus(&self, is_middle_game: bool, pins: &Vec<u8>) -> i32 {
        let bonus: Vec<Vec<i32>> = if is_middle_game {
            vec![
                vec![-62, -53, -12, -4, 3, 13, 22, 28, 33], // Knight
                vec![-48, -20, 16, 26, 38, 51, 55, 63, 63, 68, 81, 81, 91, 98], // Bishop
                vec![-60, -20, 2, 3, 3, 11, 22, 31, 40, 40, 41, 48, 57, 57, 62], // Rook
                vec![-30, -12, -8, -9, 20, 23, 23, 35, 38, 53, 64, 65, 65, 66, 67, 67, 72, 72, 77, 79, 93, 108, 108, 108, 110, 114, 114, 116], // Queen
            ]
        } else {
            vec![
                vec![-81, -56, -31, -16, 5, 11, 17, 20, 25], // Knight
                vec![-59, -23, -3, 13, 24, 42, 54, 57, 65, 73, 78, 86, 88, 97], // Bishop
                vec![-78, -17, 23, 39, 70, 99, 103, 121, 134, 139, 158, 164, 168, 169, 172], // Rook
                vec![-48, -30, -7, 19, 40, 55, 59, 75, 78, 96, 96, 100, 121, 127, 131, 133, 136, 141, 147, 150, 151, 168, 168, 171, 182, 182, 192, 219], // Queen
            ]
        };

        let mut total_bonus = 0;

        // Loop through each piece type
        let piece_types = [
            (self.bitboards.white_knights, 0, Piece::Knight),
            (self.bitboards.white_bishops, 1, Piece::Bishop),
            (self.bitboards.white_rooks, 2, Piece::Rook),
            (self.bitboards.white_queens, 3, Piece::Queen),
        ];
        let mobility_area = self.mobility_area();
        for (bitboard, piece_index, piece_type) in piece_types.iter() {
            let mut pieces = *bitboard;
            while pieces != 0 {
                let square = pieces.trailing_zeros() as u64; // Get square position
                let square_position = 1 << square;

                let mobility_index = self.mobility(*piece_type, square_position, mobility_area, pins); // Call mobility function
                if mobility_index < bonus[*piece_index].len(){
                    total_bonus += bonus[*piece_index][mobility_index];
                }
                pieces &= pieces - 1; // Remove piece from bitboard
            }
        }
        total_bonus
    }
    
    fn get_black_pawn_protected_squares(&self) -> u64 {
        let not_a_file = 0xfefefefefefefefe;
        let not_h_file = 0x7f7f7f7f7f7f7f7f;

        // Black pawns attack southwest (↓↙) and southeast (↓↘)
        let right_attacks = (self.bitboards.black_pawns >> 7) & not_a_file;
        let left_attacks = (self.bitboards.black_pawns >> 9) & not_h_file;

        right_attacks | left_attacks
    }

    fn get_white_pawns_on_rank(&self, rank: u8) -> u64 {
        // Rank is 0-based (0=rank1, 1=rank2, ..., 7=rank8)
        let rank_mask = 0xFF << (rank * 8);
        self.bitboards.white_pawns & rank_mask
    }
    
    fn get_blocked_white_pawns(&self) -> u64 {
        let mut blocked_pawns = 0;
        let white_pawns = self.bitboards.white_pawns;
        let all_pieces = !self.bitboards.get_empty_squares(); // All occupied squares

        // For each white pawn, check if the square directly in front is occupied
        let mut pawns = white_pawns;
        while pawns != 0 {
            let pawn_square = pawns.trailing_zeros() as u8;
            let square_in_front = pawn_square + 8; // White pawns move north (+8)

            // Only check if the square in front is on the board (not promotion rank)
            if square_in_front < 64 && (all_pieces & (1 << square_in_front)) != 0 {
                blocked_pawns |= 1 << pawn_square;
            }

            pawns &= pawns - 1; // Clear the least significant bit
        }

        blocked_pawns
    }
    
    pub fn mobility_area(&self) -> u64 {
        let mut mobility_area: u64 = !0; // Start with all bits set to 1.
    
        // Set the king's square to 0
        mobility_area &= !self.bitboards.white_king;
    
        // Set the queen's square to 0
        mobility_area &= !self.bitboards.white_queens;
    
        // Exclude squares protected by enemy pawns
        let protected_squares = self.get_black_pawn_protected_squares();
        mobility_area &= !protected_squares;

        // Exclude white pawns on ranks 2 and 3
        // Rank 2 (human-readable) is index 1 (0-based)
        // Rank 3 (human-readable) is index 2 (0-based)
        let white_pawns_ranks_2_3 =  self.get_white_pawns_on_rank(1) | self.get_white_pawns_on_rank(2);        mobility_area &= !white_pawns_ranks_2_3;
        mobility_area &= !white_pawns_ranks_2_3;

        // Exclude blocked white pawns
        let blocked_white_pawns = self.get_blocked_white_pawns();
        mobility_area &= !blocked_white_pawns;
        
        // the function called blockers_for_king is not important
        // as i will use my existance checks_and_pins function
        // Create bitboard of all pinned pieces
        let pins = self.blockers_for_king();
        let mut pinned_bitboard = 0u64;
        for &square in &pins {
            //println!("{}", square);
            pinned_bitboard |= 1 << square;
        }
        mobility_area &= !pinned_bitboard;
        
        // Return the updated mobility area bitboard
        mobility_area
    }
    
    pub fn mobility(&self, piece: Piece, square_position: u64, mobility_area: u64, pins: &Vec<u8>) -> usize {
        match piece{
            Piece::Knight => {
                let knight_attack = self.knight_attack(square_position, pins);
                return (knight_attack & mobility_area).count_ones() as usize;
            },
            Piece::Bishop => {
                let bishop_attack = self.bishop_xray_attack(pins, square_position);
                return (bishop_attack & mobility_area).count_ones() as usize;

            },
            Piece::Rook => {
                let rook_attack = self.rook_xray_attack(pins, square_position);
                return (rook_attack & mobility_area).count_ones() as usize;
            },
            Piece::Queen => {
                let queen_attack = self.queen_attack(pins, square_position);
                return (queen_attack & mobility_area).count_ones() as usize;
            },
            Piece::Pawn =>{
                return 0;
            },
            Piece::King =>{
                return 0;
            }
        };
    }
    
    // return number of accessible squares by one knight at a time 
    fn knight_attack(&self, knight : u64, pins: &Vec<u8>) -> u64{
        // in stock fish this function returns all possible squares the knight can reach
        // but if the knight is pinned so it is ignored 
        // First check if this knight is pinned
        let knight_square = knight.trailing_zeros() as u8;
        
        // If knight is in the pins vector, return 0 (pinned knights can't move)
        if pins.contains(&knight_square) {
            return 0;
        }
        else {            
            let attacks = self.get_knight_attacked_squares(knight);
            return attacks;
        }
    }
    // THREATS MIDDLE GAME
    
    fn threats_mg(&self) -> i32 {
        let mut v = 0;

        v += 69 * self.hanging();
        v += self.king_threat();
        v += 48 * self.pawn_push_threat();
        v += 173 * self.threat_safe_pawn();
        v += 60 * self.slider_on_queen();
        v += 16 * self.knight_on_queen();
        v += 7 * self.restricted();
        v += 14 * self.weak_queen_protection();
        
        // iterate over board to check for minor threat function and rook threat function

        v
    }

    fn threats_eg(&self) -> i32 {
        let mut v: i32 =0;
        v += 36 * self.hanging();
        v += 89* self.king_threat();
        v += 39 * self.pawn_push_threat();
        v += 94 * self.threat_safe_pawn();
        v += 18 * self.slider_on_queen();
        v += 11 * self.knight_on_queen();
        v += 7 * self.restricted();
        //minor_threat
        //rook_threat
        v
    }
    
    fn hanging(&self) -> i32 {
        // sum function

        if self.weak_enemies() == 0{
            return 0;
        }
        
        // check for attacks 

        0
    }
    
    fn weak_enemies(&self) -> i32 {
        // sum function

        // check if the square is protected or not, if protected return 0, if not return 1
        // 
        
        // check for attacks, attacks functions are a lot, remember that.

        0
    }
    
    fn king_threat(&self) -> i32 {
        // sum function

        // if square is not enemy piece return 0

        if self.weak_enemies() == 0{
            return 0;
        }
        
        // check for king attack 

        0
    }
    
    fn pawn_push_threat(&self) -> i32 {
        // sum function

        // is square is not enemy return 0

        // iterate to check if there is a pawn push threat
        // 

        // else
        
        0
    }
    
    fn threat_safe_pawn(&self) -> i32 {
        // sum function

        // if square not enemy pawn return 0

        // check pawn attack function

        // check safe pawn function

        0
    }

    fn safe_pawn(&self) -> i32 {
        // sum function

        // if square is not enemy pawn return 0

        // check attacks funciton

        0
    }

    // this function sees if i can gain tempo on a queen using a slider piece
    fn slider_on_queen(&self) -> i32 {
        // sum function

        // check for enemy pawn

        // check for attack function
        
        // check for mobility area funciton

        //check for diagonal attacks and xray attacks

        0
    }
    
    // this function sees if i can gain tempo on a queen using a knight
    fn knight_on_queen(&self) -> i32 {
        // sum function

        0
    }

    fn restricted(&self) -> i32 {
        // sum function

        // check attack function

        // check attack function for opposite color

        0
    }

    fn weak_queen_protection(&self) -> i32 {
        // sum function

        if self.weak_enemies() == 0{
            return 0;
        }
        
        // check queen attacks function

        0
    }

    // PASSED MIDDLE GAME

    fn passed_mg(&self) -> i32 {
        // sum function

        // check for passed leverable function

        let mut v = 0;

        // add bonus depending on the passed pawn rank

        // sub bonus depending on the passed pawn file

        v
    }

    fn passed_eg(&self) -> i32 {
        
        // if !self.passed_leverable() {
        //     return 0;
        // }
        let mut v: i32 = 0;


        v
    }

    // SPACE FUNCTION

    // this function calculate how much space a side has
    pub fn space(&self,is_middle_game: bool) -> i32 {
        if self.non_pawn_material(is_middle_game) + self.color_flip().non_pawn_material(is_middle_game) < 12222 {
            return 0
        }
        let piece_count = self.bitboards.get_ally_pieces(Turn::White).count_ones() as i32;
        let mut blocked= 0;
        
        let mut white_pawn_bitboard = self.bitboards.white_pawns;
        while white_pawn_bitboard != 0 {
            let pawn  = white_pawn_bitboard.trailing_zeros() as u64;
            if (Square::from(pawn as u8).rank() as usize) < 7 {
                if ((1u64 << pawn) << 8) & self.bitboards.black_pawns != 0{
                    blocked +=1;
                    // continue;
                } 
            }
            if (Square::from(pawn as u8).rank() as usize) < 6 {
                if (Square::from(pawn as u8).file() as usize) > (0 as usize) && (Square::from(pawn as u8).file() as usize) < 7{
                    if ((((1u64 << pawn) << 15) & self.bitboards.black_pawns !=0 ) && (((1u64 << pawn) << 17) & self.bitboards.black_pawns != 0)){
                        blocked+=1;
                        // continue;
                    }
                } 
            }
            white_pawn_bitboard &= white_pawn_bitboard -1;
        }

        let mut black_pawn_bitboard = self.bitboards.black_pawns;
        while black_pawn_bitboard != 0 {
            let pawn  = black_pawn_bitboard.trailing_zeros() as u64;
            if (Square::from(pawn as u8).rank() as usize) > 0 {
                if ((1u64 << pawn) >> 8 ) & self.bitboards.white_pawns != 0{
                    blocked +=1;
                    // continue;
                } 
            }
            if (Square::from(pawn as u8).rank() as usize) > 1 {
                if (Square::from(pawn as u8).file() as usize) > (0 as usize) && (Square::from(pawn as u8).file() as usize) < 7{
                    if ((((1u64 << pawn) >> 15) & self.bitboards.white_pawns != 0) && (((1u64 << pawn) >> 17) & self.bitboards.white_pawns != 0)){
                        blocked+=1;
                        // continue;
                    }
                } 
            }
            black_pawn_bitboard &= black_pawn_bitboard -1;
        }

        let enemy_pawn_attacks = {
            let not_a_file : u64 = 0xfefefefefefefefe;
            let not_h_file : u64 = 0x7f7f7f7f7f7f7f7f;
            let right_capture = (self.bitboards.black_pawns >> 7) & not_a_file;
            let left_capture = (self.bitboards.black_pawns >> 9) & not_h_file;                
            right_capture | left_capture
        };

        let default_space : u64 = 0x000000003C3C3C00;
        let mut space = (default_space & !self.bitboards.white_pawns); // removes pawns from space

        space = space & !enemy_pawn_attacks; //remove enemy pawn attacks from space

        let mut space_count = space.count_ones();
        let mut pawn_bitboard = self.bitboards.white_pawns;
        let mut behind_pawn_mask : u64 =0;
        while pawn_bitboard != 0 {
            let pawn = pawn_bitboard.trailing_zeros() as u64;
            if (Square::from(pawn as u8).file() as usize) > 1 && (Square::from(pawn as u8).file() as usize) < 6 {
                let mut mask = 1u64 << pawn; 
                let mut count = 0 ;
                while mask > 0xFF && count <3 { // While the mask isn't in rank 1
                    count +=1;
                    mask >>= 8; // Shift down one rank
                    behind_pawn_mask |= mask;
                }
            }
            pawn_bitboard &= pawn_bitboard -1;
        }
        let attacked_squares = {
                enemy_pawn_attacks |
                self.get_knight_attacked_squares(self.bitboards.black_knights) |
                self.get_bishop_attacked_squares(&self.bitboards.black_bishops) |
                self.get_rook_attacked_squares(&self.bitboards.black_rooks) |
                self.get_queen_attacked_squares(&self.bitboards.black_queens) |
                self.get_king_attacked_squares(self.bitboards.black_king)
        };
        
        let double_count = (space & behind_pawn_mask & !attacked_squares);

        space_count += double_count.count_ones();
        let weight = piece_count - 3 + i32::min(blocked, 9);
        let sum = (space_count as i32) * weight * weight / 16 ;

        sum
    }

    pub fn print_bitboard_raw(&self,bb: u64) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bb >> square) & 1;
                print!("{}", bit);
            }
            println!();
        }
    }
    
    // KING MIDDLE GAME

    fn king_mg(&self, pins: &Vec<u8>, flip_pins: &Vec<u8>) -> i32 {
        let mut v = 0;
        let kd = self.king_danger(pins, flip_pins);
        
        let (strength_arr,_) = self.strength_square();
        let (storm_arr,_) = self.storm_square(false);
        let (shelter_strength,shelter_storm) = self.shelter_strength_and_storm(strength_arr, storm_arr); 
        v -= shelter_strength;
        v += shelter_storm;
        v += kd * kd / 4096;
        v += 8 * self.flank_attack(pins);
        v += 17 * self.pawnless_flank();
        
        v
    }

    fn king_eg(&self, pins: &Vec<u8>, flip_pins: &Vec<u8>) -> i32 {
        let mut v: i32 =0;

        v
    }
    
    // check if the king is in danger, or can be in danger
    fn king_danger(&self, pins: &Vec<u8>, flip_pins: &Vec<u8>) -> i32 {
        // this is a big function with a lot of branches 
        let king_ring_for_pawns = self.king_ring(true);
        let normal_king_ring = self.king_ring(false);
        
        let (count, weight) = self.king_attackers_count(pins, king_ring_for_pawns,normal_king_ring );
        let king_attacks = self.king_attacks(count, pins);
        let weak = self.weak_bonus(normal_king_ring, pins, flip_pins);
        0
    }
    /*
     This function evaluates weak squares in the enemy king's ring from White's perspective. It assigns a bonus (1) if a square meets two conditions:

     It is a weak square (undefended or poorly defended).

     It is part of the enemy king's ring (critical attack zone around the king).
    */
    // return number of weak bonus
    // Call 2 functions king_ring, weak_squares
    pub fn weak_bonus(&self, normal_king_ring:u64, pins: &Vec<u8>, flip_pins: &Vec<u8>) -> i32{
        
        let weak_squares = self.weak_squares(pins, flip_pins);
        return (weak_squares & normal_king_ring).count_ones() as i32;
    }


    // Function Attack it depends on 
    // King attack - Knight attack - Bishop xray attak - Rook xray attak - Queen Attack 
    // pawn attack -Not implemented but will deal with it-
    // i will return 2 things - bitboard contains ones at aquares attacked by any piece
    // and vector of size 64 at each index contains number of pieces attack this square
    pub fn attack(&self, pins: &Vec<u8>) -> (u64, [u8; 64], i32){
        let mut attack_bitboard: u64 = 0;
        let mut attack_counts = [0u8; 64];
        let mut num_of_attacks = 0;
        // Process king attacks
        let mut king_attacks = self.king_attack();
        attack_bitboard |= king_attacks;
        while king_attacks != 0 {
            let square = king_attacks.trailing_zeros() as u8;
            attack_counts[square as usize] += 1;
            king_attacks &= king_attacks - 1;
            num_of_attacks += 1;
        }

        let mut knights = self.bitboards.white_knights;
        let mut bishops = self.bitboards.white_bishops;
        let mut queens = self.bitboards.white_queens;
        let mut rooks = self.bitboards.white_rooks;
        
        while knights != 0 {
            let square = knights.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let mut attacked_squares = self.knight_attack(square_position, pins);
            attack_bitboard |= attacked_squares;
            while attacked_squares != 0 {
                let square = attacked_squares.trailing_zeros() as u8;
                attack_counts[square as usize] += 1;
                attacked_squares &= attacked_squares - 1;
                num_of_attacks += 1;

            }

            knights &= knights - 1;
        }
        while bishops != 0 { // For Bishop w = 52
            let square = bishops.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let mut attacked_squares = self.bishop_xray_attack(pins, square_position);
            attack_bitboard |= attacked_squares;
            while attacked_squares != 0 {
                let square = attacked_squares.trailing_zeros() as u8;
                attack_counts[square as usize] += 1;
                attacked_squares &= attacked_squares - 1;
                num_of_attacks += 1;

            }
            bishops &= bishops - 1;
        }
        while rooks != 0 { // 44
            let square = rooks.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let mut attacked_squares = self.rook_xray_attack(pins, square_position);
            attack_bitboard |= attacked_squares;
            while attacked_squares != 0 {
                let square = attacked_squares.trailing_zeros() as u8;
                attack_counts[square as usize] += 1;
                attacked_squares &= attacked_squares - 1;
                num_of_attacks += 1;

            }
            rooks &= rooks - 1;
        }
        while queens != 0 { //10
            let square = queens.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let mut attacked_squares = self.queen_attack(pins, square_position);
            attack_bitboard |= attacked_squares;
            while attacked_squares != 0 {
                let square = attacked_squares.trailing_zeros() as u8;
                attack_counts[square as usize] += 1;
                attacked_squares &= attacked_squares - 1;
                num_of_attacks += 1;

            }
            queens &= queens - 1;
        }

        let mut white_pawns = self.bitboards.white_pawns;

        while white_pawns != 0 {
            let square = white_pawns.trailing_zeros() as u8;
            let pawn = 1 << square;
            let mut pawn_attack = Bitboards::move_north_east(pawn) |Bitboards::move_north_west(pawn);

            attack_bitboard |= pawn_attack;
            while pawn_attack != 0 {
                let square = pawn_attack.trailing_zeros() as u8;
                attack_counts[square as usize] += 1;
                pawn_attack &= pawn_attack - 1;
                num_of_attacks += 1;

            }
            white_pawns &= white_pawns - 1;
        }

        return (attack_bitboard, attack_counts, num_of_attacks);
    }


    pub fn king_attack(&self)->u64{
        let king_bitboard = self.bitboards.white_king;
        let mut king_attack = 0;
        king_attack |= Bitboards::move_east(king_bitboard);
        king_attack |= Bitboards::move_west(king_bitboard);
        king_attack |= Bitboards::move_north(king_bitboard);
        king_attack |= Bitboards::move_south(king_bitboard);
        king_attack |= Bitboards::move_north_east(king_bitboard);
        king_attack |= Bitboards::move_north_west(king_bitboard);
        king_attack |= Bitboards::move_south_east(king_bitboard);
        king_attack |= Bitboards::move_south_west(king_bitboard);

        return king_attack;

    }
    // Call Function Attack, King_attack, Queen_attack
    /*
        Weak squares. Attacked squares defended at most once by our queen or king.

        The weak_squares function identifies squares that:

        Are attacked by white -Call Attack function and deal with bitboard returned-

        Are defended by black at most once -so we need to call color flip-

        If defended once, only by black's queen or king (not minor pieces or pawns) -another check- 
     */
    pub fn weak_squares(&self,pins: &Vec<u8>, flip_pins: &Vec<u8>)->u64{ // return bitboard contains 1 at weak squares
        
        /*
        If ≥ 2 defenders → not weak (return 0)

        If 0 defenders → weak (return 1)

        If 1 defender → only weak if defender is queen or king
        
         */

        // Helper function to flip the attack count array
        fn flip_attack_array(arr: [u8; 64]) -> [u8; 64] {
            let mut flipped = [0; 64];
            for i in 0..64 {
                let rank = i / 8;          // Original rank (0-7)
                let file = i % 8;          // File stays the same
                let new_rank = 7 - rank;   // Mirrored rank
                let new_index = new_rank * 8 + file;
                flipped[new_index] = arr[i];
            }
            flipped
        }


        let (white_attacks, _ , _) = self.attack(pins);

        let flipped_bitboard = self.color_flip();
        let (black_defend, black_defend_array, _) = flipped_bitboard.attack(flip_pins);

        let black_defend_correct = self.flip_vertical(black_defend);
        let black_defend_array_correct = flip_attack_array(black_defend_array);

        let mut king_defended_squares = flipped_bitboard.king_attack();
        king_defended_squares = self.flip_vertical(king_defended_squares);
        
        let mut queen_defended_squares = 0;
        
        let mut queen_bitboard = flipped_bitboard.bitboards.white_queens;
        while queen_bitboard != 0  {
            let square = queen_bitboard.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;
            let  attacked_squares = self.queen_attack(flip_pins, square_position);
            queen_defended_squares |= attacked_squares;    
            queen_bitboard &= queen_bitboard - 1;
        }

        queen_defended_squares = self.flip_vertical(queen_defended_squares);
        let mut weak_squares = 0;
        // weak squares --> attacked by white and not defended by black
        weak_squares |= white_attacks & !black_defend_correct;  
        // weak squares --> attacked by white and defended by black
        let mut attack_defend = white_attacks & black_defend_correct;
        // if square defended by 2 or more not weak

        while attack_defend != 0 {
            let square = attack_defend.trailing_zeros() as usize;
            if black_defend_array_correct[square] >= 2 {
                // do nothing
            }
            // if == 0 --> has been handeled above
            if black_defend_array_correct[square] == 1 { // weak if defnder is queen or king
                let square_pos = 1 << (square as u8);
                if square_pos & king_defended_squares != 0 || square_pos & queen_defended_squares != 0{ // 
                    weak_squares |= square_pos;
                }
            }
            attack_defend &= attack_defend - 1;
        }
        weak_squares
    }
    // takes king attackers count if = 0 then return 0
    pub fn king_attacks(&self, count:i32,pins: &Vec<u8> ) -> i32{
        if count == 0 {
            return 0;
        }
        
        let king_bitboard = self.bitboards.black_king;        

        let mut king_squares = 0;
        king_squares |= Bitboards::move_east(king_bitboard);
        king_squares |= Bitboards::move_west(king_bitboard);
        king_squares |= Bitboards::move_north(king_bitboard);
        king_squares |= Bitboards::move_south(king_bitboard);
        king_squares |= Bitboards::move_north_east(king_bitboard);
        king_squares |= Bitboards::move_north_west(king_bitboard);
        king_squares |= Bitboards::move_south_east(king_bitboard);
        king_squares |= Bitboards::move_south_west(king_bitboard);

        let mut c = 0;

        let mut knights = self.bitboards.white_knights;
        let mut bishops = self.bitboards.white_bishops;
        let mut queens = self.bitboards.white_queens;
        let mut rooks = self.bitboards.white_rooks;
        
        while knights != 0 { // For Kinght weight = 81
            let square = knights.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.knight_attack(square_position, pins);
            let attacks = attacked_squares & king_squares;
            if attacks != 0{
                c += attacks.count_ones();
            }
            knights &= knights - 1;
        }
        while bishops != 0 { // For Bishop w = 52
            let square = bishops.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.bishop_xray_attack(pins, square_position);
            let attacks = attacked_squares & king_squares;
            if attacks != 0{
                c += attacks.count_ones();
            }
            bishops &= bishops - 1;
        }
        while rooks != 0 { // 44
            let square = rooks.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.rook_xray_attack(pins, square_position);
            let attacks = attacked_squares & king_squares;
            if attacks != 0{
                c += attacks.count_ones();
            }
            rooks &= rooks - 1;
        }
        while queens != 0 { //10
            let square = queens.trailing_zeros() as u64; // Get square position
            let square_position = 1 << square;

            let attacked_squares = self.queen_attack(pins, square_position);
            let attacks = attacked_squares & king_squares;
            if attacks != 0{
                c += attacks.count_ones();
            }
            queens &= queens - 1;
        }

        return c as i32;
    }
       
    // From Stock Fish the 2 functions does the same thing
    // but shelter return W and storm return S
    // so i will make only 1 function
    pub fn shelter_strength_and_storm(&self, strength_arr:[i32; 64], storm_arr:[i32; 64]) -> (i32, i32) {
        // calculate the pieces sheltring the king
        let mut w = 0;
        let mut s = 1024;

        let mut checked_squares = self.bitboards.black_king;
        if self.castling_rights.black_king_side == true{
            checked_squares |= 1 << 62;
        }
        if self.castling_rights.black_queen_side == true {
            checked_squares |= 1 << 58;
        }
        while checked_squares != 0 {
            let sq = checked_squares.trailing_zeros() as usize;
            let w1 = strength_arr[sq];
            let s1 = storm_arr[sq];
            
            if s1 - w1 < s - w {
                w = w1;
                s = s1;
            }   
            checked_squares &= checked_squares - 1;
        }
        // w (the strength value)
        // s (the storm value)
        (w,s)
    }
    
    
    // This Function has bug if the black pawn at rank 0 but this will
    // not happen in normal play

    pub fn storm_square(&self, eg:bool) -> ([i32; 64], i32){
        let mut storm = [0; 64];
        let mut total = 0;

        // Storm tables matching JavaScript version
        const UNBLOCKED_STORM: [[i32; 7]; 4] = [
            [85, -289, -166, 97, 50, 45, 50],
            [46, -25, 122, 45, 37, -10, 20],
            [-6, 51, 168, 34, -2, -22, -14],
            [-15, -11, 101, 4, 11, -15, -29]
        ];

        const BLOCKED_STORM: [[i32; 7]; 2] = [
            [0, 0, 76, -10, -7, -4, -1],
            [0, 0, 78, 15, 10, 6, 2]
        ];

        // Process each square
        for square in 0..64 {
            let file = (square % 8) as u8;  // 0-7 (a-h)
            let rank = (square / 8) as u8;
            let mut v = 0;
            let mut ev = 5;  // Endgame value starts at 5

            // Adjust king file to b-g (1-6)
            let kx = cmp::min(6, cmp::max(1, file));

            for x in kx-1..=kx+1 {
                if x > 7 { continue; }  // Skip invalid files

                let file_mask = Bitboards::file_mask(x);
                let mut black_pawns = self.bitboards.black_pawns & file_mask;
                let mut white_pawns = self.bitboards.white_pawns & file_mask;

                // Find highest black pawn (unprotected)
                let mut us = 0;
                while black_pawns != 0 {
                    let sq = black_pawns.trailing_zeros() as u8;
                    let pos = 1 << sq;
                    let r = sq / 8;


                    // Check if pawn is unprotected
                    let attack = Bitboards::move_south_east(pos) | Bitboards::move_south_west(pos);
                    if (attack & self.bitboards.white_pawns) == 0 && r <= rank {
                        us = 7 - r;  // Convert to table index
                    }
                    black_pawns &= black_pawns - 1;
                }

                // Find highest white pawn
                let mut them = 0;
                while white_pawns != 0 {
                    let sq = white_pawns.trailing_zeros() as u8;
                    let r = sq / 8;
                    if r <= rank {
                        them = 7 - r;  // Keep the highest valid pawn
                    }
                    white_pawns &= white_pawns - 1;
                }

                let f = cmp::min(x, 7 - x) as usize;

                // Check for blocked case (our pawn directly below theirs)
                if us > 0 && them == us + 1 {
                    v += BLOCKED_STORM[0][them as usize];
                    ev += BLOCKED_STORM[1][them as usize];
                } else {
                    v += UNBLOCKED_STORM[f][them as usize];
                }
            }

            storm[square as usize] = if eg { ev } else { v };
            total += if eg { ev } else { v };
        }

        (storm, total)
    }

    // This Function has bug if the black pawn at rank 0 but this will
    // not happen in normal play

    pub fn strength_square(&self) -> ([i32; 64], i32){
        let mut strength = [0; 64];
        let mut total = 0;
        
        // Weakness table [file][pawn_rank]
        const WEAKNESS: [[i32; 7]; 4] = [
            [-6, 81, 93, 58, 39, 18, 25],
            [-43, 61, 35, -49, -29, -11, -63],
            [-10, 75, 23, -2, 32, 3, -45],
            [-39, -13, -29, -52, -48, -67, -166]
        ];

        // Process each square
        for square in 0..64 {
            let file = (square % 8) as u8;  // 0-7 (a-h)
            let rank = (square / 8) as u8;
            // Base value
            let mut v = 5;

            // Adjust king file to b-g (1-6)
            let kx = cmp::min(6, cmp::max(1, file));

            for x in kx-1..=kx+1{
                let mut us = 0;

                let file_mask = Bitboards::file_mask(x);
                let mut black_pawns_in_this_file = self.bitboards.black_pawns & file_mask;
                
                // Find Highest Unprotected Pawn
                while black_pawns_in_this_file != 0{
                    let square = black_pawns_in_this_file.trailing_zeros() as u8;
                    let pos = 1 << square;

                    let possiple_attack = Bitboards::move_south_east(pos)|Bitboards::move_south_west(pos);
                    if possiple_attack & self.bitboards.white_pawns == 0{
                        let r = square / 8;
                        if r <= rank{
                            us = 7 - r;
                        } 
                    }
                    black_pawns_in_this_file &= black_pawns_in_this_file -1;
                }
                let f = cmp::min(x, 7-x);
                v += WEAKNESS[f as usize][us as usize];
            }            
            strength[square as usize] = v;
            total += v;
        }

        (strength, total)
    }


    // The logic of flank attack will be 1- find flank area 2- get attacks and count number of flank attacks
    pub fn flank_attack(&self, pins: &Vec<u8>) -> i32 {

        // if (square.y > 4) return 0;
        // reject rank 0, 1, 2 from calculations
        let mut flank_area:u64 = 0xFFFFFFFFFF000000;
        let file_h: u64 = 0x8080808080808080;
        let file_g: u64 = 0x4040404040404040;
        let file_f: u64 = 0x2020202020202020;
        let file_e: u64 = 0x1010101010101010;
        let file_d: u64 = 0x0808080808080808;
        let file_c: u64 = 0x0404040404040404;
        let file_b: u64 = 0x0202020202020202;
        let file_a: u64 = 0x0101010101010101;

        let black_king_file = (self.bitboards.black_king.trailing_zeros() as u8) % 8;

        if black_king_file == 0 {
            flank_area = flank_area & (file_a | file_b | file_c);
        }else if black_king_file == 1 || black_king_file == 2 {
            flank_area = flank_area & (file_a | file_b | file_c | file_d);
        }else if  black_king_file == 3 || black_king_file == 4{
            flank_area = flank_area & (file_c | file_d | file_e | file_f);
        }else if  black_king_file == 5 || black_king_file == 6{
            flank_area = flank_area & (file_e | file_f | file_g | file_h);
        }else if black_king_file == 7{
            flank_area = flank_area & (file_f | file_g | file_h);
        }
        let (attacks, attacks_array, _) = self.attack(pins);

        let mut checked_area = flank_area & attacks;

        let mut c = 0;

        while checked_area != 0 {
            
            let square = checked_area.trailing_zeros() as usize;
            if attacks_array[square] == 0{
                c += 0;
            }else if  attacks_array[square] == 1{
                c += 1;
            }else {
                c+=2;
            }
            checked_area &= checked_area - 1;
        }
        c
    }
    
    pub fn pawnless_flank(&self) -> i32 {
        // return 0 or 1
        /*
            Far Queenside (kx=0, a-file): Checks pawns on a,b,c.

            Queenside (kx=1-2, b/c-file): Checks pawns on a,b,c,d.

            Center (kx=3-4, d/e-file): Checks pawns on c,d,e,f.

            Kingside (kx=5-6, f/g-file): Checks pawns on e,f,g,h.

            Far Kingside (kx=7, h-file): Checks pawns on f,g,h.
        */
        let king_file = (self.bitboards.black_king.trailing_zeros() as u8) % 8; // 0 -> 7
        // Get all pawns (both colors)
        let all_pawns = self.bitboards.white_pawns | self.bitboards.black_pawns;
        // Define flank masks based on king's file
        let flank_mask: u64 = match king_file {
            0 => 0x707070707070707, // a,b,c files (0x01 | 0x02 | 0x04)
            1 | 2 => 0x0F0F0F0F0F0F0F0F, // a,b,c,d files
            3 | 4 => 0x3C3C3C3C3C3C3C3C, // c,d,e,f files
            5 | 6 => 0xF0F0F0F0F0F0F0F0, // e,f,g,h files
            7 => 0xE0E0E0E0E0E0E0E0, // f,g,h files
            _ => 0,
        };
        // println!("{} THE {} Pawns {}",king_file, flank_mask, all_pawns);
        // Check if any pawn exists in the flank area
        if (all_pawns & flank_mask) != 0 {
            0  // Pawns exist in flank -> No Penalty
        } else {
            1  // No pawns in flank (pawnless) -> Penaly
        }
    }


    fn winnable_total_mg(&self, v: Option<i32>) -> i32 { //this is wrong
        let v = if let Some(v) = v {
            let ret = if v > 0 {
                1
            }else if v < 0 {
                -1
            }else {
                0 
            };
            
            ret
        }else {

            let v = self.middle_game_evaluation(true);

            let ret = if v > 0 {
                1
            }else if v < 0 {
                -1
            }else {
                0 
            };
            
            ret
        };


        return v * i32::max(i32::min(self.winnable() + 50, 0), -i32::abs(v));
    }
    
    fn winnable_total_eg(&self, v: Option<i32>) -> i32 {
        let v: i32 = 0;

        0
    }

    fn winnable(&self) -> i32 {

        0
    }

    /*
        this logic is correct and tested using psqt_bonus function
        Yousse, Please check it again and look for performance and you can test it with other function,
        then update the code, remove Turn:White or Turn:Black from code

     */
    // Function to flip the board vertically while keeping columns intact
    pub fn color_flip(&self) -> Self {
        // match self.turn {
        //     Turn::White =>{
        //         println!("White");
        //     },
        //     Turn::Black =>{
        //         println!("Black");
        //     },
        // };
        
        // Clone the current board
        let mut clone_board = self.clone();


        // match clone_board.turn {
        //     Turn::White =>{
        //         println!("White");
        //     },
        //     Turn::Black =>{
        //         println!("Black");
        //     },
        // };

        // The turn is White and still white

        // match clone_board.turn {
        //     Turn::White =>{
        //         println!("White");
        //     },
        //     Turn::Black =>{
        //         println!("Black");
        //     },
        // };

        // The turn is White and still white

        // Flip the bitboards correctly
        clone_board.bitboards.white_pawns = self.flip_vertical(self.bitboards.black_pawns);
        clone_board.bitboards.white_bishops = self.flip_vertical(self.bitboards.black_bishops);
        clone_board.bitboards.white_knights = self.flip_vertical(self.bitboards.black_knights);
        clone_board.bitboards.white_rooks = self.flip_vertical(self.bitboards.black_rooks);
        clone_board.bitboards.white_queens = self.flip_vertical(self.bitboards.black_queens);
        clone_board.bitboards.white_king = self.flip_vertical(self.bitboards.black_king);

        clone_board.bitboards.black_pawns = self.flip_vertical(self.bitboards.white_pawns);
        clone_board.bitboards.black_bishops = self.flip_vertical(self.bitboards.white_bishops);
        clone_board.bitboards.black_knights = self.flip_vertical(self.bitboards.white_knights);
        clone_board.bitboards.black_rooks = self.flip_vertical(self.bitboards.white_rooks);
        clone_board.bitboards.black_queens = self.flip_vertical(self.bitboards.white_queens);
        clone_board.bitboards.black_king = self.flip_vertical(self.bitboards.white_king);

        // We Should Take care of Castling also

        clone_board.castling_rights.white_king_side= self.castling_rights.black_king_side;
        clone_board.castling_rights.white_queen_side= self.castling_rights.black_queen_side;
        clone_board.castling_rights.black_king_side= self.castling_rights.white_king_side;
        clone_board.castling_rights.black_queen_side= self.castling_rights.white_queen_side;

        
        
        
        // Return the modified cloned board
        clone_board
    }

    pub fn flip_vertical(&self, bb: u64) -> u64 {
        let mut flipped = 0;
        for rank in 0..8 {
            let rank_bits = (bb >> (rank * 8)) & 0xFF;
            flipped |= rank_bits << ((7 - rank) * 8);
        }
        flipped
    }

    /*
        Using reverse_bits method logic
        the logic is not corredt as in flipping we should make vertical flipping
        means if a black pawn at a7 it must be turned to a1, which means we should keep the column the same
        then flipping rows only
        8 <-> 1
        7 <-> 2
        6 <-> 3
        5 <-> 4
        
        this logic do this correctly but also do horizontal flipping also, example:
        
        White:♚ - Black:♔

        8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
        7 ♙ ♙ ♙ . ♙ . ♙ ♙
        6 . . . ♙ . . . .
        5 . ♟ . . . . . ♟
        4 . . . ♟ . ♙ . .
        3 ♟ . ♟ . . ♟ . .
        2 . . . . . ♟ . ♟
        1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        a b c d e f g h

        -------------------------

        White:♚ - Black:♔

        8 ♖ ♘ ♗ ♔ ♕ ♗ ♘ ♖
        7 ♙ . ♙ . . . . .
        6 . . ♙ . . ♙ . ♙
        5 . . ♟ . ♙ . . .
        4 ♙ . . . . . ♙ .
        3 . . . . ♟ . . .
        2 ♟ ♟ . ♟ . ♟ ♟ ♟
        1 ♜ ♞ ♝ ♚ ♛ ♝ ♞ ♜
        a b c d e f g h

     */
    // pub fn color_flip(&self) -> Self {
    //     // Clone the current board
    //     let mut clone_board = self.clone();

    //     // Flip the bitboards vertically using reverse_bits
    //     clone_board.bitboards.white_pawns = self.bitboards.black_pawns.reverse_bits();
    //     clone_board.bitboards.white_bishops = self.bitboards.black_bishops.reverse_bits();
    //     clone_board.bitboards.white_knights = self.bitboards.black_knights.reverse_bits();
    //     clone_board.bitboards.white_rooks = self.bitboards.black_rooks.reverse_bits();
    //     clone_board.bitboards.white_queens = self.bitboards.black_queens.reverse_bits();
    //     clone_board.bitboards.white_king = self.bitboards.black_king.reverse_bits();

    //     clone_board.bitboards.black_pawns = self.bitboards.white_pawns.reverse_bits();
    //     clone_board.bitboards.black_bishops = self.bitboards.white_bishops.reverse_bits();
    //     clone_board.bitboards.black_knights = self.bitboards.white_knights.reverse_bits();
    //     clone_board.bitboards.black_rooks = self.bitboards.white_rooks.reverse_bits();
    //     clone_board.bitboards.black_queens = self.bitboards.white_queens.reverse_bits();
    //     clone_board.bitboards.black_king = self.bitboards.white_king.reverse_bits();

    //     // Return the modified cloned board
    //     clone_board
    // }

    /*
        Youssef Logic
        the logic is not corredt as in flipping we should make vertical flipping
        means if a black pawn at a7 it must be turned to a1, which means we should keep the column the same
        then flipping rows only
        8 <-> 1
        7 <-> 2
        6 <-> 3
        5 <-> 4

     */
    // pub fn color_flip(&self) -> Self {
    //     let mut clone_board = self.clone();

    //     // Swap pawns
    //     let temp = clone_board.bitboards.white_pawns;
    //     clone_board.bitboards.white_pawns = clone_board.bitboards.black_pawns;
    //     clone_board.bitboards.black_pawns = temp;

    //     // Swap knights
    //     let temp = clone_board.bitboards.white_knights;
    //     clone_board.bitboards.white_knights = clone_board.bitboards.black_knights;
    //     clone_board.bitboards.black_knights = temp;

    //     // Swap bishops
    //     let temp = clone_board.bitboards.white_bishops;
    //     clone_board.bitboards.white_bishops = clone_board.bitboards.black_bishops;
    //     clone_board.bitboards.black_bishops = temp;

    //     // Swap rooks
    //     let temp = clone_board.bitboards.white_rooks;
    //     clone_board.bitboards.white_rooks = clone_board.bitboards.black_rooks;
    //     clone_board.bitboards.black_rooks = temp;

    //     // Swap queens
    //     let temp = clone_board.bitboards.white_queens;
    //     clone_board.bitboards.white_queens = clone_board.bitboards.black_queens;
    //     clone_board.bitboards.black_queens = temp;

    //     // Swap kings
    //     let temp = clone_board.bitboards.white_king;
    //     clone_board.bitboards.white_king = clone_board.bitboards.black_king;
    //     clone_board.bitboards.black_king = temp;

    //     clone_board
    // }

    fn phase(&self, is_middle_game: bool) -> i32 {
        let mid_game_limit = 15258;
        let end_game_limit = 3915;
        let mut npm = self.non_pawn_material(is_middle_game) + self.color_flip().non_pawn_material(is_middle_game);
        npm = i32::max(end_game_limit, i32::min(npm, mid_game_limit));
        ((npm - end_game_limit) * 128) / (mid_game_limit - end_game_limit)
    }

}
