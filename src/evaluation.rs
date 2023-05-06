use crate::board::BoardState;
use crate::board::PieceColor::*;
const piece_values: [i32; 6] = [100, 300, 325, 500, 900, 10000];
//const PAWN_POSITION_VALUE: [u64; 64] =
pub fn evaluate(board_state: &BoardState) -> i32 {
    let board = board_state.board;
    let mut evaluation: i32 = 0;
    for white_piece in board_state.get_piece_positions(White) {
        let piece = board[white_piece.0][white_piece.1].piece();
        evaluation += piece_values[piece.index()];
    }
    for black_piece in board_state.get_piece_positions(Black) {
        let piece = board[black_piece.0][black_piece.1].piece();
        evaluation -= piece_values[piece.index()];
    }
    evaluation
}
