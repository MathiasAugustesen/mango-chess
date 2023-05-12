use crate::board::{BoardState, ChessCell};
use crate::evaluation;
use crate::move_generation::generate_moves;
use crate::ChessMove;
pub fn minimax(
    board_state: &BoardState,
    depth: u8,
    maximizing: bool,
) -> (i32, Option<(ChessCell, ChessCell)>) {
    if depth == 0 {
        return (evaluation::evaluate(&board_state), None);
    }
    let mut best_move = None;
    let mut best_score = match maximizing {
        true => i32::MIN,
        false => i32::MAX,
    };
    for position in generate_moves(&board_state) {
        let (score, _) = minimax(&position, depth - 1, !maximizing);
        if (maximizing && score > best_score) || (!maximizing && score < best_score) {
            best_score = score;
            best_move = position.last_move;
        }
    }
    (best_score, best_move)
}
