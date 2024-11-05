use crate::board::{self, Board};

impl Board {
    pub fn evaluate(&mut self) -> i32 {
        let mut score: i32 = 0;

        //White
        score += self.bitboards.white_pawns.count_ones() as i32;
        score += (self.bitboards.white_knights.count_ones() * 3) as i32;
        score += (self.bitboards.white_bishops.count_ones() * 3) as i32;
        score += (self.bitboards.white_rooks.count_ones() * 5) as i32;
        score += (self.bitboards.white_queens.count_ones() * 9) as i32;

        //Black
        score -= self.bitboards.black_pawns.count_ones() as i32;
        score -= (self.bitboards.black_knights.count_ones() * 3) as i32;
        score -= (self.bitboards.black_bishops.count_ones() * 3) as i32;
        score -= (self.bitboards.black_rooks.count_ones() * 5) as i32;
        score -= (self.bitboards.black_queens.count_ones() * 9) as i32;

        score
    }
}