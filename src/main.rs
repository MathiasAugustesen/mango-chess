use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use crate::constants::*;
use crate::move_generation::generate_moves;
use board::PieceColor;
mod ray_attacks;
use crate::board::PieceKind::*;
use ray_attacks::*;
use std::backtrace::Backtrace;
fn main() {
    let board_state = BoardState::new_game();
    generate_moves(&board_state);
}
