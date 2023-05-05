use crate::board::BoardState;
pub mod board;
pub mod constants;
use crate::constants::*;
use std::str::FromStr;
mod fen;
mod move_generation;
fn main() {
    let board_state = BoardState::from_fen(STARTING_FEN_STRING);
    println!("{:?}", board_state.unwrap().board);
}
