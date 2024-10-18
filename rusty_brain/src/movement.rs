const QUIET_MOVE: u8 = 0x0;
const DOUBLE_PAWN_PUSH: u8 = 0x1;
const KING_CASTLE: u8 = 0x2;
const QUEEN_CASTLE: u8 = 0x3;
const CAPTURE: u8 = 0x4;
const EP_CAPTURE: u8 = 0x5;
const KNIGHT_PROMOTION: u8 = 0x8;
const BISHOP_PROMOTION: u8 = 0x9;
const ROOK_PROMOTION: u8 = 0xA;
const QUEEN_PROMOTION: u8 = 0xB;
const KNIGHT_PROMO_CAPTURE: u8 = 0xC;
const BISHOP_PROMO_CAPTURE: u8 = 0xD;
const ROOK_PROMO_CAPTURE: u8 = 0xE;
const QUEEN_PROMO_CAPTURE: u8 = 0xF;

// Each move is represented in 16 bits
// 6 bits "from" square, 6 bits "to" square, and 4 bits for flags
pub struct Movement {
    encoded_move: u16,
}

impl Movement {

    pub fn encode(from: u8, to: u8, flags: u8) -> Self {
        Self {
            encoded_move: ((flags & 0xF) as u16) << 12 | ((from & 0x3F) as u16) << 6 | (to & 0x3F) as u16,
        }
    }

    pub fn decode(encoded_move: u16) -> (u8, u8, u8) {
        ((encoded_move & 0x3F) as u8, 
         ((encoded_move >> 6) & 0x3F) as u8, 
         ((encoded_move >> 12) & 0x0F) as u8)
    }

    // Get squares or FLAGS
    pub fn get_to(&self) -> u8 {
        (self.encoded_move & 0x3F) as u8
    }
    pub fn get_from(&self) -> u8 {
        ((self.encoded_move >> 6) & 0x3F) as u8
    }
    pub fn get_flags(&self) -> u8 {
        ((self.encoded_move >> 12) & 0x0F) as u8
    }

    // Set Squares
    pub fn set_to(&mut self, to: u8) {
        self.encoded_move &= !0x3F;
        self.encoded_move |= (to & 0x3F) as u16;
    }
    pub fn set_from(&mut self, from: u8) {
        self.encoded_move &= !0xFC0;
        self.encoded_move |= ((from & 0x3F) as u16) << 6;
    }

    // Set or clear FLAGS
    pub fn set_flags(&mut self, flags: u8) {
        self.encoded_move &= !0xF000; // Clear existing flags
        self.encoded_move |= ((flags & 0xF) as u16) << 12; // Set new flags
    }   
    pub fn clear_flags(&mut self, flags: u8) {
        self.encoded_move &= !((flags & 0xF) as u16) << 12; // Clear specified flags
    }

    // Check FLAGS
    pub fn is_quiet_move(&self) -> bool {
        (self.encoded_move >> 12 & 0x0F) == QUIET_MOVE as u16
    }
    pub fn is_capture(&self) -> bool {
        (self.encoded_move & (CAPTURE as u16) << 12) != 0
    }
    pub fn is_double_pawn_push(&self) -> bool {
        (self.encoded_move & (DOUBLE_PAWN_PUSH as u16) << 12) != 0
    }
    pub fn is_king_castle(&self) -> bool {
        (self.encoded_move & (KING_CASTLE as u16) << 12) != 0
    }
    pub fn is_queen_castle(&self) -> bool {
        (self.encoded_move & (QUEEN_CASTLE as u16) << 12) != 0
    }
    pub fn is_ep_capture(&self) -> bool {
        (self.encoded_move & (EP_CAPTURE as u16) << 12) != 0
    }
    pub fn is_knight_promotion(&self) -> bool {
        (self.encoded_move & (KNIGHT_PROMOTION as u16) << 12) != 0
    }
    pub fn is_bishop_promotion(&self) -> bool {
        (self.encoded_move & (BISHOP_PROMOTION as u16) << 12) != 0
    }
    pub fn is_rook_promotion(&self) -> bool {
        (self.encoded_move & (ROOK_PROMOTION as u16) << 12) != 0
    }
    pub fn is_queen_promotion(&self) -> bool {
        (self.encoded_move & (QUEEN_PROMOTION as u16) << 12) != 0
    }
    pub fn is_knight_promo_capture(&self) -> bool {
        (self.encoded_move & (KNIGHT_PROMO_CAPTURE as u16) << 12) != 0
    }
    pub fn is_bishop_promo_capture(&self) -> bool {
        (self.encoded_move & (BISHOP_PROMO_CAPTURE as u16) << 12) != 0
    }
    pub fn is_rook_promo_capture(&self) -> bool {
        (self.encoded_move & (ROOK_PROMO_CAPTURE as u16) << 12) != 0
    }
    pub fn is_queen_promo_capture(&self) -> bool {
        (self.encoded_move & (QUEEN_PROMO_CAPTURE as u16) << 12) != 0
    }
}
