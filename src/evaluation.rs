use crate::board::BoardState;
use crate::board::Piece;
use crate::board::PieceColor::*;
pub fn evaluate(board_state: &BoardState) -> i32 {
    let board = board_state.board;
    let mut evaluation: i32 = 0;
    // add white evaluation
    evaluation += board_state
        .get_piece_positions(White)
        .into_iter()
        .map(|pos| evaluate_piece(board[pos.0][pos.1].piece(), 63 - pos.as_index()))
        .sum::<i32>();
    // subtract black evaluation
    evaluation -= board_state
        .get_piece_positions(Black)
        .into_iter()
        .map(|pos| evaluate_piece(board[pos.0][pos.1].piece(), pos.as_index()))
        .sum::<i32>();
    evaluation * board_state.to_move.relative_value()
}
fn evaluate_piece(piece: Piece, pos: usize) -> i32 {
    PIECE_VALUES[piece.index()] + POSITION_VALUES[piece.index()][pos]
}
pub fn evaluate_game_end() -> i32 {
    0
}
const PIECE_VALUES: [i32; 6] = [100, 300, 325, 500, 900, 10000];
const POSITION_VALUES: [[i32; 64]; 6] = [
    PAWN_POSITION_VALUES,
    KNIGHT_POSITION_VALUES,
    BISHOP_POSITION_VALUES,
    ROOK_POSITION_VALUES,
    QUEEN_POSITION_VALUES,
    KING_POSITION_VALUES,
];
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
