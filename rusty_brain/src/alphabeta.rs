use std::i32;
use crate::board::{Board, Turn};
use crate::movement::Move;

impl Board {

    pub fn find_best_move(&mut self, depth: i32) -> (Move, i32) {

        //using vanilla minimax
        /** 
        let eval = match self.turn {
            Turn::White => self.maxi(true, depth),
            Turn::Black => self.mini(true, depth)
        };
        **/

        //using alphabeta
        let eval = match self.turn {
            Turn::White => self.alpha_beta_max(true, i32::MIN, i32::MAX, depth),
            Turn::Black => self.alpha_beta_min(true, i32::MIN, i32::MAX, depth),
        };
        
        (self.best_move.unwrap_or_else( || Move::encode(0, 0, 0)), eval)
    }

    fn maxi(&mut self, maximazing: bool, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }
        let mut max = i32::MIN;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score = self.mini(false, depth - 1);
            self.undo_move();

            if score > max {
                max = score;
                if maximazing {
                    self.best_move = Some(current_move);
                }
            }
        }
        return max;
    }

    fn mini(&mut self, minimizing: bool, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }
        let mut min = i32::MAX;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score = self.maxi(false, depth - 1);
            self.undo_move();

            if score < min {
                min = score;
                if minimizing {
                    self.best_move = Some(current_move);
                }
            }
        }
        return min;
    }

    fn alpha_beta_max(&mut self, maximizing: bool, mut alpha: i32, beta: i32, depth_left: i32) -> i32 {

        if depth_left == 0 {
            return self.evaluate();
        }

        let mut best_value = i32::MIN;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score: i32 = self.alpha_beta_min(false, alpha, beta, depth_left - 1);
            self.undo_move();
            
            if score > best_value {
                best_value = score;
                if maximizing {
                    self.best_move = Some(current_move);
                }
                if score > alpha {
                    alpha = score;
                }
            }
            if score >= beta {
                if maximizing {
                    self.best_move = Some(current_move);
                }
                return score;
            }
        }

        return best_value;

    }
    
    fn alpha_beta_min(&mut self, minimizing: bool, alpha: i32, mut beta: i32, depth_left: i32) -> i32 {
        
        if depth_left == 0 {
            return self.evaluate();
        }

        let mut best_value = i32::MAX;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score = self.alpha_beta_max(false, alpha, beta, depth_left-1);
            self.undo_move();

            if score < best_value {
                best_value = score;
                if minimizing {
                    self.best_move = Some(current_move);
                }
                if score < beta {
                    beta = score;
                }
            }
            if score <= alpha {
                if minimizing {
                    self.best_move = Some(current_move);
                }
                return score;
            }
        }
        return best_value;
    }
}