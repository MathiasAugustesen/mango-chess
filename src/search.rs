use crate::board_elements::ChessMove;
use crate::board_state::BoardState;
use crate::move_generation::generate_pseudo_moves_for_player;
use crate::move_scoring::move_score;

pub fn search(board_state: &BoardState, depth: u8) -> (i32, Option<ChessMove>) {
    let mut alpha = -i32::MAX;
    let beta = i32::MAX;
    let mut best_eval = -i32::MAX;
    let mut best_move = None;
    let mut possible_moves = generate_pseudo_moves_for_player(board_state);

    possible_moves.sort_by_cached_key(|&mov| -move_score(board_state, mov));
    for mov in possible_moves {
        let mut copy_board = board_state.clone();
        copy_board.make_move(mov);
        if !copy_board.is_valid_move() {
            continue;
        }
        let eval = -negamax(&copy_board, depth - 1, -beta, -alpha);

        if eval > best_eval {
            best_eval = eval;
            best_move = copy_board.last_move;
        }
        alpha = alpha.max(eval);
    }
    (best_eval, best_move)
}

fn negamax(board_state: &BoardState, depth: u8, alpha: i32, beta: i32) -> i32 {
    if board_state.is_terminal() {
        return board_state.terminal_eval();
    }
    if depth == 0 {
        return board_state.pov_eval();
    }
    let mut best_eval = -i32::MAX;
    let mut available_pseudo_moves = generate_pseudo_moves_for_player(board_state);

    available_pseudo_moves.sort_by_cached_key(|&mov| -move_score(board_state, mov));
    for mov in available_pseudo_moves {
        let mut copy_board = board_state.clone();
        copy_board.make_move(mov);
        if !copy_board.is_valid_move() {
            continue;
        }
        let eval = -negamax(&copy_board, depth - 1, -beta, -alpha);
        let alpha = alpha.max(eval);
        best_eval = eval.max(best_eval);
        if alpha >= beta {
            break;
        }
    }
    best_eval
}

#[cfg(test)]
mod tests {
    use crate::board_state::BoardState;
    use crate::constants::*;

    use super::search;

    #[test]
    fn mate_in_two_has_correct_eval_and_move() {
        let board_state = BoardState::from_fen(
            "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0",
        )
        .unwrap();

        let (eval, best_move) = search(&board_state, 3);

        assert!(eval > 1_000_000);
        assert_eq!(best_move, Some((D5, F6).into()))
    }
}
