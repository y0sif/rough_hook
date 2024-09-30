pub struct Bitboards{
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,

    
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl Bitboards {
    // initialize the board at the starting position
    pub fn new() -> Self {
        Bitboards{
            white_pawns: 0x000000000000FF00,
            white_bishops: 0x0000000000000024,
            white_knights: 0x0000000000000042,
            white_rooks: 0x0000000000000081,
            white_queens: 0x0000000000000008,
            white_king: 0x0000000000000010,

            black_pawns: 0x00FF000000000000,
            black_bishops: 0x2400000000000000,
            black_knights: 0x4200000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x1000000000000000,
            black_king: 0x0800000000000000,
        }
    }    
}