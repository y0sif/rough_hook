use std::process::id;

use crate::{bitboards::{self, Bitboards}, board::{Board, Turn}, square::{self, Rank, Square}};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mg = self.middle_game_evaluation(true);
        mg
    }
    
    fn middle_game_evaluation(&self, nowinnable: bool) -> i32 {
        let mut v = 0;
        let color_flip_board = self.color_flip();
        v += self.piece_value_mg() - color_flip_board.piece_value_mg();
        v += self.psqt_mg() - color_flip_board.psqt_mg();
        v += self.imbalance_total(&color_flip_board);
        v += self.pawns_mg() - color_flip_board.pawns_mg(); 
        v += self.mobility_mg() - color_flip_board.mobility_mg();
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
        self.piece_value_bonus(true)
    }

    fn piece_value_bonus(&self, is_middle_game: bool) -> i32 {
        // pawn, knight, bishop, rook, queen
        let a = if is_middle_game {
            [124, 781, 825, 1276, 2538]
        } else {
            [206, 854, 915, 1380, 2682]
        };

        let mut sum = 0;

        match self.turn {
            Turn::White => {
                sum += self.bitboards.white_pawns.count_ones() as i32 * a[0];
                sum += self.bitboards.white_knights.count_ones() as i32 * a[1];
                sum += self.bitboards.white_bishops.count_ones() as i32 * a[2];
                sum += self.bitboards.white_rooks.count_ones() as i32 * a[3];
                sum += self.bitboards.white_queens.count_ones() as i32 * a[4];
            }
            Turn::Black => {
                sum += self.bitboards.black_pawns.count_ones() as i32 * a[0];
                sum += self.bitboards.black_knights.count_ones() as i32 * a[1];
                sum += self.bitboards.black_bishops.count_ones() as i32 * a[2];
                sum += self.bitboards.black_rooks.count_ones() as i32 * a[3];
                sum += self.bitboards.black_queens.count_ones() as i32 * a[4];
            }
        }

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
        
        match self.turn {
            Turn::White => {
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
            }
            Turn::Black => {
                let mut pawn_bitboard = self.bitboards.black_pawns;
                 
                while pawn_bitboard != 0 {
                    let square = pawn_bitboard.trailing_zeros() as u8;
                    let rank = Square::from(square).rank() as usize;
                    let file = Square::from(square).file() as usize;
                    
                    sum += p_bonus[7-rank][file];
                    
                    pawn_bitboard &= pawn_bitboard - 1;
                }

                sum += calculate_bonus(self.bitboards.black_knights, &bonus[0], -1);
                sum += calculate_bonus(self.bitboards.black_bishops, &bonus[1], -1);
                sum += calculate_bonus(self.bitboards.black_rooks, &bonus[2], -1);
                sum += calculate_bonus(self.bitboards.black_queens, &bonus[3], -1);
                sum += calculate_bonus(self.bitboards.black_king, &bonus[4], -1);
            }
        }

        sum
    }

    // IMBALANCE TOTAL
    
    fn imbalance_total(&self, flip: &Board) -> i32 {
        let mut v = 0;
        v += self.imbalance() - flip.imbalance();
        v += self.bishop_pair() - flip.bishop_pair();

        v / 16
    }

    pub fn imbalance(&self) -> i32 {
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
            let mut sum = 0;
            let mut bb = bitboard;

            while bb != 0 {
                if i % 6 > j {
                    bb &= bb - 1;
                    continue;
                }
                sum += table[i];

                bb &= bb - 1; 
            }
            sum
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

                bb &= bb - 1;
            }
            
            match self.turn {
                Turn::White => {
                    if bishop[0] > 1 {
                        sum += enemy_table[idx][0]
                    }
                    if bishop[1] > 1 {
                        sum += ally_table[idx][0]
                    }
                },
                Turn::Black => {
                    if bishop[0] > 1 {
                        sum += ally_table[idx][0]
                    }
                    if bishop[1] > 1 {
                        sum += enemy_table[idx][0]
                    }
                }
            }
            sum
        };

        match self.turn {
            Turn::White => {
                bishop[0] = self.bitboards.white_bishops.count_ones();
                bishop[1] = self.bitboards.black_bishops.count_ones();
                
                v += calculate_bonus(&bishop, self.bitboards.white_pawns, &qo, &qt, 1);
                v += calculate_bonus(&bishop, self.bitboards.white_knights, &qo, &qt, 2);
                v += calculate_bonus(&bishop, self.bitboards.white_bishops, &qo, &qt, 3);
                v += calculate_bonus(&bishop, self.bitboards.white_rooks, &qo, &qt,4);
                v += calculate_bonus(&bishop, self.bitboards.white_queens, &qo, &qt,5);

            },
            Turn::Black => {
                bishop[0] = self.bitboards.black_bishops.count_ones();
                bishop[1] = self.bitboards.white_bishops.count_ones();

                v += calculate_bonus(&bishop, self.bitboards.black_pawns, &qt, &qo, 1);
                v += calculate_bonus(&bishop, self.bitboards.black_knights, &qt, &qo, 2);
                v += calculate_bonus(&bishop, self.bitboards.black_bishops, &qt, &qo, 3);
                v += calculate_bonus(&bishop, self.bitboards.black_rooks, &qt, &qo,4);
                v += calculate_bonus(&bishop, self.bitboards.black_queens, &qt, &qo,5);
            }
        }

        v
    }
    
    pub fn bishop_pair(&self) -> i32 {
        match self.turn {
            Turn::White => {
                if self.bitboards.white_bishops.count_ones() < 2 {
                    return 0;
                }else{
                    return 1438;
                }
            },
            Turn::Black => {
                if self.bitboards.black_bishops.count_ones() < 2 {
                    return 0;
                }else{
                    return 1438;
                }
            }
        }     
    }
    
    // PAWNS MIDDLE GAME
    
    fn pawns_mg(&self) -> i32 {
        // sum function
        
        let mut v = 0;
        
        let mut pawn_bitboard = match self.turn {
            Turn::White => self.bitboards.white_pawns,
            Turn::Black => self.bitboards.black_pawns,
        };
        
        while pawn_bitboard != 0 {
            let square = pawn_bitboard.trailing_zeros() as u8;
            let square_position = 1 << square;

            if self.doubled_isolated(square, square_position) == 1{
                v -= 11;
            }else if self.isolated(square_position) == 1{
                v -= 5;
            }else if self.backward(square_position) == 1{
                v -= 9;
            }

            v -= self.doubled(square_position) * 11;
        
            if self.connected_bonus(square_position, square) == 1{
                v += self.connected(square_position);
            }
            
            v -= 13 * self.weak_unopposeed_pawn(square_position, square);
            
            let arr = [0, -11, -3];
            
            v += arr[self.blocked(square_position, square) as usize]; 

            pawn_bitboard &= pawn_bitboard - 1;
        }
    
        v
    }
    
    fn doubled_isolated(&self, square: u8, square_position: u64) -> i32 {
        match self.turn {
            Turn::White => {
                if self.isolated(square_position) == 1 {
                    let mut obe = 0;
                    let mut eop = 0;
                    let mut ene = 0;

                    let above_pawns = Bitboards::north_mask_ex(square) & self.bitboards.white_pawns; 
                    let under_pawns = Bitboards::south_mask_ex(square) & self.bitboards.white_pawns; 
                    
                    obe += above_pawns.count_ones();
                    eop += under_pawns.count_ones();

                    let neighbor_enemy_pawns = Bitboards::file_mask_to_end(Bitboards::move_east(square_position).trailing_zeros() as u8) |
                                                    Bitboards::file_mask_to_end(Bitboards::move_west(square_position).trailing_zeros() as u8);
                    
                    let neighbor_enemy_pawns = neighbor_enemy_pawns & self.bitboards.black_pawns;
                    
                    ene += neighbor_enemy_pawns.count_ones();

                    if obe > 0 && ene == 0 && eop > 0 {
                        return 1;
                    }
                }    
            },
            Turn::Black => {
                if self.isolated(square_position) == 1 {
                    let mut obe = 0;
                    let mut eop = 0;
                    let mut ene = 0;

                    let under_pawns = Bitboards::north_mask_ex(square) & self.bitboards.black_pawns; 
                    let above_pawns = Bitboards::south_mask_ex(square) & self.bitboards.black_pawns; 
                    
                    obe += above_pawns.count_ones();
                    eop += under_pawns.count_ones();

                    let neighbor_enemy_pawns = Bitboards::file_mask_to_end(Bitboards::move_east(square_position).trailing_zeros() as u8) |
                                                    Bitboards::file_mask_to_end(Bitboards::move_west(square_position).trailing_zeros() as u8);
                    
                    let neighbor_enemy_pawns = neighbor_enemy_pawns & self.bitboards.white_pawns;
                    
                    ene += neighbor_enemy_pawns.count_ones();

                    if obe > 0 && ene == 0 && eop > 0 {
                        return 1;
                    }
                }    
                
            },
        }
        
        0
    }

    fn isolated(&self, square_position: u64) -> i32 {
        let neighbor_pawns = Bitboards::file_mask_to_end(Bitboards::move_east(square_position).trailing_zeros() as u8)
                                  | Bitboards::file_mask_to_end(Bitboards::move_west(square_position).trailing_zeros() as u8);

        match self.turn {
            Turn::White => {
                if neighbor_pawns & self.bitboards.white_pawns != 0 {
                    return 0;
                }

            },
            Turn::Black => {
                if neighbor_pawns & self.bitboards.black_pawns != 0 {
                    return 0;
                }

            },
        }

        1
    }

    fn backward(&self, square_position: u64) -> i32 {
        match self.turn {
            Turn::White => {
                let neighbor_pawns = Bitboards::south_mask_ex(Bitboards::move_east(square_position).trailing_zeros() as u8) | 
                                          Bitboards::south_mask_ex(Bitboards::move_west(square_position).trailing_zeros() as u8) |
                                          Bitboards::move_east(square_position) | 
                                          Bitboards::move_west(square_position);
                if neighbor_pawns & self.bitboards.white_pawns != 0 {
                    return 0;
                }
                
                let enemy_pawns = Bitboards::move_north(Bitboards::move_north(square_position));
                let enemy_pawns = Bitboards::move_east(enemy_pawns) | Bitboards::move_west(enemy_pawns);
                
                if enemy_pawns & self.bitboards.black_pawns != 0 {
                    return 1;
                }
                
            },
            Turn::Black => {
                let neighbor_pawns = Bitboards::north_mask_ex(Bitboards::move_east(square_position).trailing_zeros() as u8) | 
                                          Bitboards::north_mask_ex(Bitboards::move_west(square_position).trailing_zeros() as u8) |
                                          Bitboards::move_east(square_position) | 
                                          Bitboards::move_west(square_position);

                if neighbor_pawns & self.bitboards.black_pawns != 0 {
                    return 0;
                }
                
                let enemy_pawns = Bitboards::move_north(Bitboards::move_north(square_position));
                let enemy_pawns = Bitboards::move_east(enemy_pawns) | Bitboards::move_west(enemy_pawns);
                
                if enemy_pawns & self.bitboards.white_pawns != 0 {
                    return 1;
                }
                
            },
        }

        0
    }

    fn doubled(&self, square_position: u64) -> i32 {
        match self.turn {
            Turn::White => {
                let above_pawn = Bitboards::move_north(square_position);
                if above_pawn & self.bitboards.white_pawns == 0 {
                    return 0;
                }
                
                let support_pawns = Bitboards::move_south(square_position);
                let support_pawns = Bitboards::move_east(support_pawns) | Bitboards::move_west(support_pawns);
                if support_pawns != 0 {
                    return 0;
                }
                
                return 1;
            },
            Turn::Black => {
                let under_pawns = Bitboards::move_south(square_position);
                if under_pawns & self.bitboards.white_pawns == 0 {
                    return 0;
                }
                
                let support_pawns = Bitboards::move_north(square_position);
                let support_pawns = Bitboards::move_east(support_pawns) | Bitboards::move_west(support_pawns);
                if support_pawns != 0 {
                    return 0;
                }
                
                return 1;
            },
        }
        // if square not pawn return 0

        // if square above pawn is not pawn return 0

        // if pawn have neighbors return 0

        1 
    }

    fn connected_bonus(&self, square_position: u64, square: u8) -> i32 {

        if self.connected(square_position) == 0{
            return 0;
        }

        let seed = [0, 7, 8, 12, 29, 48, 86];
        
        let op = self.opposed(square);
        let ph = self.phalanx(square_position);
        let su = self.supported(square_position);
        let bl = match self.turn {
            Turn::White => {
                if Bitboards::move_north(square_position) & self.bitboards.black_pawns != 0 {
                    return 1;
                }
                0
            },
            Turn::Black => {
                if Bitboards::move_south(square_position) & self.bitboards.white_pawns != 0 {
                    return 1;
                }
                0
            },
        };
        
        let r = Square::from(square).rank() as usize;
        
        if r < 2 || r > 7 {
            return 0;
        }

        match self.turn {
            Turn::White => {
                seed[r - 1] * (2 + ph - op) + 21 * su
            },
            Turn::Black => {
                seed[r + 1] * (2 + ph - op) + 21 * su
            },
        }
    }

    fn connected(&self, square_position: u64) -> i32 {
        if self.supported(square_position) != 0 || self.phalanx(square_position) == 1{
            return 1;
        }   

        0
    }
    
    fn supported(&self, square_position: u64) -> i32 {
        match self.turn {
            Turn::White => {
                let support_pawns = Bitboards::move_south(square_position);
                let support_pawns = Bitboards::move_east(support_pawns) | Bitboards::move_west(support_pawns);
                
                return support_pawns.count_ones() as i32;

            },
            Turn::Black => {
                let support_pawns = Bitboards::move_north(square_position);
                let support_pawns = Bitboards::move_east(support_pawns) | Bitboards::move_west(support_pawns);
                
                return support_pawns.count_ones() as i32;
            },
        }
        
    }
    
    fn phalanx(&self, square_position: u64) -> i32 {
        let phalan = Bitboards::move_east(square_position) | Bitboards::move_west(square_position);
        
        if phalan != 0 {
            return 1;
        }

        0
    }

    fn weak_unopposeed_pawn(&self, square_position: u64, square: u8) -> i32 {
        if self.opposed(square) == 1{
            return 0;
        }

        let mut v = 0;

        if self.isolated(square_position) == 1{
            v += 1;
        }else if self.backward(square_position) == 1{
            v += 1;
        }

        v
    }

    fn opposed(&self, square: u8) -> i32 {

        match self.turn {
            Turn::White => {
                let op = Bitboards::north_mask_ex(square) & self.bitboards.black_pawns;
                
                if op != 0 {
                    return 1;
                }
            },
            Turn::Black => {
                let op = Bitboards::south_mask_ex(square) & self.bitboards.white_pawns;
                
                if op != 0 {
                    return 1;
                }
                
            },
        }

        0
    }
    
    fn blocked(&self, square_position: u64, square: u8) -> i32 {
        match self.turn {
            Turn::White => {
                let rank = Square::from(square).rank();
                
                if rank != Rank::Second && rank != Rank::Third {
                    return 0;
                }
                
                if Bitboards::move_north(square_position) & self.bitboards.black_pawns == 0 {
                    return 0;
                }
                
                return 4 - rank as i32;
            },
            Turn::Black => {
                let rank = Square::from(square).rank();
                
                if rank != Rank::Seventh && rank != Rank::Sixth {
                    return 0;
                }
                
                if Bitboards::move_south(square_position) & self.bitboards.white_pawns == 0 {
                    return 0;
                }
                
                return rank as i32;
            },
        }
    }
    
    // MOBILITY MIDDLE GAME

    fn mobility_mg(&self) -> i32 {
        // sum function
        
        self.mobility_bonus(true)
    }
    
    fn mobility_bonus(&self, is_middle_game: bool) -> i32 {
        // sum function

        // bonus depending on the middle game flag

        // check for square to see which piece is it and apply the bonus accordingly

        // else

        return 0
    }
    
    fn mobility(&self) -> i32 {
        // sum function

        let mut v = 0;

        // apply mobility bonues depending on the piece on given square
        // 
        // there is a lot of function calls here, remember that.

        v
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

    // COLOR FLIP FOR BOARD

    pub fn color_flip(&self) -> Self {
        let mut clone_board = self.clone();

        // Swap pawns
        let temp = clone_board.bitboards.white_pawns;
        clone_board.bitboards.white_pawns = clone_board.bitboards.black_pawns;
        clone_board.bitboards.black_pawns = temp;

        // Swap knights
        let temp = clone_board.bitboards.white_knights;
        clone_board.bitboards.white_knights = clone_board.bitboards.black_knights;
        clone_board.bitboards.black_knights = temp;

        // Swap bishops
        let temp = clone_board.bitboards.white_bishops;
        clone_board.bitboards.white_bishops = clone_board.bitboards.black_bishops;
        clone_board.bitboards.black_bishops = temp;

        // Swap rooks
        let temp = clone_board.bitboards.white_rooks;
        clone_board.bitboards.white_rooks = clone_board.bitboards.black_rooks;
        clone_board.bitboards.black_rooks = temp;

        // Swap queens
        let temp = clone_board.bitboards.white_queens;
        clone_board.bitboards.white_queens = clone_board.bitboards.black_queens;
        clone_board.bitboards.black_queens = temp;

        // Swap kings
        let temp = clone_board.bitboards.white_king;
        clone_board.bitboards.white_king = clone_board.bitboards.black_king;
        clone_board.bitboards.black_king = temp;

        clone_board
    }
}