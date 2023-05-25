use crate::board::{BoardState, ChessCell};
use crate::evaluation;
use crate::move_generation::generate_moves;
use std::cmp;
pub fn minimax(
    board_state: &BoardState,
    depth: u8,
    alpha: i32,
    beta: i32,
    maximizing: bool,
) -> (i32, Option<(ChessCell, ChessCell)>) {
    if depth == 0 {
        return (evaluation::evaluate(&board_state), None);
    }
    let mut best_move = None;
    if maximizing {
        let mut max_eval = i32::MIN;
        for position in generate_moves(&board_state) {
            let (eval, _) = minimax(&position, depth - 1, alpha, beta, false);
            if eval > max_eval {
                max_eval = eval;
                best_move = position.last_move;
            }
            let alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break
            }
        }
        return (max_eval, best_move)   
    }
    else {
        let mut min_eval = i32::MAX;
        for position in generate_moves(&board_state) {
            let (eval, _) = minimax(&position, depth - 1, alpha, beta, true);
            if eval < min_eval {
                min_eval = eval;
                best_move = position.last_move;
            }
            let beta = cmp::min(beta, eval);
            if beta <= alpha {
                break
            }
        }
        return (min_eval, best_move)
    }
}