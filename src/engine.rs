use crate::board::BoardState;
use crate::evaluation;
use crate::move_generation::generate_moves;
pub fn maxi(board_state: &BoardState, depth: u8) -> i32 {
    if depth == 0 {
        return evaluation::evaluate(board_state);
    }
    let mut max_evaluation = i32::MIN;
    for mov in generate_moves(board_state) {
        let score = mini(&mov, depth - 1);
        if score > max_evaluation {
            max_evaluation = score;
        }
    }
    max_evaluation
}
fn mini(board_state: &BoardState, depth: u8) -> i32 {
    if depth == 0 {
        return evaluation::evaluate(board_state);
    }
    let mut min_evaluation = i32::MAX;
    for mov in generate_moves(board_state) {
        let score = maxi(&mov, depth - 1);
        if score < min_evaluation {
            min_evaluation = score;
        }
    }
    min_evaluation
}
