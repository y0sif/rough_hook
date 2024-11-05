use std::i32;
use crate::board::{Board, Turn};
use crate::movement::Move;

impl Board {

    pub fn find_best_move(&mut self, depth: i32) -> (Move, i32) {

        //using vanilla minimax
        let eval = match self.turn {
            Turn::White => self.maxi(true, depth),
            Turn::Black => self.mini(true, depth)
        };

        (self.best_move.unwrap(), eval)
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

}