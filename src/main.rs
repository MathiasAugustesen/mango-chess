use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board::ChessCell;
pub mod engine;
pub mod evaluation;
mod ray_attacks;
struct ChessMove(ChessCell, ChessCell);
impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}
fn main() {
    let board_state = BoardState::new_game();
    let best_score = engine::maxi(&board_state, 5);
    /*for mov in moves {
        let mut new_moves = generate_moves(&mov);
        total_moves.append(&mut new_moves);
    }
    */
    /*for mov in moves {
        let new_moves = generate_moves(&mov);
        for new_move in new_moves {
            let newer_moves = generate_moves(&new_move);
            for shiny_move in newer_moves {
                let shiny_moves = generate_moves(&shiny_move);
                for fancy_move in shiny_moves {
                    let mut fancy_moves = generate_moves(&fancy_move);
                    total_moves.append(&mut fancy_moves);
                }
            }
        }
    }
    */
    println!("{:?}", best_score);
}
