use crate::{board::BoardState, move_generation::generate_moves};
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use board::{ChessCell, PieceColor, CastlingType};
use constants::*;
pub mod engine;
pub mod evaluation;
pub mod move_ordering;
mod ray_attacks;
const DEPTH: u8 = 5;
use crate::board::PieceColor::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    pub start: ChessCell,
    pub dest: ChessCell,
}
impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.dest)
    }
}
impl From<(ChessCell, ChessCell)> for ChessMove {
    fn from(cells: (ChessCell, ChessCell)) -> ChessMove {
        ChessMove {
            start: cells.0,
            dest: cells.1,
        }
    }
}
impl From<CastlingType> for ChessMove {
    fn from(value: CastlingType) -> Self {
        let (start, dest) =
        match value {
            CastlingType::WhiteKingSide => ((RANK_1, E_FILE), (RANK_1, G_FILE)),
            CastlingType::WhiteQueenSide => ((RANK_1, E_FILE), (RANK_1, C_FILE)),
            CastlingType::BlackKingSide => ((RANK_8, E_FILE), (RANK_8, G_FILE)),
            CastlingType::BlackQueenSide => ((RANK_8, E_FILE), (RANK_8, C_FILE))
        };
        ChessMove {
            start: ChessCell::from(start),
            dest: ChessCell::from(dest),
        }
    }

}
impl ChessMove {
    pub fn reverse(self) -> ChessMove {
        ChessMove {
            start: self.dest,
            dest: self.start,
        }
    }
}
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
        if generate_moves(&mut board_state).is_empty() {
            let game_winner = board_state.get_game_winner();
            println!("{game_winner}");
            return;
        }
        let (best_eval, best_move, nodes_evaluated, prunes) =
            engine::search(&mut board_state, DEPTH);
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
