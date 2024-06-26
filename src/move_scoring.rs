use crate::board_state::BoardState;
use crate::{
    board_elements::{ChessMove, Piece},
    evaluation::positional_value,
};

pub fn move_score(board_state: &BoardState, mov: ChessMove) -> i32 {
    let moving_piece = board_state.board.square(mov.start).piece().unwrap();

    let mut move_score = positional_value_delta(moving_piece, mov);
    if let Some(captured_piece) = board_state.board.square(mov.dest).piece() {
        move_score += captured_piece.value() - moving_piece.value() / 10;
    }
    move_score
}
#[inline]
pub fn positional_value_delta(piece: Piece, mov: ChessMove) -> i32 {
    positional_value(piece, mov.dest.as_index()) - positional_value(piece, mov.start.as_index())
}
