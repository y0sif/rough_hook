#[cfg(test)]
mod tests {
    use crate::square::File;
    use crate::{board::Board, square::{Rank, Square}};

    #[test]
    fn test_pawns() {
        let mut board = Board::empty();
        board.bitboards.white_pawns = 1;
        for i in 0..64 {
            let moves = board.pawn_moves();

            if i > 7 && i < 16{
                assert_eq!(moves.len(), 2);
            }else if i > 55{
                assert_eq!(moves.len(), 0);
            }else{
                assert_eq!(moves.len(), 1);
            }

            board.bitboards.white_pawns <<= 1;

        }
              
    }
    
    #[test]
    fn test_knights() {
        let mut board = Board::empty();
        board.bitboards.white_knights = 1;

        for i in 0..64 {
            let square = Square::from(i);
            let rank = square.rank();
            let file = square.file();
            let moves = board.knight_moves();
            if square == Square::A1 || square == Square::H1 || square == Square::A8 || square == Square::H8 {
                assert_eq!(moves.len(), 2);
            }else if square == Square::B1 || square == Square::G1 || square == Square::B8 || square == Square::G8 || square == Square::A2 || square == Square::A7 || square == Square::H2 || square == Square::H7 {
                assert_eq!(moves.len(), 3);
            }else if rank == Rank::First || rank == Rank::Eighth {
                assert_eq!(moves.len(), 4);
            }else if square == Square::B2 || square == Square::G2 || square == Square::B7 || square == Square::G7 {
                assert_eq!(moves.len(), 4);
            }else if rank == Rank::Second || rank == Rank::Seventh || file == File::B || file == File::G {
                assert_eq!(moves.len(), 6);
            }else{
                assert_eq!(moves.len(), 8);
            }
            
            board.bitboards.white_knights <<= 1;
        }
    }

    #[test]
    fn test_bishops() {
        let mut board = Board::empty();
        board.bitboards.white_bishops = 1;

        for i in 0..64 {
            let moves = board.bishop_moves();
            let file = Square::from(i).file();
            let rank = Square::from(i).rank();
            if rank == Rank::First || rank == Rank::Eighth{
                assert_eq!(moves.len(), 7);
            }else if file == File::A || file == File::H {
                assert_eq!(moves.len(), 7);
            }else if rank == Rank::Second || rank == Rank::Seventh{
                assert_eq!(moves.len(), 9);
            }else if file == File::B || file == File::G {
                assert_eq!(moves.len(), 9);
            }else if file == File::C || file == File::F{
                assert_eq!(moves.len(), 11);
            }else if rank == Rank::Third || rank == Rank::Sixth{
                assert_eq!(moves.len(), 11);
            }else if file == File::D || file == File::E {
                assert_eq!(moves.len(), 13);
            }
            board.bitboards.white_bishops <<= 1;
        }
    }

    #[test]
    fn test_rooks() {
        let mut board = Board::empty();
        board.bitboards.white_rooks = 1;

        for _ in 0..64 {
            let moves = board.rook_moves();

            assert_eq!(moves.len(), 14);
            
            board.bitboards.white_rooks <<= 1;
        }
    }

    #[test]
    fn test_queens() {
        let mut board = Board::empty();
        board.bitboards.white_queens = 1;

        for i in 0..64 {
            let moves = board.queen_moves();
            let file = Square::from(i).file();
            let rank = Square::from(i).rank();
            if rank == Rank::First || rank == Rank::Eighth{
                assert_eq!(moves.len(), 7 + 14);
            }else if file == File::A || file == File::H {
                assert_eq!(moves.len(), 7 + 14);
            }else if rank == Rank::Second || rank == Rank::Seventh{
                assert_eq!(moves.len(), 9 + 14);
            }else if file == File::B || file == File::G {
                assert_eq!(moves.len(), 9 + 14);
            }else if file == File::C || file == File::F{
                assert_eq!(moves.len(), 11 + 14);
            }else if rank == Rank::Third || rank == Rank::Sixth{
                assert_eq!(moves.len(), 11 + 14);
            }else if file == File::D || file == File::E {
                assert_eq!(moves.len(), 13 + 14);
            }
            board.bitboards.white_queens <<= 1;
        }
        
    }
    
    #[test]
    fn test_king() {
        let mut board = Board::empty();
        board.bitboards.white_king = 1;

        for i in 0..64 {
            let moves = board.king_moves();
            if i == 0 || i == 7 || i == 56 || i == 63 {
                assert_eq!(moves.len(), 3);
            }else if i < 8 || i > 55{
                assert_eq!(moves.len(), 5);
            }else if Square::from(i).file() == File::A || Square::from(i).file() == File::H {
                assert_eq!(moves.len(), 5);
            }else{
                assert_eq!(moves.len(), 8);
            } 
            
            board.bitboards.white_king <<= 1;
        }
    }
}