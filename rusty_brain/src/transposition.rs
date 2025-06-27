use crate::zobrist::Zobrist;
use crate::movement::Move;
use crate::board::Board;
use std::collections::HashMap;

pub struct TranspositionTable {
    zobrist_key: Zobrist,
    table: HashMap<u64,TTEntry>,
}
pub struct TTEntry {
    pub best_move: Option<Move>, 
    pub depth: i32,          
    pub score: i32,          
    pub node_type: Node,   
}

pub enum Node {
    Exact,       // Exact (PV-Node)
    UpperBound,  // Upper bound (All-Node) , so it's <= alpha
    LowerBound,  // Lower bound (Cut-Node) , so it's >= beta
}

impl TranspositionTable {
    pub fn init() -> Self {
        //init the zobrist (fill random values)
        TranspositionTable {
            zobrist_key: Zobrist::init_zobrist(),
            table: HashMap::new(),
        }
    }

    pub fn retrieve_from_table(&self, board: &mut Board) -> Option<&TTEntry> {
        let position_hash = self.zobrist_key.zobrist_hash(&board);
        self.table.get(&position_hash)
    }

    pub fn store_in_table(&mut self, board: &mut Board, best_move: Option<Move>, depth_left: i32, best_value: i32, alpha: i32, beta: i32) {
        let position_hash = self.zobrist_key.zobrist_hash(&board);
        let node = if best_value <= alpha {
            Node::UpperBound
        } else if best_value >= beta {
            Node::LowerBound
        } else {
            Node::Exact
        };

        self.table.insert(
            position_hash,
            TTEntry {
                best_move: best_move,
                depth: depth_left,
                score: best_value,
                node_type: node,
            },
        );
    }
}