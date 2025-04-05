use std::process::id;
use crate::piece::Piece;

use crate::{bitboards::{self, Bitboards}, board::{Board, Turn}, square::{self, Rank, Square}};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mg = self.middle_game_evaluation(true);
        mg
    }
    
    fn middle_game_evaluation(&self, nowinnable: bool) -> i32 {
        let mut v = 0;
        let color_flip_board = self.color_flip();
        let (_, pins) = self.checks_and_pins();
        let (_, flip_pins) = self.checks_and_pins();

        v += self.piece_value_mg() - color_flip_board.piece_value_mg();
        v += self.psqt_mg() - color_flip_board.psqt_mg();
        v += self.imbalance_total(&color_flip_board);
        v += self.pawns_mg() - color_flip_board.pawns_mg(); 
        v += self.pieces_mg() - color_flip_board.pieces_mg();
        v += self.mobility_mg(&pins) - color_flip_board.mobility_mg(&flip_pins);
        v += self.threats_mg() - color_flip_board.threats_mg();
        v += self.passed_mg() - color_flip_board.passed_mg();
        v += self.space() - color_flip_board.space();
        v += self.king_mg() - color_flip_board.king_mg();
        
        if !nowinnable {
            v += self.winnable_total_mg(Some(v));
        }

        v
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
    fn backward(&self,square_position: u64, square: u8) -> i32 {
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

    fn pieces_mg(&self) -> i32 {
        let mut v = 0;
        v += [0, 31, -7, 30, 56][self.outpost_total() as usize];
        v += 18 * self.minor_behind_pawn();
        v -= 3 * self.bishop_pawns();
        v -= 4 * self.bishop_xray_pawns();
        v += 6 * self.rook_on_queen_file();
        v += 16 * self.rook_on_king_ring();
        v += self.rook_on_file();
        v -= self.trapped_rook() * 55 ; //idk incomplete for now
        v -= 56 * self.weak_queen();
        v -= 2 * self.queen_infiltration();
        //king protector line idk
        v += 45 * self.long_diagonal_bishop();
        return v;
    }

    fn outpost_total(&self) -> i32 {
    
        0
    }

    fn outpost(&self) -> i32 {
    
        0
    } 

    fn outpost_square(&self) -> i32 {
        //needs Rank function, might be able to fix from bitboards rank 
        0
    }

    fn pawn_attacks_span(&self) -> i32 {
    
        0
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

    fn bishop_pawns(&self) -> i32 {

        0
    }

    fn pawn_attack(&self) -> i32 { //might be able to remove and replace 

        0
    }

    fn bishop_xray_pawns(&self) -> i32 {

        0
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

    fn rook_on_king_ring(&self) -> i32 {

        0
    }

    fn king_attackers_count(&self) -> i32 { //might be able to remove and replace

        0
    }

    fn king_ring(&self) -> i32 {

        0
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
            let num_of_pawns_masked = file_mask & rook_bitboard;
            sum += num_of_pawns_masked.count_ones() as i32 -2;
            rook_bitboard &= rook_bitboard -1;
        }
        sum
    }

    fn trapped_rook(&self) -> i32 {

        0
    }

    fn weak_queen(&self) -> i32 {

        0
    }

    fn queen_infiltration(&self) -> i32 {

        0
    }

    fn king_protector(&self) -> i32 {

        0
    }

    fn king_distance(&self) -> i32 {

        0
    }

    fn long_diagonal_bishop(&self) -> i32 {

        0
    }

    // MOBILITY MIDDLE GAME

    pub fn mobility_mg(&self, pins: &Vec<u8>) -> i32 {
        // sum function
        
        self.mobility_bonus(true, pins)
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
        let mobility_area = self.mobility_area(pins);
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
    
    pub fn mobility_area(&self, pins: &Vec<u8>) -> u64 {
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
        let mut pinned_bitboard = 0u64;
        for &square in pins {
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

    // SPACE FUNCTION

    // this function calculate how much space a side has
    fn space(&self) -> i32 {

        0
    }

    // KING MIDDLE GAME

    fn king_mg(&self) -> i32 {
        let mut v = 0;
        let mut kd = self.king_danger();
        
        v -= self.shelter_strength();
        v += self.shelter_storm();
        v += kd * kd / 4096;
        v += 8 * self.flank_attack();
        v += 17 * self.pawnless_flank();
        
        v
    }
    
    // check if the king is in danger, or can be in danger
    fn king_danger(&self) -> i32 {
        // this is a big function with a lot of branches 
        0
    }

    fn shelter_strength(&self) -> i32 {
        // calculate the pieces sheltring the king

        0
    }
    
    fn shelter_storm(&self) -> i32 {

        0
    }
    
    fn flank_attack(&self) -> i32 {

        0
    }
    
    fn pawnless_flank(&self) -> i32 {

        0
    }

    // WINNABLE MIDDLE GAME

    fn winnable_total_mg(&self, v: Option<i32>) -> i32 {
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
        fn flip_vertical(bb: u64) -> u64 {
            let mut flipped = 0;
            for rank in 0..8 {
                let rank_bits = (bb >> (rank * 8)) & 0xFF;
                flipped |= rank_bits << ((7 - rank) * 8);
            }
            flipped
        }

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
        clone_board.bitboards.white_pawns = flip_vertical(self.bitboards.black_pawns);
        clone_board.bitboards.white_bishops = flip_vertical(self.bitboards.black_bishops);
        clone_board.bitboards.white_knights = flip_vertical(self.bitboards.black_knights);
        clone_board.bitboards.white_rooks = flip_vertical(self.bitboards.black_rooks);
        clone_board.bitboards.white_queens = flip_vertical(self.bitboards.black_queens);
        clone_board.bitboards.white_king = flip_vertical(self.bitboards.black_king);

        clone_board.bitboards.black_pawns = flip_vertical(self.bitboards.white_pawns);
        clone_board.bitboards.black_bishops = flip_vertical(self.bitboards.white_bishops);
        clone_board.bitboards.black_knights = flip_vertical(self.bitboards.white_knights);
        clone_board.bitboards.black_rooks = flip_vertical(self.bitboards.white_rooks);
        clone_board.bitboards.black_queens = flip_vertical(self.bitboards.white_queens);
        clone_board.bitboards.black_king = flip_vertical(self.bitboards.white_king);

        // Return the modified cloned board
        clone_board
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