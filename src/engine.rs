use crate::board::BoardState;
use crate::board_elements::ChessMove;
use crate::move_generation::generate_pseudo_moves_for_player;
use crate::move_ordering::move_sort;
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
        return board_state.eval * board_state.to_move.signum();
    }
    let mut best_eval = -i32::MAX;
    let mut available_pseudo_moves = generate_pseudo_moves_for_player(board_state);

    available_pseudo_moves.sort_by_cached_key(|&mov| -move_sort(&board_state, mov));
    for mov in available_pseudo_moves {
        let mut copy_board = board_state.clone();
        copy_board.make_move(mov);
        if !copy_board.is_valid_move() {
            continue;
        }
        let eval = -negamax(&copy_board, depth - 1, -beta, -alpha, counter, prunes);
        let alpha = alpha.max(eval);
        best_eval = eval.max(best_eval);
        if alpha >= beta {
            *prunes += 1;
            break;
        }
    }
    best_eval
}

pub fn search(board_state: &BoardState, depth: u8) -> (i32, Option<ChessMove>, i32, i32) {
    let mut alpha = -i32::MAX;
    let beta = i32::MAX;
    let mut best_eval = -i32::MAX;
    let mut best_move = None;
    let mut nodes_evaluated = 0;
    let mut prunes = 0;
    let mut possible_moves = generate_pseudo_moves_for_player(board_state);

    possible_moves.sort_by_cached_key(|&mov| -move_sort(&board_state, mov));
    for mov in possible_moves {
        let mut copy_board = board_state.clone();
        copy_board.make_move(mov);
        if !copy_board.is_valid_move() {
            continue;
        }
        let eval = -negamax(
            &copy_board,
            depth - 1,
            -beta,
            -alpha,
            &mut nodes_evaluated,
            &mut prunes,
        );

        if eval > best_eval {
            best_eval = eval;
            best_move = copy_board.last_move;
        }
        alpha = alpha.max(eval);
    }
    println!(
        "evaluated {} nodes and pruned {} branches",
        nodes_evaluated, prunes
    );
    (best_eval, best_move, nodes_evaluated, prunes)
}
