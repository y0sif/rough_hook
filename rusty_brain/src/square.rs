use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    First = 0,
    Second,
    Third,
    Forth,
    Fifth,
    Sixth,
    Seventh,
    Eighth, 
}

impl Rank {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

// to print the squares values as string
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = (self.clone() as u8) / 8 + 1;
        let file = (self.clone() as u8) % 8;
        let file_char = char::from(b'A' + file);
        write!(f, "{}{}", file_char, rank)
    }
}

impl From<u8> for Square {
    fn from(index: u8) -> Self {
        match index {
            0 => Square::A1,  1 => Square::B1,  2 => Square::C1,  3 => Square::D1,
            4 => Square::E1,  5 => Square::F1,  6 => Square::G1,  7 => Square::H1,
            8 => Square::A2,  9 => Square::B2, 10 => Square::C2, 11 => Square::D2,
            12 => Square::E2, 13 => Square::F2, 14 => Square::G2, 15 => Square::H2,
            16 => Square::A3, 17 => Square::B3, 18 => Square::C3, 19 => Square::D3,
            20 => Square::E3, 21 => Square::F3, 22 => Square::G3, 23 => Square::H3,
            24 => Square::A4, 25 => Square::B4, 26 => Square::C4, 27 => Square::D4,
            28 => Square::E4, 29 => Square::F4, 30 => Square::G4, 31 => Square::H4,
            32 => Square::A5, 33 => Square::B5, 34 => Square::C5, 35 => Square::D5,
            36 => Square::E5, 37 => Square::F5, 38 => Square::G5, 39 => Square::H5,
            40 => Square::A6, 41 => Square::B6, 42 => Square::C6, 43 => Square::D6,
            44 => Square::E6, 45 => Square::F6, 46 => Square::G6, 47 => Square::H6,
            48 => Square::A7, 49 => Square::B7, 50 => Square::C7, 51 => Square::D7,
            52 => Square::E7, 53 => Square::F7, 54 => Square::G7, 55 => Square::H7,
            56 => Square::A8, 57 => Square::B8, 58 => Square::C8, 59 => Square::D8,
            60 => Square::E8, 61 => Square::F8, 62 => Square::G8, 63 => Square::H8,
            _ => panic!("Invalid square index: {}", index),
        }
    }
}

impl From<&str> for Square {
    fn from(value: &str) -> Self {
        match value {
            "a1" => Square::A1, "b1" => Square::B1, "c1" => Square::C1, "d1" => Square::D1,
            "e1" => Square::E1, "f1" => Square::F1, "g1" => Square::G1, "h1" => Square::H1,
            "a2" => Square::A2, "b2" => Square::B2, "c2" => Square::C2, "d2" => Square::D2,
            "e2" => Square::E2, "f2" => Square::F2, "g2" => Square::G2, "h2" => Square::H2,
            "a3" => Square::A3, "b3" => Square::B3, "c3" => Square::C3, "d3" => Square::D3,
            "e3" => Square::E3, "f3" => Square::F3, "g3" => Square::G3, "h3" => Square::H3,
            "a4" => Square::A4, "b4" => Square::B4, "c4" => Square::C4, "d4" => Square::D4,
            "e4" => Square::E4, "f4" => Square::F4, "g4" => Square::G4, "h4" => Square::H4,
            "a5" => Square::A5, "b5" => Square::B5, "c5" => Square::C5, "d5" => Square::D5,
            "e5" => Square::E5, "f5" => Square::F5, "g5" => Square::G5, "h5" => Square::H5,
            "a6" => Square::A6, "b6" => Square::B6, "c6" => Square::C6, "d6" => Square::D6,
            "e6" => Square::E6, "f6" => Square::F6, "g6" => Square::G6, "h6" => Square::H6,
            "a7" => Square::A7, "b7" => Square::B7, "c7" => Square::C7, "d7" => Square::D7,
            "e7" => Square::E7, "f7" => Square::F7, "g7" => Square::G7, "h7" => Square::H7,
            "a8" => Square::A8, "b8" => Square::B8, "c8" => Square::C8, "d8" => Square::D8,
            "e8" => Square::E8, "f8" => Square::F8, "g8" => Square::G8, "h8" => Square::H8,
            _ => panic!("Invalid square string: {}", value),
        }
    }
}

impl Square {
    pub fn rank(self) -> Rank {
        let rank_index = (self as u8) / 8;
        match rank_index {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Forth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid rank index: {}", rank_index),
        }
    }

    pub fn file(self) -> File {
        let file_index = (self as u8) % 8;
        match file_index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file index: {}", file_index),
        }
    }
}