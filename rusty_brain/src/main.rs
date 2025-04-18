use rusty_brain::{board::Board, nnue::NNUE, uci};
fn main() {
    // let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string());
    // board.print_board();
    // let eval = NNUE.evaluate(&board.white_accumulator, &board.black_accumulator);
    // println!("Evaluation: {}", eval);
    let mut uci = uci::Uci::new();
    uci.listen();
}