use crate::board::BoardState;
use crate::{evaluation, ChessMove};
use crate::move_generation::generate_moves;
pub fn negamax(
    board_state: &BoardState,
    depth: u8,
    alpha: i32,
    beta: i32,
    counter: &mut i32,
    prunes: &mut i32,
) -> i32 {
    if depth == 0 {
        *counter += 1;
        return evaluation::evaluate(&board_state);
    }
    let mut best_eval = -i32::MAX;
    let generated_moves = generate_moves(&board_state);
    //generated_moves.sort_unstable_by_key(evaluation::evaluate);
    for position in generated_moves {
        let eval = -negamax(&position, depth - 1, -beta, -alpha, counter, prunes);
        let alpha = alpha.max(eval);
        best_eval = eval.max(best_eval);
        if alpha >= beta {
            *prunes += 1;
            break;
        }
    }
    best_eval
}

pub fn search(
    board_state: &BoardState,
    depth: u8,
) -> (i32, Option<ChessMove>, i32, i32) {
    let mut alpha = -i32::MAX;
    let beta = i32::MAX;
    let mut best_eval = -i32::MAX;
    let mut best_move = None;
    let mut nodes_evaluated = 0;
    let mut prunes = 0;
    let possible_moves = generate_moves(board_state);
    if possible_moves.is_empty() {
        return (
            evaluation::evaluate(board_state),
            None,
            nodes_evaluated,
            prunes,
        );
    }
    for mov in possible_moves {
        let eval = -negamax(
            &mov,
            depth - 1,
            -beta,
            -alpha,
            &mut nodes_evaluated,
            &mut prunes,
        );
        if eval > best_eval {
            best_eval = eval;
            best_move = mov.last_move();
        }
        alpha = alpha.max(eval);
    }
    println!(
        "evaluated {} nodes and pruned {} branches",
        nodes_evaluated, prunes
    );
    (best_eval, best_move, nodes_evaluated, prunes)
}
