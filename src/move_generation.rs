use crate::board::BoardState;
use crate::board::ChessCell;
use crate::board::Piece;
use crate::board::PieceColor;
use crate::board::PieceColor::*;
use crate::board::PieceKind::*;
use crate::board::Square;
use crate::constants::*;
use crate::ray_attacks::KING_RAY_ATTACKS;
use crate::ray_attacks::KNIGHT_RAY_ATTACKS;
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub fn generate_moves(board_state: &BoardState) -> Vec<BoardState> {
    let board = board_state.board;
    let mut new_moves: Vec<BoardState> = Vec::with_capacity(16);
    let mut potential_moves: Vec<(ChessCell, ChessCell)> = Vec::with_capacity(16);
    let player_to_move = board_state.to_move;
    let piece_positions = board_state.get_piece_positions(player_to_move);

    for position in piece_positions {
        if let Square::Full(piece) = board[position.0][position.1] {
            generate_pseudo_moves_for_piece(piece, board_state, position, &mut potential_moves);
        }
    }
    for (position, destination) in potential_moves {
        let mut new_board = board_state.clone();
        new_board.make_move(position, destination);
        if new_board.is_valid_move() {
            new_moves.push(new_board);
        }
    }
    new_moves
}
pub fn generate_pseudo_moves_for_piece(
    piece: Piece,
    board_state: &BoardState,
    position: ChessCell,
    pseudo_moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    let color = piece.color();
    match piece.kind() {
        Pawn => match color {
            White => white_pawn_moves(board_state, position, pseudo_moves),
            Black => black_pawn_moves(board_state, position, pseudo_moves),
        },
        Knight => knight_moves(color, board_state, position, pseudo_moves),
        Bishop => bishop_moves(color, board_state, position, pseudo_moves),
        Rook => rook_moves(color, board_state, position, pseudo_moves),
        King => king_moves(color, board_state, position, pseudo_moves),
        Queen => queen_moves(color, board_state, position, pseudo_moves),
    }
}
fn white_pawn_moves(
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    let board = board_state.board;
    let ChessCell(rank, file) = position;
    let left_capture = board[rank + 1][file - 1];
    if left_capture.is_enemy_of(White) {
        moves.push((position, ChessCell(rank + 1, file - 1)))
    }
    let right_capture = board[rank + 1][file + 1];
    if right_capture.is_enemy_of(White) {
        moves.push((position, ChessCell(rank + 1, file + 1)))
    }
    let one_forward = board[rank + 1][file];
    if one_forward.is_empty() {
        moves.push((position, ChessCell(rank + 1, file)));
        let two_forward = board[rank + 2][file];
        if rank == RANK_2 && two_forward.is_empty() {
            moves.push((position, ChessCell(rank + 2, file)));
        }
    }
}

fn black_pawn_moves(
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    let board = board_state.board;
    let ChessCell(rank, file) = position;
    let left_capture = board[rank - 1][file - 1];
    if left_capture.is_enemy_of(Black) {
        moves.push((position, ChessCell(rank - 1, file - 1)))
    }
    let right_capture = board[rank - 1][file + 1];
    if right_capture.is_enemy_of(Black) {
        moves.push((position, ChessCell(rank - 1, file + 1)))
    }
    let one_forward = board[rank - 1][file];
    if one_forward.is_empty() {
        moves.push((position, ChessCell(rank - 1, file)));
        let two_forward = board[rank - 2][file];
        if rank == RANK_7 && two_forward.is_empty() {
            moves.push((position, ChessCell(rank - 2, file)));
        }
    }
}
pub fn knight_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    let board = board_state.board;
    let destinations = KNIGHT_RAY_ATTACKS[position.as_index()];
    for destination in destinations {
        let attacked_cell = ChessCell::from_index(*destination);
        let attacked_square = board[attacked_cell.0][attacked_cell.1];
        if attacked_square.is_empty_or_enemy_of(color) {
            moves.push((position, attacked_cell));
        }
    }
}
pub fn bishop_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
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
                    moves.push((position, ChessCell(dest_rank, dest_file)));
                    distance += 1;
                }
                Square::Full(_) => {
                    if dest_square.is_enemy_of(color) {
                        moves.push((position, ChessCell(dest_rank, dest_file)));
                    }
                    break;
                }
            }
        }
    }
}
pub fn rook_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
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
                    moves.push((position, ChessCell(dest_rank, dest_file)));
                    distance += 1
                }
                Square::Full(_) => {
                    if dest_square.is_enemy_of(color) {
                        moves.push((position, ChessCell(dest_rank, dest_file)));
                    }
                    break;
                }
            }
        }
    }
}
pub fn queen_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    bishop_moves(color, board_state, position, moves);
    rook_moves(color, board_state, position, moves);
}
pub fn king_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<(ChessCell, ChessCell)>,
) {
    let board = board_state.board;
    let destinations = KING_RAY_ATTACKS[position.as_index()];
    for destination in destinations {
        let cell = ChessCell::from_index(*destination);
        if board[cell.0][cell.1].is_empty_or_enemy_of(color) {
            moves.push((position, cell));
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::board::BoardState;

    use super::generate_moves;

    #[test]
    fn generate_moves_from_starting_position() {
        let board_state = BoardState::new_game();
        let legal_moves = generate_moves(&board_state);
        assert_eq!(legal_moves.len(), 20);
    }
    #[test]
    fn generate_moves_from_bongcloud() {
        let board_state =
            BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2")
                .unwrap();
        let legal_moves = generate_moves(&board_state);
        assert_eq!(legal_moves.len(), 29);
    }
}
