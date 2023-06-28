use crate::board::{BoardState, ChessCell};
use crate::evaluation;
use crate::move_generation::generate_moves;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub fn negamax(
    board_state: &BoardState,
    depth: u8,
    alpha: i32,
    beta: i32,
    counter: &mut i32,
    prunes: &mut i32,
) -> (i32, Option<(ChessCell, ChessCell)>) {
    if depth == 0 {
        *counter += 1;
        return (evaluation::evaluate(&board_state), None);
    }
    let mut best_move = None;
    let mut best_score = -i32::MAX;
    let mut generated_moves = generate_moves(&board_state);
    for position in generated_moves {
        let (score, _) = negamax(&position, depth - 1, -beta, -alpha, counter, prunes);
        let score = -score;
        let alpha = alpha.max(score);
        if score >= best_score {
            best_score = score;
            best_move = position.last_move;
        }
        if alpha >= beta {
            *prunes += 1;
            break;
        }
    }
    (best_score, best_move)
}
