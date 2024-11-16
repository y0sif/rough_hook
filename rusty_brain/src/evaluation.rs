use crate::{board::{Board, Turn}, square};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mg = 
        middle_game_evaluation(self);
        0
    }
    
    fn middle_game_evaluation(&self, nowinnable: bool) -> i32 {
        let mut v = 0;
        let color_flip_board = self.color_flip();
        v += piece_value_mg(self, square) - piece_value_mg(color_flip_board, square);
        v += psqt_mg(self) - psqt_mg(color_flip_board);
        v += imbalance_total(self, color_flip_board);
        v += pawns_mg(self) - pawns_mg(color_flip_board); 
        v += mobility_mg(self) - mobility_mg(color_flip_board);
        v += threats_mg(self) - threats_mg(color_flip_board);
        v += passed_mg(self) - passed_mg(color_flip_board);
        v += space(self) - space(color_flip_board);
        v += king_mg(self) - king_mg(color_flip_board);
        
        if !nowinnable {
            v += winnable_total_mg(self, v);
        }

        v
    }
    
    // PIECE VALUE MIDDLE GAME
    
    fn piece_value_mg(&self, square: todo!()) -> i32 {
        // sum function

        piece_value_bonus(self, square, true)
        
    }

    fn piece_value_bonus(self, square: todo!(), is_middle_game: bool) -> i32 {
        // sum function

        let a = if is_middle_game {
            [124, 781, 825, 1276, 2538]
        }else {
            [206, 854, 915, 1380, 2682]
        };
        // should use square to add the bonus

        0
    }
    
    // PSQT MIDDLE GAME

    fn psqt_mg(&self, square: todo!()) -> i32 {
        // sum functions

        psqt_bonus(self, square, true)
    }
    
    fn psqt_bonus(&self, square: todo!(), is_middle_game: bool) -> i32 {
        // apply bonus somehow

        0
    }

    // IMBALANCE TOTAL
    
    fn imbalance_total(&self, flip: &Board) -> i32 {
        let mut v = 0;
        v += imbalance(self) - imbalance(flip);
        v += bishop_pair(self) - bishop_pair(flip);

        v / 16
    }

    fn imbalance(&self) -> i32 {
        // sum function

        // calculate niggerlicious imbalance

        0
    }
    
    fn bishop_pair(&self, square: todo!()) -> i32 {
        if bishop_count(self, todo!()) < 2 {
            return 0;
        }
        
        // if no square return 1438

        // if square is bishop return 1 else 0
    }
    
    fn bishop_count(&self, square: todo!()) -> i32 {
        // sum function
        
        // if square is bishop return 1

        0
    }
    
    // PAWNS MIDDLE GAME
    
    fn pawns_mg(&self, square: todo!()) -> i32 {
        // sum function
        
        let mut v = 0;
        
        if self.doubled_isolated(square) {
            v -= 11;
        }else if self.isolated(square) {
            v -= 5;
        }else if self.backward(square) {
            v -= 9;
        }

        v -= doubled(self, square) * 11;
    
        if connected_bonus(self, square) {
            v += connected(self, square);
        }
        
        v -= 13 * weak_unopposeed_pawn(self, square);
        
        // add something niggerlicious to v in case of blocked function

        v
    }
    
    fn doubled_isolated(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        if self.isolated(square) {
            // do niggerlicious stuff here
        }

        0
    }

    fn isolated(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        // iterate over the same rank, if there is a pawn -> not isolated so return 0

        // else
        1
    }

    fn backward(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        // iterate to check if it is backwards or not

        //else 
        0
    }

    fn doubled(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        // if square above pawn is not pawn return 0

        // if pawn have neighbors return 0

        1 
    }

    fn connected_bonus(&self, square: todo!()) -> i32 {
        // sum function

        if !connected(self, square) {
            return 0;
        }

        // calculate connected bonus from the given function in the wiki

        0
    }

    fn connected(&self, square: todo!()) -> i32 {
        // sum function

        // check for phalanx and supported

        0
    }

    fn weak_unopposeed_pawn(&self, square: todo!()) -> i32 {
        // sum function

        if self.opposed(square) {
            return 0;
        }

        let mut v = 0;

        if self.isolated(square) {
            v += 1;
        }else if self.backward(square) {
            v += 1;
        }

        v
    }

    fn opposed(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        // check if the pawn is opposed by enemy pawn -> return 1

        0
    }
    
    fn blocked(&self, square: todo!()) -> i32 {
        // sum function

        // if square not pawn return 0

        // check for a condition regarding pawn placement

        // if there is a pawn behind me -> return 0

        // else return 4 - pawn rank
        0
    }
    
    // MOBILITY MIDDLE GAME

    fn mobility_mg(&self, square: todo!()) -> i32 {
        // sum function
        
        mobility_bonus(self, square, true)
    }
    
    fn mobility_bonus(&self, square: todo!(), is_middle_game: bool) -> i32 {
        // sum function

        // bonus depending on the middle game flag

        // check for square to see which piece is it and apply the bonus accordingly

        // else

        return 0
    }
    
    fn mobility(&self, square: todo!()) -> i32 {
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

        v += 69 * hanging(self);
        v += king_threat(self);
        v += 48 * pawn_push_threat(self);
        v += 173 * threat_safe_pawn(self);
        v += 60 * slider_on_queen(self);
        v += 16 * knight_on_queen(self);
        v += 7 * restricted(self);
        v += 14 * weak_queen_protection(self);
        
        // iterate over board to check for minor threat function and rook threat function

        v
    }
    
    fn hanging(&self, square: todo!()) -> i32 {
        // sum function

        if !weak_enemies(self, square) {
            return 0;
        }
        
        // check for attacks 

        0
    }
    
    fn weak_enemies(&self, square: todo!()) -> i32 {
        // sum function

        // check if the square is protected or not, if protected return 0, if not return 1
        // 
        
        // check for attacks, attacks functions are a lot, remember that.

        0
    }
    
    fn king_threat(&self, square: todo!()) -> i32 {
        // sum function

        // if square is not enemy piece return 0

        if !weak_enemies(self, square) {
            return 0;
        }
        
        // check for king attack 

        0
    }
    
    fn pawn_push_threat(&self, square: todo!()) -> i32 {
        // sum function

        // is square is not enemy return 0

        // iterate to check if there is a pawn push threat
        // 

        // else
        
        0
    }
    
    fn threat_safe_pawn(&self, square: todo!()) -> i32 {
        // sum function

        // if square not enemy pawn return 0

        // check pawn attack function

        // check safe pawn function

        0
    }

    fn safe_pawn(&self, square: todo!()) -> i32 {
        // sum function

        // if square is not enemy pawn return 0

        // check attacks funciton

        0
    }

    // this function sees if i can gain tempo on a queen using a slider piece
    fn slider_on_queen(&self, square: todo!()) -> i32 {
        // sum function

        // check for enemy pawn

        // check for attack function
        
        // check for mobility area funciton

        //check for diagonal attacks and xray attacks

        0
    }
    
    // this function sees if i can gain tempo on a queen using a knight
    fn knight_on_queen(self, square: todo!()) -> i32 {
        // sum function

        0
    }

    fn restricted(&self, square: todo!()) -> i32 {
        // sum function

        // check attack function

        // check attack function for opposite color

        0
    }

    fn weak_queen_protection(&self, square: todo!()) -> i32 {
        // sum function

        if !weak_enemies(self, square) {
            return 0;
        }
        
        // check queen attacks function

        0
    }

    // PASSED MIDDLE GAME

    fn passed_mg(&self, square: todo!()) -> i32 {
        // sum function

        // check for passed leverable function

        let mut v = 0;

        // add bonus depending on the passed pawn rank

        // sub bonus depending on the passed pawn file

        v
    }

    // SPACE FUNCTION

    // this function calculate how much space a side has
    fn space(&self, square: todo!()) -> i32 {

        0
    }

    // KING MIDDLE GAME

    fn king_mg(&self) -> i32 {
        let mut v = 0;
        let mut kd = king_danger(self);
        
        v -= shelter_strength(self);
        v += shelter_storm(self);
        v += kd * kd / 4096;
        v += 8 * flank_attack(self);
        v += 17 * pawnless_flank(self);
        
        v
    }
    
    // check if the king is in danger, or can be in danger
    fn king_danger(&self) -> i32 {
        // this is a big function with a lot of branches 
        0
    }

    fn shelter_strength(&self, square: todo!()) -> i32 {
        // calculate the pieces sheltring the king

        0
    }
    
    fn shelter_storm(&self, square: todo!()) -> i32 {

        0
    }
    
    fn flank_attack(&self, square: todo!()) -> i32 {

        0
    }
    
    fn pawnless_flank(&self, square: todo!()) -> i32 {

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


        return v * i32::max(i32::min(winnable(self) + 50, 0), -i32::abs(v));
    }
    
    fn winnable(&self) -> i32 {

        0
    }

    // COLOR FLIP FOR BOARD

    fn color_flip(&self) -> Self {
        let mut clone_board = self.clone();

        macro_rules! swap_pieces {
            ($piece:ident) => {
                let temp = clone_board.bitboards.white_$piece;
                clone_board.bitboards.white_$piece = clone_board.bitboards.black_$piece;
                clone_board.bitboards.black_$piece = temp;
            };
        }

        swap_pieces!(pawns);
        swap_pieces!(knights);
        swap_pieces!(bishops);
        swap_pieces!(rooks);
        swap_pieces!(queens);
        swap_pieces!(king);

        clone_board
    }
}