use rusty_brain::{board::{self, Board}, movement::Move, square::Square};
 
fn main() {
    println!("Core Engine");
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R2Pp1k/8/6P1/8 w - - 0 1".to_string());
    board.print_board();
    let moves = board.generate_legal_moves();
    println!("Len = {}" , moves.len());
    for m in &moves {
        println!("{}",m);
    }
    println!("capture log = {}" ,board.capture_log.len()) ;
    println!("castling_rights_log = {}" ,board.castling_rights_log.len());
    println!("move_log = {}" ,board.move_log.len()) ;
    println!("white prev pin = {}" ,board.white_prev_pins.len());
    println!("black prev pin = {}" ,board.black_prev_pins.len());

    println!("enemybitboard = {}" , board.bitboards.get_enemy_pieces(board.turn));
    println!("allybitboard = {}" , !board.bitboards.get_enemy_pieces(board.turn));



    // print!("---------------------------------------------------------------------\n");
    // board.make_move(moves[0]);
    // board.print_board();
    // let moves = board.generate_legal_moves();
    // for m in &moves {
    //     println!("{}",m);
    // }
    // print!("---------------------------------------------------------------------\n");

    // board.make_move(moves[12]);
    // board.print_board();
    // let moves = board.generate_legal_moves();
    // for m in &moves {
    //     println!("{}",m);
    // }
    // print!("---------------------------------------------------------------------\n");
    // board.undo_move();
    // board.print_board();
}
