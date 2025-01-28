use rusty_brain::uci::Uci;

fn main() {
    let mut uci = Uci::new();
    uci.listen();
