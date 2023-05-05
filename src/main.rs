use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use crate::constants::*;
use std::str::FromStr;

fn main() {
    let board_state = BoardState::from_fen(STARTING_FEN_STRING);
    println!("{:?}", board_state.unwrap().board);
}
