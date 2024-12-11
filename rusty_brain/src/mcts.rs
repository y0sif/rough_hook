use crate::board::Board;
use std::rc::Rc;
use core::cell::RefCell;

pub struct Hyperparameters {
    iterations: u32,
    c: f32,
    depth: u32, //limit the simulation phase
}

struct Node<'a> {
    board: &'a mut Board,
    parent: Option<Rc<RefCell<Node<'a>>>>,
    children: Vec<Rc<RefCell<Node<'a>>>>,
    visits: u32,
    value: f32,
}

impl<'a> Node<'a> {
    fn new(board: &'a mut Board, parent: Option<Rc<RefCell<Node<'a>>>>) -> Self {
        Node {
            board,
            parent,
            children: vec![],
            visits: 0,
            value: 0.0,
        }
    }

    fn best_child(&self, c: f32) -> Option<Rc<RefCell<Node<'a>>>> {
        //UCB formula: Exploitation + Exploration
        //wi/ni + C*sqrt(len(N)/ni)
        //where w is the value, n is the visit, i is the current child node, and N is the value of root

        let mut selected_child = None;
        let mut max_ucb = f32::MIN;

        for child in &self.children {
            let child_ref = child.borrow();
            let child_visits = child_ref.visits as f32;
            let child_value = child_ref.value;

            //Calculate the UCB value
            //only exploit if it was visited before
            let exploitation = if child_visits != 0.0 { child_value / child_visits } else { 0.0 };
            let exploration = c * ((self.visits as f32).ln() / child_visits).sqrt();
            let ucb = exploitation + exploration;

            if ucb > max_ucb {
                max_ucb = ucb;
                selected_child = Some(child.clone());
            }
        }
        selected_child
    }

    fn fully_expanded(&mut self) -> bool {
        self.children.len() == self.board.generate_legal_moves().len()
    }

    fn terminal(&self) -> bool {
        self.board.checkmate || self.board.stalemate || self.board.draw
    }
}

impl Board {
    pub fn mcts(&mut self, hyperparameters: &Hyperparameters) {
        
        let root_node = Rc::new(RefCell::new(Node::new(self, None)));

        for _ in 0..hyperparameters.iterations {
            //Phase 1: Selection
            let mut current_node = Rc::clone(&root_node);

            //To select a child, the node must have been fully expanded & node must not be terminal (game hasn't ended)
            while {
                let mut current_node_mut = current_node.borrow_mut();
                let fully_expanded = current_node_mut.fully_expanded();
                let terminal = current_node_mut.terminal();
                fully_expanded && !terminal
            } {
                let best_child = current_node
                    .borrow_mut()
                    .best_child(hyperparameters.c)
                    .unwrap();
                current_node = best_child;
            }

            //Phase 2: Expansion

        }
    }
}
