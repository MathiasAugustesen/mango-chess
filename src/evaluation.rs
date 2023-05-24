use crate::board::BoardState;
use crate::board::PieceColor::*;
const PIECE_VALUES: [i32; 6] = [100, 300, 325, 500, 900, 10000];
#[rustfmt::skip]
const PAWN_POSITION_VALUES: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0, 
    65,  60,  60,  70,  70,  60,  60,  65, 
    15,  15,  15,  20,  20,  15,  15,  15, 
    5,   5,   0,   10,  10,  0,   5,   5, 
    0,   -5,  -20, 30,  30,  -20, -5,  0, 
    10,  5,   -10, -10, -10, -10, 5,   10, 
    5,   10,  10,  -20, -20,  10, 10,  5, 
    0,   0,   0,   0,   0,   0,   0,   0, //<-a1 starts here
];
#[rustfmt::skip]
const KNIGHT_POSITION_VALUES: [i32; 64] = [
    -30, -20, -20, -10, -10, -20, -20, -30, 
    -15, -10, -10, -5,  -5,  -10, -10, -15, 
    0,   5,   5,   10,  10,  5,   5,   0, 
    -5,  5,   10,  25,  25,  10,  5,   -5, 
    -5,  0,   20,  25,  25,  20,  0,   -5, 
    -5,  -5,  20,  0,   0,   20,  -5,  -5, 
    -20, -10, -10, -10, -10, -10, -10, -20, 
    -50, -20, 0,   -10, -10, 0,   -20, -50,
];
#[rustfmt::skip]
const BISHOP_POSITION_VALUES: [i32; 64] = [
    -10, -15, -15, -20, -20, -15, -15, -10,
    -5,  0,   -10, -10, -10, -10, -5,  0,
    10,  5,   5,   12,  12,  5,   5,   10,
    0,   20,  5,   10,  10,  5,   20,  0,
    -5,  0,   20,  10,  10,  20,  0,   -5,
    -5,  0,   5,   15,  15,  5,   0,   -5,
    -5,  10,  0,   5,   5,   0,   10,  -5,
    -10, 0,   -15, 0,   0,   -15, 0,   -10,
];
#[rustfmt::skip]
const ROOK_POSITION_VALUES: [i32; 64] = [
    10,  10,  15,  15,  15,  15,  10,  10,
    20,  25,  30,  30,  30,  30,  25,  20,
    5,   10,  15,  15,  15,  15,  10,  5,
    5,   5,   10,  10,  10,  10,  5,   5,
    5,   5,   10,  10,  10,  10,  5,   5,
    0,   0,   5,   10,  10,  5,   0,   0,
    -5,  -5,  0,   5,   5,   0,   -5,  -5, 
    -10, -10, 15,  20,  20,  15,  -10, -10,
];
#[rustfmt::skip]
const QUEEN_POSITION_VALUES: [i32; 64] = [
    -10, -10, -10, -10, -10, -10, -10, -10,
    -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
    -10, -10, -5,  -5,  -5,  -5,  -10, -10,
    -5,  -5,  -5,  -5,  -5,  -5,  -5,  -5,
    -10, 0,   0,   0,   0,   0,   0,   -5,
    -15, -10, 5,   0,   5,   0,   0,   -5,
    -15, -10, -5,  5,   0,   5,   -15, -10,
    -20, -15, -5,  -5,  0,   -5,  -15, -20,
];
#[rustfmt::skip]
const KING_POSITION_VALUES: [i32; 64] = [
    -50, -50, -50, -50, -50, -50, -50, -50,
    -35, -35, -35, -35, -35, -35, -35, -35,
    -25, -25, 5,   -25, -25, 5,   -25, -25,
    -15, -15, 15,  0,   0,   -15, -15, -15,
    -15, -15, 15,  0,   0,   -15, -15, -15,
    -15, -15, -15, -15, -15, -15, -15, -15,
    -20, -5,  5,   -15, -15, -20, -10, -20,
    5,   15,  1,   0,   1,   10,  15,  5,
];
//const PAWN_POSITION_VALUE: [u64; 64] =
pub fn evaluate(board_state: &BoardState) -> i32 {
    let board = board_state.board;
    let mut evaluation: i32 = 0;
    for white_piece in board_state.get_piece_positions(White) {
        let piece = board[white_piece.0][white_piece.1].piece();
        evaluation += PIECE_VALUES[piece.index()];
    }
    for black_piece in board_state.get_piece_positions(Black) {
        let piece = board[black_piece.0][black_piece.1].piece();
        evaluation -= PIECE_VALUES[piece.index()];
    }
    evaluation
}
