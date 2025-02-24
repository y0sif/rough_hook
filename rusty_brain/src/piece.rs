#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum PieceMapping {
    WK = 0,
    WP = 1,
    WN = 2,
    WB = 3,
    WR = 4,
    WQ = 5,
    BK = 6,
    BP = 7,
    BN = 8,
    BB = 9,
    BR = 10,
    BQ = 11,
}