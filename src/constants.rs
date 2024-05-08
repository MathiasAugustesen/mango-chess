use crate::board_elements::{BitBoard, ChessCell};

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

pub const A1: ChessCell = ChessCell(RANK_1, A_FILE);
pub const B1: ChessCell = ChessCell(RANK_1, B_FILE);
pub const C1: ChessCell = ChessCell(RANK_1, C_FILE);
pub const D1: ChessCell = ChessCell(RANK_1, D_FILE);
pub const E1: ChessCell = ChessCell(RANK_1, E_FILE);
pub const F1: ChessCell = ChessCell(RANK_1, F_FILE);
pub const G1: ChessCell = ChessCell(RANK_1, G_FILE);
pub const H1: ChessCell = ChessCell(RANK_1, H_FILE);

pub const A2: ChessCell = ChessCell(RANK_2, A_FILE);
pub const B2: ChessCell = ChessCell(RANK_2, B_FILE);
pub const C2: ChessCell = ChessCell(RANK_2, C_FILE);
pub const D2: ChessCell = ChessCell(RANK_2, D_FILE);
pub const E2: ChessCell = ChessCell(RANK_2, E_FILE);
pub const F2: ChessCell = ChessCell(RANK_2, F_FILE);
pub const G2: ChessCell = ChessCell(RANK_2, G_FILE);
pub const H2: ChessCell = ChessCell(RANK_2, H_FILE);

pub const A3: ChessCell = ChessCell(RANK_3, A_FILE);
pub const B3: ChessCell = ChessCell(RANK_3, B_FILE);
pub const C3: ChessCell = ChessCell(RANK_3, C_FILE);
pub const D3: ChessCell = ChessCell(RANK_3, D_FILE);
pub const E3: ChessCell = ChessCell(RANK_3, E_FILE);
pub const F3: ChessCell = ChessCell(RANK_3, F_FILE);
pub const G3: ChessCell = ChessCell(RANK_3, G_FILE);
pub const H3: ChessCell = ChessCell(RANK_3, H_FILE);

pub const A4: ChessCell = ChessCell(RANK_4, A_FILE);
pub const B4: ChessCell = ChessCell(RANK_4, B_FILE);
pub const C4: ChessCell = ChessCell(RANK_4, C_FILE);
pub const D4: ChessCell = ChessCell(RANK_4, D_FILE);
pub const E4: ChessCell = ChessCell(RANK_4, E_FILE);
pub const F4: ChessCell = ChessCell(RANK_4, F_FILE);
pub const G4: ChessCell = ChessCell(RANK_4, G_FILE);
pub const H4: ChessCell = ChessCell(RANK_4, H_FILE);

pub const A5: ChessCell = ChessCell(RANK_5, A_FILE);
pub const B5: ChessCell = ChessCell(RANK_5, B_FILE);
pub const C5: ChessCell = ChessCell(RANK_5, C_FILE);
pub const D5: ChessCell = ChessCell(RANK_5, D_FILE);
pub const E5: ChessCell = ChessCell(RANK_5, E_FILE);
pub const F5: ChessCell = ChessCell(RANK_5, F_FILE);
pub const G5: ChessCell = ChessCell(RANK_5, G_FILE);
pub const H5: ChessCell = ChessCell(RANK_5, H_FILE);

pub const A6: ChessCell = ChessCell(RANK_6, A_FILE);
pub const B6: ChessCell = ChessCell(RANK_6, B_FILE);
pub const C6: ChessCell = ChessCell(RANK_6, C_FILE);
pub const D6: ChessCell = ChessCell(RANK_6, D_FILE);
pub const E6: ChessCell = ChessCell(RANK_6, E_FILE);
pub const F6: ChessCell = ChessCell(RANK_6, F_FILE);
pub const G6: ChessCell = ChessCell(RANK_6, G_FILE);
pub const H6: ChessCell = ChessCell(RANK_6, H_FILE);

pub const A7: ChessCell = ChessCell(RANK_7, A_FILE);
pub const B7: ChessCell = ChessCell(RANK_7, B_FILE);
pub const C7: ChessCell = ChessCell(RANK_7, C_FILE);
pub const D7: ChessCell = ChessCell(RANK_7, D_FILE);
pub const E7: ChessCell = ChessCell(RANK_7, E_FILE);
pub const F7: ChessCell = ChessCell(RANK_7, F_FILE);
pub const G7: ChessCell = ChessCell(RANK_7, G_FILE);
pub const H7: ChessCell = ChessCell(RANK_7, H_FILE);

pub const A8: ChessCell = ChessCell(RANK_8, A_FILE);
pub const B8: ChessCell = ChessCell(RANK_8, B_FILE);
pub const C8: ChessCell = ChessCell(RANK_8, C_FILE);
pub const D8: ChessCell = ChessCell(RANK_8, D_FILE);
pub const E8: ChessCell = ChessCell(RANK_8, E_FILE);
pub const F8: ChessCell = ChessCell(RANK_8, F_FILE);
pub const G8: ChessCell = ChessCell(RANK_8, G_FILE);
pub const H8: ChessCell = ChessCell(RANK_8, H_FILE);
