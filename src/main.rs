use crate::{board::BoardState, move_generation::generate_moves};
pub mod board;
pub mod board_elements;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board_elements::PieceColor;
pub mod engine;
pub mod evaluation;
pub mod move_ordering;
mod ray_attacks;
const DEPTH: u8 = 5;
use crate::board_elements::PieceColor::*;

pub enum GameResult {
    Winner(PieceColor),
    Draw,
}
impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Winner(game_winner) => {
                write!(f, "Checkmate! {game_winner} takes the crown.")
            }
            GameResult::Draw => write!(f, "The game ends in a draw."),
        }
    }
}
fn main() {
    let mut board_state = BoardState::new_game();
    let mut total_nodes_evaluated = 0;
    let mut total_prunes = 0;
    let mut moves = 0;
    loop {
        println!("{}", board_state.board);
        if generate_moves(&mut board_state).is_empty() {
            let game_winner = board_state.get_game_winner();
            println!("{game_winner}");
            return;
        }
        let (best_eval, best_move, nodes_evaluated, prunes) =
            engine::search(&board_state, DEPTH);
        total_nodes_evaluated += nodes_evaluated;
        total_prunes += prunes;
        moves += 1;
        let best_move = best_move.unwrap();
        let absolute_eval = best_eval
            * match board_state.to_move {
                White => 1,
                Black => -1,
            };
        println!(
            "Evaluation is {} with the move {}. After {} turns, searched a grand total of {} nodes and pruned {} branches in total.",
            absolute_eval, best_move, moves, total_nodes_evaluated, total_prunes
        );
        board_state.make_move(best_move);
    }
}
