use rusty_brain::board::Board;

fn main() {
    println!("Core Engine");
    let mut board = Board::from_fen("7k/6r1/8/2N5/8/8/8/3K4 w - - 0 1".to_string());
    board.print_board();
}
