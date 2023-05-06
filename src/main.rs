use crate::board::BoardState;
pub mod board;
pub mod constants;
pub mod fen;
pub mod move_generation;
use crate::constants::*;
use board::PieceColor;
mod ray_attacks;
use crate::board::PieceKind::*;
use ray_attacks::*;
fn main() {
    let ray_attacks = generate_big_piece_ray_attacks(Queen);
    println!("{:?}", ray_attacks);
}
