use rand::Rng;
use crate::board::{Board, Turn};

pub struct Zobrist {
    piece_key: [[u64; 12]; 64], // 1 value for each piece in each square
    black_to_move_key: u64, //1 value for black to move
    castling_rights_key: [u64; 4], // 4 values for castling rights
    en_passant_key: [u64; 64], // 64 values for each square
}

impl Zobrist {
    //init the Zobrist with random values
    pub fn init_zobrist() -> Self {
        let mut rng = rand::thread_rng();
        let mut piece_key = [[0u64; 12]; 64]; 
        let mut castling_rights_key = [0u64; 4]; 
        let mut en_passant_key = [0u64; 64];
        //black to move key is directly initialized below

        //pieces key
        for i in 0..64 {
            for j in 0..12 {
                piece_key[i][j] = rng.gen(); // Generate a random 64-bit value for each square and piece type
            }
        }

        //move key
        let black_to_move_key = rng.gen();

        //castling rights keys
        for i in 0..4 {
            castling_rights_key[i] = rng.gen();
        }

        //en passant key. One for each file (Wiki approach)
        //new approach. 64 values, one for each square.
        //use the en_passant_square value in the board struct
        for i in 0..64 {
            en_passant_key[i] = rng.gen();
        }

        Zobrist {
            piece_key,
            black_to_move_key, 
            castling_rights_key, 
            en_passant_key,
        }

    }

    // Hash the position
    // Key elements to hash:
    //  1) one number for each piece in each square
    //  2) one number to indicate it is black to move
    //  3) four numbers for castling rights
    //  4) eight numbers to indicate the files with valid en passant
    pub fn zobrist_hash(&self, board: &Board) -> u64 {
        let mut hash_value = 0u64;

        let mut occupied_squares = board.bitboards.get_ally_pieces(Turn::White)
            | board.bitboards.get_enemy_pieces(Turn::White);

        while occupied_squares != 0 {

            //get the index of the first '1' in the binary
            let square = occupied_squares.trailing_zeros();
            
            //en passant
            //new approach, check if that square is the same as the one in board
            // + 8 because the en passant square is the destination not current square 
            if board.en_passant_square.is_some_and(|sqr| square == sqr as u32 + 8) {
                hash_value ^= self.en_passant_key[square as usize];
            }

            //pieces key
            //for efficiency start with most probable pieces
            //pawns -> queen(s) -> knights -> bishops -> rooks -> king 
            //total of 12 pieces
            if board.bitboards.white_pawns & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][0];
            } 
            else if board.bitboards.black_pawns & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][1];
            }
            else if board.bitboards.white_queens & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][2];
            }
            else if board.bitboards.black_queens & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][3];
            }
            else if board.bitboards.white_knights & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][4];
            }
            else if board.bitboards.black_knights & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][5];
            }
            else if board.bitboards.white_bishops & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][6];
            } 
            else if board.bitboards.black_bishops & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][7];
            }
            else if board.bitboards.white_rooks & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][8];
            }
            else if board.bitboards.black_rooks & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][9];
            }
            else if board.bitboards.white_king & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][10];
            }
            else if board.bitboards.black_king & (1 << square) != 0 {
                hash_value ^= self.piece_key[square as usize][11];
            }

            occupied_squares &= occupied_squares - 1;

        }

        //black turn key
        if board.turn == Turn::Black {
            hash_value ^= self.black_to_move_key;
        }

        //castling rights key
        if board.castling_rights.white_king_side {
            hash_value ^= self.castling_rights_key[0];
        }
        if board.castling_rights.white_queen_side {
            hash_value ^= self.castling_rights_key[1];
        }
        if board.castling_rights.black_king_side {
            hash_value ^= self.castling_rights_key[2];
        }
        if board.castling_rights.black_queen_side {
            hash_value ^= self.castling_rights_key[3];
        }


        hash_value
    }
}