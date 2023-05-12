use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board::ChessCell;
pub mod engine;
pub mod evaluation;
mod ray_attacks;
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
    dbg!(board_state.white_bitboard, board_state.black_bitboard);
    let (best_eval, best_move) = engine::maxi(&board_state, 5);
    let best_move = best_move.unwrap();
    println!(
        "Evaluation is {} after playing the move {}",
        best_eval,
        ChessMove::from(best_move)
    );
    board_state.move_piece(best_move.0, best_move.1);
    dbg!(board_state.white_bitboard, board_state.black_bitboard);
}
