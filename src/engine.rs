use crate::board::BoardState;
use crate::move_generation::{generate_moves, generate_pseudo_moves_for_player};
use crate::{evaluation, ChessMove};
pub fn negamax(
    board_state: &mut BoardState,
    depth: u8,
    alpha: i32,
    beta: i32,
    counter: &mut i32,
    prunes: &mut i32,
) -> i32 {
    if depth == 0 {
        *counter += 1;
        return board_state.eval();
    }
    let mut best_eval = -i32::MAX;
    let available_pseudo_moves = generate_pseudo_moves_for_player(board_state);
    //generated_moves.sort_unstable_by_key(evaluation::evaluate);
    for mov in available_pseudo_moves {
        board_state.make_move(mov);
        if !board_state.is_valid_move() {
            board_state.unmake_move();
            continue;
        }
        let eval = -negamax(board_state, depth - 1, -beta, -alpha, counter, prunes);
        board_state.unmake_move();
        let alpha = alpha.max(eval);
        best_eval = eval.max(best_eval);
        if alpha >= beta {
            *prunes += 1;
            break;
        }
    }
    best_eval
}

pub fn search(board_state: &mut BoardState, depth: u8) -> (i32, Option<ChessMove>, i32, i32) {
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
        board_state.make_move(mov);
        let eval = -negamax(
            board_state,
            depth - 1,
            -beta,
            -alpha,
            &mut nodes_evaluated,
            &mut prunes,
        );

        if eval > best_eval {
            best_eval = eval;
            best_move = board_state.last_move();
        }
        board_state.unmake_move();
        alpha = alpha.max(eval);
    }
    println!(
        "evaluated {} nodes and pruned {} branches",
        nodes_evaluated, prunes
    );
    (best_eval, best_move, nodes_evaluated, prunes)
}
