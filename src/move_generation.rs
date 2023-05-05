use crate::board;
use crate::board::BoardState;
use crate::board::ChessCell;
use crate::board::Piece;
use crate::board::PieceColor;
use crate::board::PieceColor::*;
use crate::board::PieceKind::*;
use crate::board::Square;
use crate::constants::*;
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const KING_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
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
    let color = piece.color();
    match piece.kind() {
        Pawn => match color {
            White => white_pawn_moves(board_state, position, &mut moves),
            Black => black_pawn_moves(board_state, position, &mut moves),
        },
        Knight => knight_moves(color, board_state, position, &mut moves),
        Bishop => bishop_moves(color, board_state, position, &mut moves),
        Rook => rook_moves(color, board_state, position, &mut moves),
        King => king_moves(color, board_state, position, &mut moves),
        Queen => queen_moves(color, board_state, position, &mut moves),
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
        if board[jump.0][jump.1].is_empty_or_enemy_of(color) {
            moves.push(jump.into())
        }
    }
}
fn bishop_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessCell>,
) {
    let board = board_state.board;
    for direction in &BISHOP_DIRECTIONS {
        let mut distance = 1;
        loop {
            let dest_rank = (position.0 as i32 + direction.0 * distance) as usize;
            let dest_file = (position.1 as i32 + direction.1 * distance) as usize;
            let dest_square = board[dest_rank][dest_file];
            match dest_square {
                Square::Aether => break,
                Square::Empty => {
                    moves.push(ChessCell(dest_rank, dest_file));
                    distance += 1
                }
                Square::Full(_) => {
                    if dest_square.is_enemy_of(color) {
                        moves.push(ChessCell(dest_rank, dest_file));
                        break;
                    }
                }
            }
        }
    }
}
fn rook_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessCell>,
) {
    let board = board_state.board;
    for direction in &ROOK_DIRECTIONS {
        let mut distance = 1;
        loop {
            let dest_rank = (position.0 as i32 + direction.0 * distance) as usize;
            let dest_file = (position.1 as i32 + direction.1 * distance) as usize;
            let dest_square = board[dest_rank][dest_file];
            match dest_square {
                Square::Aether => break,
                Square::Empty => {
                    moves.push(ChessCell(dest_rank, dest_file));
                    distance += 1
                }
                Square::Full(_) => {
                    if dest_square.is_enemy_of(color) {
                        moves.push(ChessCell(dest_rank, dest_file));
                        break;
                    }
                }
            }
        }
    }
}
fn queen_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessCell>,
) {
    bishop_moves(color, board_state, position, moves);
    rook_moves(color, board_state, position, moves);
}
fn king_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessCell>,
) {
    let board = board_state.board;
    for king_move in KING_DIRECTIONS {
        let dest_rank = (position.0 as i32 + king_move.0) as usize;
        let dest_file = (position.1 as i32 + king_move.0) as usize;
        let dest_square = board[dest_rank][dest_file];
        if dest_square.is_empty_or_enemy_of(color) {
            moves.push(ChessCell(dest_rank, dest_file));
        }
    }
}
