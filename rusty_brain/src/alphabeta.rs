use std::cmp::max;
use std::i32;
use crate::board::{Board, Turn};
use crate::movement::Move;
use crate::transposition::{TranspositionTable, Node};


impl Board {

    pub fn find_best_move(&mut self, transposition_table: &mut TranspositionTable, depth: i32) -> (Move, i32) {

        //using vanilla minimax
        /* 
        let eval = match self.turn {
            Turn::White => self.maxi(true, depth),
            Turn::Black => self.mini(true, depth)
        };
        */

        //using normal alphabeta
        /* 
        let eval = match self.turn {
            Turn::White => self.alpha_beta_max(true, i32::MIN, i32::MAX, depth),
            Turn::Black => self.alpha_beta_min(true, i32::MIN, i32::MAX, depth),
        };
        */
        
        //using alphabeta with transposition table
        /*
        let eval = match self.turn {
            Turn::White => self.alpha_beta_max_tt(transposition_table, true, i32::MIN, i32::MAX, depth),
            Turn::Black => self.alpha_beta_min_tt(transposition_table,true, i32::MIN, i32::MAX, depth),
        };
        (self.best_move.unwrap_or_else( || Move::encode(0, 0, 0)), eval)
        */
        
        //Iterative deepening, needs move ordering to show its strength
        let eval = match self.turn {
            Turn::White => self.iterative_deepening(transposition_table, true, depth),
            Turn::Black => self.iterative_deepening(transposition_table, false, depth),
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


    fn alpha_beta_max_tt(&mut self, transposition_table: &mut TranspositionTable, maximizing: bool, mut alpha: i32, mut beta: i32, depth_left: i32) -> i32 {

        if let Some(entry) = transposition_table.retrieve_from_table(self) {
            
            if entry.depth >= depth_left {
                match entry.node_type {
                    Node::Exact => {
                        if maximizing {
                            self.best_move = entry.best_move;
                        }
                        return entry.score;
                    },
                    Node::LowerBound => {
                        if entry.score > alpha {
                            alpha = entry.score;
                        }
                    }
                    Node::UpperBound => {
                        if entry.score < beta {
                            beta = entry.score;
                        }
                    }
                }
                if alpha >= beta {
                    if maximizing {
                        self.best_move = entry.best_move;
                    }                    
                    return entry.score;
                }

            }     

        }

        if depth_left == 0 {
            return self.evaluate();
        }

        let mut best_value = i32::MIN;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score: i32 = self.alpha_beta_min_tt(transposition_table, false, alpha, beta, depth_left - 1);
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
                    transposition_table.store_in_table(self, self.best_move, depth_left, best_value, alpha, beta);
                }
                return score;
            }
        }

        transposition_table.store_in_table(self, self.best_move, depth_left, best_value, alpha, beta);

        return best_value;

    }

    
    fn alpha_beta_min_tt(&mut self,  transposition_table: &mut TranspositionTable, minimizing: bool, mut alpha: i32, mut beta: i32, depth_left: i32) -> i32 {
        
        if let Some(entry) = transposition_table.retrieve_from_table(self) {
    
            if entry.depth >= depth_left {
                match entry.node_type {
                    Node::Exact => {
                        if minimizing {
                            self.best_move = entry.best_move;
                        }
                        return entry.score;
                    },
                    Node::LowerBound => {
                        if entry.score > alpha {
                            alpha = entry.score;
                        }
                    }
                    Node::UpperBound => {
                        if entry.score < beta {
                            beta = entry.score;
                        }
                    }
                }
                if alpha >= beta {
                    if minimizing {
                        self.best_move = entry.best_move;
                    }                    
                    return entry.score;
                }
            }         
        }
        
        if depth_left == 0 {
            return self.evaluate();
        }

        let mut best_value = i32::MAX;
        let moves: Vec<Move> = self.generate_legal_moves();

        for current_move in moves {

            self.make_move(current_move);
            let score = self.alpha_beta_max_tt(transposition_table, false, alpha, beta, depth_left-1);
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
                    transposition_table.store_in_table(self, self.best_move, depth_left, best_value, alpha, beta);
                }
                return score;
            }
        }

        transposition_table.store_in_table(self, self.best_move, depth_left, best_value, alpha, beta);
        
        return best_value;
    }

    fn iterative_deepening(&mut self, transposition_table: &mut TranspositionTable, maximizing: bool, max_depth: i32) -> i32 {
        
        let mut best_score = 0;
        let mut guess = 0;
        let mut delta = 100;
        
        for depth in 1..=max_depth{
            let mut alpha = guess - delta;
            let mut beta = guess + delta;

            loop{
                best_score = if maximizing{
                    self.alpha_beta_max_tt(transposition_table, true, alpha, beta, depth)
                }else{
                    self.alpha_beta_min_tt(transposition_table, true, alpha, beta, depth)
                };
                if best_score <= alpha{
                    alpha = alpha -delta; //fail low, widen window low side
                } else if best_score >= beta{
                    beta = beta + delta; // fail high, widen window high side
                }else{ // success!
                    guess = best_score; //want to figure out if it should be zeroed out every new depth or not
                    break;
                }
            delta = delta *2;}
        }
        
        best_score
    }

}