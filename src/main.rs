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
    let mut maximizing = true;

    loop {
        let (best_eval, best_move) = engine::minimax(&board_state, DEPTH, maximizing);
        let best_move = ChessMove::from(best_move.unwrap());
        println!("Evaluation is {} after the move {}", best_eval, best_move);
        board_state.make_move(best_move.0, best_move.1);
        maximizing = !maximizing;
    }
}
