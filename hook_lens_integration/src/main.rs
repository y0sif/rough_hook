use std::env;
// Add the following import if Board is from a crate, e.g., chess or shakmaty

// or define Board if it's your own type, or import from your project modules

use rusty_brain::{
    bitboards::{self, Bitboards},
    board::{self, Board},
    transposition::TranspositionTable,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let fen = &args[1];
        let mut fen_string = fen.to_string();
        fen_string.push_str(" w - - 0 1");
        let mut board = Board::from_fen(fen_string.to_string());
        let mut trans_table = TranspositionTable::init();
        let best_move = board.find_best_move(&mut trans_table, 5);
        println!("{}", best_move.0);

        //let fen_string = get_fen_string_from(board_image_path, model_path, 1);
        // println!("Fen String : {}", fen_string);

        // println!("|-------------------------- Testing ---------------------------|");
        // println!("| Testing in Debug Mode : \'cargo test --release -p hook_lens\'  |");
        // println!("| Testing in Development Mode : \'cargo test -p hook_lens\'      |");
        // println!("|--------------------------------------------------------------|");
    }
}
