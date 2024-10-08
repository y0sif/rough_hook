use crate::{bitboards::Bitboards, board::Board};
pub struct Magic;

impl Magic {

    pub const ROOK_SHIFTS: [u8; 64] = [ 52, 52, 52, 52, 52, 52, 52, 52, 53, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 52, 54, 53, 53, 53, 53, 54, 53, 52, 53, 54, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52];
    pub const BISHOP_SHIFTS: [u8; 64] = [ 58, 60, 59, 59, 59, 59, 60, 58, 60, 59, 59, 59, 59, 59, 59, 60, 59, 59, 57, 57, 57, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59, 60, 60, 59, 59, 59, 59, 60, 60, 58, 60, 59, 59, 59, 59, 59, 58 ];

    pub const ROOK_MAGICS: [u64; 64] = [ 468374916371625120, 18428729537625841661, 2531023729696186408, 6093370314119450896, 13830552789156493815, 16134110446239088507, 12677615322350354425, 5404321144167858432, 2111097758984580, 18428720740584907710, 17293734603602787839, 4938760079889530922, 7699325603589095390, 9078693890218258431, 578149610753690728, 9496543503900033792, 1155209038552629657, 9224076274589515780, 1835781998207181184, 509120063316431138, 16634043024132535807, 18446673631917146111, 9623686630121410312, 4648737361302392899, 738591182849868645, 1732936432546219272, 2400543327507449856, 5188164365601475096, 10414575345181196316, 1162492212166789136, 9396848738060210946, 622413200109881612, 7998357718131801918, 7719627227008073923, 16181433497662382080, 18441958655457754079, 1267153596645440, 18446726464209379263, 1214021438038606600, 4650128814733526084, 9656144899867951104, 18444421868610287615, 3695311799139303489, 10597006226145476632, 18436046904206950398, 18446726472933277663, 3458977943764860944, 39125045590687766, 9227453435446560384, 6476955465732358656, 1270314852531077632, 2882448553461416064, 11547238928203796481, 1856618300822323264, 2573991788166144, 4936544992551831040, 13690941749405253631, 15852669863439351807, 18302628748190527413, 12682135449552027479, 13830554446930287982, 18302628782487371519, 7924083509981736956, 4734295326018586370 ];
    pub const BISHOP_MAGICS: [u64; 64] = [ 16509839532542417919, 14391803910955204223, 1848771770702627364, 347925068195328958, 5189277761285652493, 3750937732777063343, 18429848470517967340, 17870072066711748607, 16715520087474960373, 2459353627279607168, 7061705824611107232, 8089129053103260512, 7414579821471224013, 9520647030890121554, 17142940634164625405, 9187037984654475102, 4933695867036173873, 3035992416931960321, 15052160563071165696, 5876081268917084809, 1153484746652717320, 6365855841584713735, 2463646859659644933, 1453259901463176960, 9808859429721908488, 2829141021535244552, 576619101540319252, 5804014844877275314, 4774660099383771136, 328785038479458864, 2360590652863023124, 569550314443282, 17563974527758635567, 11698101887533589556, 5764964460729992192, 6953579832080335136, 1318441160687747328, 8090717009753444376, 16751172641200572929, 5558033503209157252, 17100156536247493656, 7899286223048400564, 4845135427956654145, 2368485888099072, 2399033289953272320, 6976678428284034058, 3134241565013966284, 8661609558376259840, 17275805361393991679, 15391050065516657151, 11529206229534274423, 9876416274250600448, 16432792402597134585, 11975705497012863580, 11457135419348969979, 9763749252098620046, 16960553411078512574, 15563877356819111679, 14994736884583272463, 9441297368950544394, 14537646123432199168, 9888547162215157388, 18140215579194907366, 18374682062228545019 ];

    pub fn create_all_blocker_configs(move_mask: u64) -> Vec<u64> {
        let mut move_square_indices: Vec<u8> = Vec::new();

        for i in 0..64 {
            if (move_mask >> i) & 1 == 1 {
                move_square_indices.push(i);
            }
        }

        let patterns_num = 1 << move_square_indices.len();
        let mut blockers_configs = vec![0u64; patterns_num];

        for pattern_index in 0..patterns_num {
            for bit_index in 0..move_square_indices.len() {
                let bit = (pattern_index >> bit_index) & 1;
                blockers_configs[pattern_index] |= (bit as u64) << move_square_indices[bit_index];
            }
        }
        blockers_configs
    }
    
    pub fn create_rook_table(square: u8, magic: u64, left_shift: u8) -> Vec<u64> {
        let num_of_bits = 64 - left_shift;
        let lookup_size = 1 << num_of_bits;
        let mut rook_moves_table = vec![0u64; lookup_size]; 

        let move_mask = Bitboards::rook_mask_ex(square);
        let blockers_configs = Self::create_all_blocker_configs(move_mask);
        let piece_position = 1 << square;        

        for blocker in blockers_configs {
            let occupied_bitboard = piece_position & blocker;
            let legal_moves = Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_north)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_south)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_east)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_west);
            let index = blocker.wrapping_mul(magic) >> left_shift;
            rook_moves_table[index as usize] = legal_moves;
        }
        rook_moves_table
    }
    
    pub fn rook_attacks() -> [Vec<u64>; 64]{
        let mut rook_attacks = [const { Vec::new() }; 64];
        for i in 0..64 {
            rook_attacks[i] = Self::create_rook_table(i as u8, Self::ROOK_MAGICS[i], Self::ROOK_SHIFTS[i]);
        }
        rook_attacks
    }

    pub fn create_bishop_table(square: u8, magic: u64, left_shift: u8) -> Vec<u64> {
        let num_of_bits = 64 - left_shift;
        let lookup_size = 1 << num_of_bits;
        let mut bishop_moves_table = vec![0u64; lookup_size]; 

        let move_mask = Bitboards::bishop_mask_ex(square);
        let blockers_configs = Self::create_all_blocker_configs(move_mask);
        let piece_position = 1 << square;        

        for blocker in blockers_configs {
            let occupied_bitboard = piece_position & blocker;
            let legal_moves = Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_north_east)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_north_west)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_south_east)
                                  |Board::get_sliding_bitboard(piece_position, occupied_bitboard, blocker, Bitboards::move_south_west);
            let index = blocker.wrapping_mul(magic) >> left_shift;
            bishop_moves_table[index as usize] = legal_moves;
        }
        bishop_moves_table
    }
    
    pub fn bishop_attacks() -> [Vec<u64>; 64]{
        let mut bishop_attacks = [const { Vec::new() }; 64];
        for i in 0..64 {
            bishop_attacks[i] = Self::create_bishop_table(i as u8, Self::BISHOP_MAGICS[i], Self::BISHOP_SHIFTS[i]);
        }
        bishop_attacks
    }

}