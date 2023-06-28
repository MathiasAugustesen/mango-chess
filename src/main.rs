use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board::ChessCell;
pub mod engine;
pub mod evaluation;
mod ray_attacks;
const DEPTH: u8 = 5;
use crate::board::PieceColor::*;
pub struct ChessMove(ChessCell, ChessCell);
impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}
impl From<(ChessCell, ChessCell)> for ChessMove {
    fn from(cells: (ChessCell, ChessCell)) -> ChessMove {
        ChessMove(cells.0, cells.1)
    }
}
fn main() {
    let mut board_state = BoardState::new_game();
    loop {
        let mut counter = 0;
        let mut prunes = 0;
        let (best_eval, best_move) = engine::negamax(
            &board_state,
            DEPTH,
            -i32::MAX,
            i32::MAX,
            &mut counter,
            &mut prunes,
        );
        let best_move = ChessMove::from(best_move.unwrap());
        let absolute_eval = best_eval
            * match board_state.to_move {
                White => 1,
                Black => -1,
            };
        println!(
            "Evaluation is {} after the move {}. Total searched nodes: {}. Pruned {} branches",
            absolute_eval, best_move, counter, prunes
        );
        board_state.make_move(best_move.0, best_move.1);
    }
}
