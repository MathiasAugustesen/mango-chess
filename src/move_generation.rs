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
use crate::ChessMove;
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub fn generate_moves(board_state: &mut BoardState) -> Vec<ChessMove> {
    let mut valid_moves: Vec<ChessMove> = Vec::with_capacity(16);
    let potential_moves = generate_pseudo_moves_for_player(board_state);
    for mov in potential_moves {
        board_state.make_move(mov);
        if board_state.is_valid_move() {
            valid_moves.push(mov);
        }
        board_state.unmake_move();
    }
    valid_moves
}
pub fn generate_pseudo_moves_for_player(board_state: &BoardState) -> Vec<ChessMove> {
    let piece_positions = board_state.get_piece_positions(board_state.to_move);
    let mut potential_moves: Vec<ChessMove> = Vec::with_capacity(16);

    for position in piece_positions {
        let piece = board_state.board.square(position).piece().unwrap();
        generate_pseudo_moves_for_piece(piece, board_state, position, &mut potential_moves);
    }
    potential_moves
}
pub fn generate_pseudo_moves_for_piece(
    piece: Piece,
    board_state: &BoardState,
    position: ChessCell,
    pseudo_moves: &mut Vec<ChessMove>,
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
fn white_pawn_moves(board_state: &BoardState, position: ChessCell, moves: &mut Vec<ChessMove>) {
    let ChessCell(rank, file) = position;
    let left_cap = ChessCell(rank + 1, file - 1);
    let target = board_state.board.square(left_cap);
    if target.is_enemy_of(White) {
        moves.push((position, left_cap).into())
    }

    let right_cap = ChessCell(rank + 1, file + 1);
    let target = board_state.board.square(right_cap);
    if target.is_enemy_of(White) {
        moves.push((position, right_cap).into())
    }

    let one_forward = ChessCell(rank + 1, file);
    let target = board_state.board.square(one_forward);
    if target.is_empty() {
        moves.push((position, one_forward).into());

        if rank == RANK_2 {
            let two_forward = ChessCell(rank + 2, file);
            let target = board_state.board.square(two_forward);
            if target.is_empty() {
                moves.push((position, two_forward).into());
            }
        }
    }
}

fn black_pawn_moves(board_state: &BoardState, position: ChessCell, moves: &mut Vec<ChessMove>) {
    let ChessCell(rank, file) = position;
    let left_cap = ChessCell(rank - 1, file - 1);
    let target = board_state.board.square(left_cap);
    if target.is_enemy_of(Black) {
        moves.push((position, left_cap).into())
    }

    let right_cap = ChessCell(rank - 1, file + 1);
    let target = board_state.board.square(right_cap);
    if target.is_enemy_of(Black) {
        moves.push((position, right_cap).into())
    }

    let one_forward = ChessCell(rank - 1, file);
    let target = board_state.board.square(one_forward);
    if target.is_empty() {
        moves.push((position, one_forward).into());

        if rank == RANK_7 {
            let two_forward = ChessCell(rank - 2, file);
            let target = board_state.board.square(two_forward);
            if target.is_empty() {
                moves.push((position, two_forward).into());
            }
        }
    }
}
pub fn knight_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessMove>,
) {
    let destinations = KNIGHT_RAY_ATTACKS[position.as_index()];
    for destination in destinations {
        let target = ChessCell::from_index(*destination);
        let attacked_square = board_state.board.square(target);
        if attacked_square.is_empty_or_enemy_of(color) {
            moves.push((position, target).into());
        }
    }
}
pub fn bishop_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessMove>,
) {
    for direction in &BISHOP_DIRECTIONS {
        let mut distance = 1;
        loop {
            let dest_rank = (position.0 as i32 + direction.0 * distance) as usize;
            let dest_file = (position.1 as i32 + direction.1 * distance) as usize;
            let dest = ChessCell(dest_rank, dest_file);
            let target = board_state.board.square(dest);
            match target {
                Square::Aether => break,
                Square::Empty => {
                    moves.push((position, dest).into());
                    distance += 1;
                }
                Square::Full(_) => {
                    if target.is_enemy_of(color) {
                        moves.push((position, dest).into());
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
    moves: &mut Vec<ChessMove>,
) {
    for direction in &ROOK_DIRECTIONS {
        let mut distance = 1;
        loop {
            let dest_rank = (position.0 as i32 + direction.0 * distance) as usize;
            let dest_file = (position.1 as i32 + direction.1 * distance) as usize;
            let dest = ChessCell(dest_rank, dest_file);
            let target = board_state.board.square(dest);
            match target {
                Square::Aether => break,
                Square::Empty => {
                    moves.push((position, dest).into());
                    distance += 1
                }
                Square::Full(_) => {
                    if target.is_enemy_of(color) {
                        moves.push((position, dest).into());
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
    moves: &mut Vec<ChessMove>,
) {
    bishop_moves(color, board_state, position, moves);
    rook_moves(color, board_state, position, moves);
}
pub fn king_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessMove>,
) {
    let destinations = KING_RAY_ATTACKS[position.as_index()];
    for destination in destinations {
        let target = ChessCell::from_index(*destination);
        if board_state.board.square(target).is_empty_or_enemy_of(color) {
            moves.push((position, target).into());
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::board::BoardState;

    use super::generate_moves;

    #[test]
    fn generate_moves_from_starting_position() {
        let mut board_state = BoardState::new_game();
        let legal_moves = generate_moves(&mut board_state);
        assert_eq!(legal_moves.len(), 20);
    }
    #[test]
    fn generate_moves_from_bongcloud() {
        let mut board_state =
            BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2")
                .unwrap();
        let legal_moves = generate_moves(&mut board_state);
        assert_eq!(legal_moves.len(), 29);
    }
}
