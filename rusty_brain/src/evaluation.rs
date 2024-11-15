use crate::{board::{Board, Turn}, square};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mg = 
        middle_game_evaluation(self);
        0
    }
    
    fn middle_game_evaluation(&self) -> i32 {
        let mut v = 0;
        let color_flip_board = self.color_flip();
        v += piece_value_mg(self, square) - piece_value_mg(color_flip_board, square);
        v += psqt_mg(self) - psqt_mg(color_flip_board);
        v += imbalance_total(self, color_flip_board);
        v += pawns_mg(self) - pawns_mg(color_flip_board); 
        // continue from here
        v += mobility_mg(self) - mobility_mg(color_flip_board);

        0
    }
    
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

    fn psqt_mg(&self, square: todo!()) -> i32 {
        // sum functions

        psqt_bonus(self, square, true)
    }
    
    fn psqt_bonus(&self, square: todo!(), is_middle_game: bool) -> i32 {
        // apply bonus somehow

        0
    }
    
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

    fn mobility_mg(&self) -> i32 {
        0
    }

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