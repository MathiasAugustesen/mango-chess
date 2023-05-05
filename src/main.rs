use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use crate::constants::*;
use board::PieceColor;

fn main() {
    let board_state =
        BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2");
    println!(
        "{:?}",
        &board_state.unwrap().get_piece_positions(PieceColor::White)
    );
}
