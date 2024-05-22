use crate::{board_state::BoardState, move_generation::generate_moves};
pub mod board_elements;
pub mod board_state;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board_elements::PieceColor;
pub mod chess_board;
pub mod evaluation;
pub mod move_scoring;
mod ray_attacks;
pub mod search;
mod zobrist_hashing;
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
    let mut moves = 0;
    loop {
        println!("{}", board_state.board);
        if generate_moves(&board_state).is_empty() {
            let game_winner = board_state.get_game_winner();
            println!("{game_winner}");
            return;
        }
        let (best_eval, best_move) = search::search(&board_state, DEPTH);
        moves += 1;
        let best_move = best_move.unwrap();
        let absolute_eval = best_eval
            * match board_state.to_move {
                White => 1,
                Black => -1,
            };
        println!(
            "Evaluation is {} with the move {}. Total moves: {}",
            absolute_eval, best_move, moves
        );
        board_state.make_move(best_move);
    }
}
