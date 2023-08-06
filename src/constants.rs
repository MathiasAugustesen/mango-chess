use crate::board_elements::BitBoard;

pub const BOARD_START: usize = 2;
pub const BOARD_END: usize = 9;
pub const A_FILE: usize = 2;
pub const B_FILE: usize = 3;
pub const C_FILE: usize = 4;
pub const D_FILE: usize = 5;
pub const E_FILE: usize = 6;
pub const F_FILE: usize = 7;
pub const G_FILE: usize = 8;
pub const H_FILE: usize = 9;
pub const RANK_1: usize = 2;
pub const RANK_2: usize = 3;
pub const RANK_3: usize = 4;
pub const RANK_4: usize = 5;
pub const RANK_5: usize = 6;
pub const RANK_6: usize = 7;
pub const RANK_7: usize = 8;
pub const RANK_8: usize = 9;
pub const WHITE_STARTING_BITBOARD: BitBoard = BitBoard(0xFFFF);
pub const BLACK_STARTING_BITBOARD: BitBoard = BitBoard(WHITE_STARTING_BITBOARD.0 << 48);
pub const STARTING_FEN_STRING: &'static str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
