use crate::board::{BoardState, ChessCell};
use crate::evaluation;
use crate::move_generation::generate_moves;
use crate::ChessMove;
pub fn maxi(board_state: &BoardState, depth: u8) -> (i32, Option<(ChessCell, ChessCell)>) {
    if depth == 0 {
        let last_move = board_state.last_move.unwrap();
        return (
            evaluation::evaluate(board_state),
            Some((last_move.0, last_move.1)),
        );
    }
    let mut max_evaluation = i32::MIN;
    let mut best_move: Option<(ChessCell, ChessCell)> = None;
    for position in generate_moves(board_state) {
        let (score, mov) = mini(&position, depth - 1);
        if score >= max_evaluation {
            max_evaluation = score;
            best_move = mov;
        }
    }
    (max_evaluation, best_move)
}
fn mini(board_state: &BoardState, depth: u8) -> (i32, Option<(ChessCell, ChessCell)>) {
    if depth == 0 {
        let last_move = board_state.last_move.unwrap();
        return (
            evaluation::evaluate(board_state),
            Some((last_move.0, last_move.1)),
        );
    }
    let mut min_evaluation = i32::MAX;
    let mut best_move: Option<(ChessCell, ChessCell)> = None;
    for position in generate_moves(board_state) {
        let (score, mov) = maxi(&position, depth - 1);
        if score <= min_evaluation {
            min_evaluation = score;
            best_move = mov;
        }
    }

    (min_evaluation, best_move)
}
