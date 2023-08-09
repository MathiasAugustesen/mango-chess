use crate::board::BoardState;
use crate::{
    board_elements::{ChessMove, Piece},
    evaluation::positional_value,
};

pub fn move_sort(board_state: &BoardState, mov: ChessMove) -> i32 {
    if board_state.board.square(mov.start).piece().is_none() {
        println!("castling rights: {:?}", board_state.castling_rights);
        println!("White bb: {}\n Black bb: {}", board_state.white_bitboard, board_state.black_bitboard);
        println!("{}", mov);
        println!("{}", board_state.board)
    }
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
