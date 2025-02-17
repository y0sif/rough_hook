use crate::board::Board;
use std::rc::Rc;
use core::cell::RefCell;
use rand::Rng;
use crate::movement::Move;


pub struct Hyperparameters {
    pub iterations: u32,
    pub c: f32,
    pub depth: u32, //limit the simulation phase
}

struct Node {
    board: Board,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    visits: u32,
    value: f32,
}

impl Node {
    fn new(board: Board, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            board,
            parent,
            children: vec![],
            visits: 0,
            value: 0.0,
        }
    }

    fn best_child(&self, c: f32) -> Option<Rc<RefCell<Node>>> {
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
        (self.children.len() == self.board.generate_legal_moves().len()) && self.children.len() != 0
    }

    fn terminal(&self) -> bool {
        self.board.checkmate || self.board.stalemate || self.board.draw
    }
}

impl Board {

    pub fn find_best_move_mcts(&mut self, hyperparameters: &Hyperparameters) -> Move {
        let best_move = self.mcts(&hyperparameters);
        best_move
    }

    pub fn mcts(&mut self, hyperparameters: &Hyperparameters) -> Move {
        
        let root_node = Rc::new(RefCell::new(Node::new(self.clone(), None)));

        for _ in 0..hyperparameters.iterations {
            // <<Phase 1: Selection>>
            let mut current_node = Rc::clone(&root_node);
            //To select a child, the node must have been fully expanded & node must not be terminal (game hasn't ended)
            //Otherwise, the root itself is selected and is hence expanded
            //Same goes for child nodes
            while {
                let mut current_node_mut = current_node.borrow_mut();
                current_node_mut.fully_expanded() && !current_node_mut.terminal()
            } {
                let best_child = current_node
                    .borrow()
                    .best_child(hyperparameters.c)
                    .unwrap();
                current_node = best_child;
            }
            

            // <<Phase 2: Expansion>>
            {
                let mut current_node_mut = current_node.borrow_mut();
                if !current_node_mut.terminal() {
                    //We need to generate the next move for the new expanded state
                    /*
                    To avoid storing a list of used moves and unused moves, we can
                    make use of the length of the children count to get the move
                    that follows, this exploits the nature of the static order in move generation
                    */
                    let legal_moves = current_node_mut.board.generate_legal_moves();
                    let children_count = current_node_mut.children.len();
                    let chosen_move = legal_moves[children_count];
                    //Copy current board to apply the chosen move to it then assign this board to the new child
                    let mut new_board = current_node_mut.board.clone();
                    new_board.make_move(chosen_move);
                    let new_child = Rc::new(RefCell::new(Node::new(new_board, Some(Rc::clone(&current_node)))));
                    current_node_mut.children.push(new_child); //Parent has expanded
                    
                }
            }

            // <<Phase 3: Simulation>>
            let sim_evaluation = {
                let node_to_simulate_from = current_node.borrow();
                let mut temp_board = node_to_simulate_from.board.clone();
                let mut current_depth = 0;
                while !temp_board.checkmate 
                    && !temp_board.stalemate 
                    && !temp_board.draw
                    && current_depth < hyperparameters.depth {
                    let legal_moves = temp_board.generate_legal_moves();
                    if legal_moves.is_empty() {
                        break;
                    }
                    let mut rng = rand::thread_rng();
                    let random_index = rng.gen_range(0..legal_moves.len());
                    let random_move = legal_moves[random_index];
                    temp_board.make_move(random_move);
                    current_depth += 1;
                }
                temp_board.evaluate()
            };


            // <<Phase 4: Backpropagation>>
            let mut current = Some(Rc::clone(&current_node));
            while let Some(node) = current {
                let mut node_mut = node.borrow_mut();
                node_mut.visits += 1;
                node_mut.value += sim_evaluation as f32;
                current = node_mut.parent.clone();
            }
        }

        let chosen_child_rc = {
        let root_node_ref = root_node.borrow();
        root_node_ref
            .children
            .iter()
            .max_by_key(|child| child.borrow().visits)
            .unwrap()
            .clone()
        };

        let chosen_move = chosen_child_rc
            .borrow()
            .board
            .move_log
            .last()
            .cloned()
            .unwrap();

    chosen_move

    }
}