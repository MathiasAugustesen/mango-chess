use crate::board::BoardState;
use crate::board::ChessCell;
use crate::board::Piece;
use crate::board::PieceColor;
use crate::board::PieceColor::*;
use crate::board::PieceKind::*;
use crate::board::Square;
use crate::constants::*;
pub fn generate_moves(board_state: &BoardState) -> Vec<BoardState> {
    let board = board_state.board;
    let mut new_moves: Vec<BoardState> = Vec::new();
    let player_to_move = board_state.to_move;
    let piece_positions = board_state.get_piece_positions(player_to_move);
    for position in piece_positions {
        if let Square::Full(piece) = board[position.0][position.1] {
            generate_moves_for_piece(piece, board_state, position, &mut new_moves);
        }
    }
    Vec::new()
}
pub fn generate_moves_for_piece(
    piece: Piece,
    board_state: &BoardState,
    position: ChessCell,
    new_moves: &mut Vec<BoardState>,
) {
    let mut moves: Vec<ChessCell> = Vec::new();
    match piece.kind() {
        _ => panic!(),
    }
}
fn white_pawn_moves(board_state: &BoardState, position: ChessCell, moves: &mut Vec<ChessCell>) {
    let board = board_state.board;
    let ChessCell(rank, file) = position;
    let left_capture = board[rank + 1][file - 1];
    if left_capture.is_enemy_of(White) {
        moves.push(ChessCell(rank + 1, file - 1))
    }
    let right_capture = board[rank + 1][file + 1];
    if right_capture.is_enemy_of(White) {
        moves.push(ChessCell(rank + 1, file + 1))
    }
    let one_forward = board[rank + 1][file];
    if one_forward.is_empty() {
        moves.push(ChessCell(rank + 1, file));
        let two_forward = board[rank + 2][file];
        if rank == RANK_7 && board[rank + 2][file].is_empty() {
            moves.push(ChessCell(rank + 2, file));
        }
    }
}

fn black_pawn_moves(board_state: &BoardState, position: ChessCell, moves: &mut Vec<ChessCell>) {
    let board = board_state.board;
    let ChessCell(rank, file) = position;
    let left_capture = board[rank - 1][file - 1];
    if left_capture.is_enemy_of(Black) {
        moves.push(ChessCell(rank - 1, file - 1))
    }
    let right_capture = board[rank - 1][file + 1];
    if right_capture.is_enemy_of(Black) {
        moves.push(ChessCell(rank - 1, file + 1))
    }
    let one_forward = board[rank - 1][file];
    if one_forward.is_empty() {
        moves.push(ChessCell(rank - 1, file));
        let two_forward = board[rank - 2][file];
        if rank == RANK_7 && board[rank - 2][file].is_empty() {
            moves.push(ChessCell(rank - 2, file));
        }
    }
}
fn knight_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessCell>,
) {
    let board = board_state.board;
    let ChessCell(rank, file) = position;
    let jumps = [
        (rank - 2, file + 1),
        (rank - 2, file + 1),
        (rank - 1, file + 2),
        (rank - 1, file + 2),
        (rank + 1, file + 2),
        (rank + 1, file + 2),
        (rank + 2, file + 1),
        (rank + 2, file + 1),
    ];
    for jump in jumps {
        if board[jump.0][jump.1].is_empty() {}
    }
}
