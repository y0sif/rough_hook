use rusty_brain::uci;
fn main() {
    let mut uci = uci::Uci::new();
    uci.listen();
}