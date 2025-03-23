use std::io::{self, Write};
use burn::prelude::Backend;
use nnue::model::Model;

use crate::board::Board;
use crate::movement::Move;
use crate::square::Square;
use crate::transposition::TranspositionTable;
pub struct Uci<B: Backend> {
    current_board: Board<B>,
    // default_depth: u8,
    depth: i32,
    transposition_table: TranspositionTable,
    model: Model<B>,
    device: B::Device
}

struct ucioption {

}

impl<B: Backend> Uci<B> {
    pub fn new(model: Model<B>, device: B::Device) ->Self{
        Uci {
            current_board : Board::new(model.clone(), device.clone()),
            depth: 5,
            transposition_table : TranspositionTable::init(),
            model: model,
            device 
        }
    }


    pub fn listen(&mut self){
        loop {
            // print!("> "); // Display a prompt
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap(); // Read user input
            let mut all_args = input.split_whitespace();

            let command = all_args.next().unwrap_or("");

            let params: Vec<&str> = all_args.collect();

            // println!("command: {}", command);

            // println!("Paramters: {:?}", params);

            if command.eq_ignore_ascii_case("quit") {
                println!("Goodbye!");
                break;
            }
            else {
                self.parse_input(command, params);
            }
        }
    }

    fn parse_input(&mut self, command :&str, params: Vec<&str>){
        match command {
            "isready" => self.isready(),
            "uci" => self.uci(),
            "go" => self.go(params),
            "position" => self.position(params),
            _ => self.unknown_command(command)
        }
    }

    fn isready(&self) {
        println!("readyok");
    }

    fn uci(&self) {
        println!("id name rough hook");
        println!("id author rough hook team");
        //insert options when done
        println!("uciok")
    }

    fn unknown_command(&self, input: &str) {
        println!("unknown command \"{}\". please enter a valid command.", input);
    }

    fn setoption(&self){
        //will return to later 
    }

    fn position(&mut self, input_params: Vec<&str>){
        let parameters = vec!["startpos", "fen", "moves"];
        let vector = self.filter_by_params(parameters, input_params);
        for (param, value) in vector {
            match param.as_str(){
                "startpos" => self.current_board = Board::new(self.model.clone(), self.device.clone()),
                "fen" => self.current_board = Board::from_fen(value, self.model.clone(), self.device.clone()),
                "moves" => {
                    let moves = value.split_whitespace();
                    for one_move in moves {
                        let (from, to) = one_move.split_at(2);
                        let (to, promotion) = if to.len() == 3 {
                            to.split_at(2)
                        }else {
                            (to, "")
                        };
                        let generated_moves = self.current_board.generate_legal_moves();
                        for gen_move in generated_moves {
                            if Square::from(gen_move.get_from()) == Square::from(from) && (Square::from(gen_move.get_to()) == Square::from(to)) {
                                match promotion {
                                    "n" => {
                                        match gen_move.get_flags() {
                                            Move::KNIGHT_PROMOTION | Move::KNIGHT_PROMO_CAPTURE => (),
                                            _ => continue 
                                        }
                                    },
                                    "b" => {
                                        match gen_move.get_flags() {
                                            Move::BISHOP_PROMOTION | Move::BISHOP_PROMO_CAPTURE => (),
                                            _ => continue 
                                        }
                                    },
                                    "r" => {
                                        match gen_move.get_flags() {
                                            Move::ROOK_PROMOTION | Move::ROOK_PROMO_CAPTURE => (),
                                            _ => continue 
                                        }
                                    },
                                    "q" => {
                                        match gen_move.get_flags() {
                                            Move::QUEEN_PROMOTION | Move::QUEEN_PROMO_CAPTURE => (),
                                            _ => continue 
                                        }
                                    },
                                    _ => ()
                                }
                                self.current_board.make_move(gen_move);
                            } 
                        }
                    }
                    // self.current_board.print_board();
                },
                _ => ()
            }
        }
    }

    fn ucinewgame(&mut self){
        //clears hash and any information collected abou previous games.
        //should call isready after to check if it's done clearing, which would return readyok
        self.current_board = Board::new(self.model.clone(), self.device.clone());
        self.transposition_table = TranspositionTable::init()
    }

    fn go(&mut self, input_params: Vec<&str>){
        let parameters = vec!["infinite", "depth", "nodes", "mate", "MultiPV",
        "UCI_showWDL", "searchmoves", "ponder", "wtime", "btime",
        "winc", "binc", "movestogo", "movetime", "perft"];
        let vector = self.filter_by_params(parameters,input_params);
        for (param, value) in vector {
            match param.as_str() {
                "depth" => self.depth = value.parse().expect("error parsing depth failed"),
                _ => ()
            };
        }

        let best_move = self.current_board.find_best_move(&mut self.transposition_table, self.depth);
        // self.current_board.print_board(); // print board
        println!("bestmove {}", best_move.0)
    }

    fn stop(&self){

    }

    fn ponderhit(&self){

    }

 

    fn filter_by_params(&self, defined_fields: Vec<&str>, input_params: Vec<&str>) -> Vec<(String, String)> {
        let mut found_fields: Vec<(String, String)> = Vec::new();
        let mut current_param : Option<&&str> = None; // Keeps track of the current parameter
        let mut combined_value = String::new();

        let mut iter = input_params.iter();
    
        while let Some(param) = iter.next() {
            if defined_fields.contains(param) {
                // Push the previous parameter and its value(s) to the list
                if let Some(found_param) = current_param {
                    found_fields.push((found_param.to_string(), combined_value.clone()));
                    combined_value.clear();
                }
                // Start a new parameter
                current_param = Some(param);
            } else {
                // Accumulate the value(s) for the current parameter
                if !combined_value.is_empty() {
                    combined_value.push(' ');
                }
                combined_value.push_str(param);
            }
        }
    
        // Add the last parameter and its values
        if let Some(found_param) = current_param {
            found_fields.push((found_param.to_string(), combined_value));
        }
    
        found_fields
    }
    
}