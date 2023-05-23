use crate::board::BoardState;
use crate::board::Piece;
use crate::board::PieceColor::*;
use crate::board::PieceKind::*;
const PAWN_POSITION_VALUES: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 65, 60, 60, 70, 70, 60, 60, 65, 15, 15, 15, 20, 20, 15, 15, 15, 5, 5,
    0, 10, 10, 0, 5, 5, 0, -5, -20, 30, 30, -20, -5, 0, 10, 5, -10, -10, -10, -10, 5, 10, 5, 10,
    10, -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0, //<-a1 starts here
];
const KNIGHT_POSITION_VALUES: [i32; 64] = [
    -30, -20, -20, -10, -10, -20, -20, -30, -15, -10, -10, -5, -5, -10, -10, -15, 0, 5, 5, 10, 10,
    5, 5, 0, -5, 5, 10, 25, 25, 10, 5, -5, -5, 0, 20, 25, 25, 20, 0, -5, -5, -5, 20, 0, 0, 20, -5,
    -5, -20, -10, -10, -10, -10, -10, -10, -20, -50, -20, 0, -10, -10, 0, -20, -50,
];
const PIECE_VALUES: [i32; 6] = [100, 300, 325, 500, 900, 10000];
//const PAWN_POSITION_VALUE: [u64; 64] =
pub fn evaluate(board_state: &BoardState) -> i32 {
    let board = board_state.board;
    let mut evaluation: i32 = 0;
    for white_piece in board_state.get_piece_positions(White) {
        let idx = white_piece.as_index();
        let piece = board[white_piece.0][white_piece.1].piece();
        evaluation += evaluate_piece(piece, 64 - idx);
    }
    for black_piece in board_state.get_piece_positions(Black) {
        let idx = black_piece.as_index();
        let piece = board[black_piece.0][black_piece.1].piece();
        evaluation -= evaluate_piece(piece, idx);
    }
    evaluation
}
fn evaluate_piece(piece: Piece, idx: usize) -> i32 {
    let mut value = PIECE_VALUES[piece.index()];
    if piece.kind() == Pawn {
        value += PAWN_POSITION_VALUES[idx];
    } else if piece.kind() == Knight {
        value += KNIGHT_POSITION_VALUES[idx];
    }
    value
}
