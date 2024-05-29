use crate::board_elements::ChessCell;
use crate::board_elements::ChessMove;
use crate::board_elements::Piece;
use crate::board_elements::PieceColor;
use crate::board_elements::PieceColor::*;
use crate::board_elements::PieceKind::*;
use crate::board_elements::Square;
use crate::board_state::BoardState;
use crate::constants::*;
use crate::ray_attacks::KING_RAY_ATTACKS;
use crate::ray_attacks::KNIGHT_RAY_ATTACKS;
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub fn generate_moves(board_state: &BoardState) -> Vec<ChessMove> {
    let mut valid_moves: Vec<ChessMove> = Vec::with_capacity(16);
    let potential_moves = generate_pseudo_moves_for_player(board_state);
    for mov in potential_moves {
        let mut copy_board = board_state.clone();
        copy_board.make_move(mov);
        if copy_board.is_valid_move() {
            valid_moves.push(mov);
        }
    }
    valid_moves
}
pub fn generate_castling_moves(board_state: &BoardState) -> Vec<ChessMove> {
    let mut castling_moves = Vec::new();
    let ChessCell(king_rank, king_file) = board_state.king_location_of(board_state.to_move);
    for castling_type in board_state.available_castling_types(board_state.to_move) {
        let direction = castling_type.direction();
        let step_one_cell = ChessCell(king_rank, (king_file as i8 + direction) as usize);
        let step_two_cell = ChessCell(king_rank, (king_file as i8 + direction * 2) as usize);
        let step_one = board_state.board.square(step_one_cell);
        let step_two = board_state.board.square(step_two_cell);

        if step_one.is_empty()
            && step_two.is_empty()
            && !board_state.square_is_attacked(
                board_state.king_location_of(board_state.to_move),
                board_state.to_move.opposite(),
            )
            && !board_state.square_is_attacked(step_one_cell, board_state.to_move.opposite())
            && !board_state.square_is_attacked(step_two_cell, board_state.to_move.opposite())
        {
            castling_moves.push(ChessMove::from(castling_type));
        }
    }
    castling_moves
}
pub fn generate_pseudo_moves_for_player(board_state: &BoardState) -> Vec<ChessMove> {
    let piece_positions = board_state.get_piece_positions(board_state.to_move);
    let mut potential_moves: Vec<ChessMove> = Vec::with_capacity(16);

    for position in piece_positions {
        let piece = board_state.board.square(position).piece().unwrap();
        generate_pseudo_moves_for_piece(piece, board_state, position, &mut potential_moves);
    }
    potential_moves.extend(generate_castling_moves(board_state));
    potential_moves.extend(generate_en_passant_moves(board_state));
    potential_moves
}
pub fn generate_pseudo_moves_for_piece(
    piece: Piece,
    board_state: &BoardState,
    position: ChessCell,
    pseudo_moves: &mut Vec<ChessMove>,
) {
    let color = piece.color;
    match piece.kind {
        Pawn => pawn_moves(color, board_state, position, pseudo_moves),
        Knight => knight_moves(color, board_state, position, pseudo_moves),
        Bishop => bishop_moves(color, board_state, position, pseudo_moves),
        Rook => rook_moves(color, board_state, position, pseudo_moves),
        King => king_moves(color, board_state, position, pseudo_moves),
        Queen => queen_moves(color, board_state, position, pseudo_moves),
    }
}

fn pawn_moves(
    color: PieceColor,
    board_state: &BoardState,
    position: ChessCell,
    moves: &mut Vec<ChessMove>,
) {
    let mut temp_moves: Vec<ChessMove> = vec![];
    let push_direction = color.signum();
    let ChessCell(rank, file) = position;

    let left_cap = ChessCell((rank as i32 + push_direction) as usize, file - 1);
    let target = board_state.board.square(left_cap);
    if target.is_enemy_of(color) {
        temp_moves.push((position, left_cap).into())
    }

    let right_cap = ChessCell((rank as i32 + push_direction) as usize, file + 1);
    let target = board_state.board.square(right_cap);
    if target.is_enemy_of(color) {
        temp_moves.push((position, right_cap).into())
    }

    let one_forward = ChessCell((rank as i32 + push_direction) as usize, file);
    let target = board_state.board.square(one_forward);
    if target.is_empty() {
        temp_moves.push((position, one_forward).into());
        if rank == color.pawn_starting_rank() {
            let two_forward = ChessCell((rank as i32 + push_direction) as usize, file);
            let target = board_state.board.square(two_forward);
            if target.is_empty() {
                temp_moves.push((position, two_forward).into());
            }
        }
    }

    if rank == color.promotion_rank() {
        let promotion_pieces = [
            Piece::knight(color),
            Piece::bishop(color),
            Piece::rook(color),
            Piece::queen(color),
        ];
        temp_moves = temp_moves
            .into_iter()
            .flat_map(|mov| {
                promotion_pieces
                    .iter()
                    .map(move |promotion_piece| ChessMove {
                        start: mov.start,
                        dest: mov.dest,
                        promotion: Some(*promotion_piece),
                    })
            })
            .collect()
    }

    moves.append(&mut temp_moves);
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
pub fn generate_en_passant_moves(board_state: &BoardState) -> Vec<ChessMove> {
    let mut en_passant_moves = Vec::new();
    if let Some(en_passant_square) = board_state.en_passant {
        let target_rank = board_state.to_move.opposite().en_passant_rank();
        let threatened_square = ChessCell(target_rank, en_passant_square.1);
        if board_state
            .board
            .square(threatened_square)
            .is_color(board_state.to_move)
        {
            return Vec::new();
        }

        let attacking_rank = board_state.to_move.en_passant_attacking_rank();
        let left_threat = ChessCell(attacking_rank, en_passant_square.1 - 1);
        let right_threat = ChessCell(attacking_rank, en_passant_square.1 + 1);

        for threat in [left_threat, right_threat] {
            let threatening_square = board_state.board.square(threat);
            if threatening_square.is_aether() {
                continue;
            }
            if let Some(threatening_piece) = threatening_square.piece() {
                if threatening_piece.kind == Pawn && threatening_piece.color == board_state.to_move
                {
                    en_passant_moves.push((threat, threatened_square).into());
                }
            }
        }
    }
    en_passant_moves
}
#[cfg(test)]
mod tests {
    use crate::{
        board_elements::{ChessMove, Piece, PieceColor, PieceKind},
        board_state::BoardState,
    };

    use super::generate_moves;
    use crate::constants::*;

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

    #[test]
    fn en_passant_with_one_pawn_can_capture() {
        let mut board_state = BoardState::new_game();
        board_state.make_move((E2, E4).into());
        board_state.make_move((D7, D5).into());
        board_state.make_move((E4, E5).into());
        board_state.make_move((F7, F5).into());

        let legal_moves = generate_moves(&board_state);

        assert!(legal_moves.contains(&(E5, F6).into()));

        board_state.make_move((E5, F6).into());

        assert_eq!(
            board_state.board.square(F6).piece().unwrap(),
            Piece {
                color: PieceColor::White,
                kind: PieceKind::Pawn
            }
        );
        assert!(!board_state.board.square(F5).has_piece());
    }

    #[test]
    fn en_passant_with_two_pawns_can_both_capture() {
        let mut board_state =
            BoardState::from_fen("rnbqkbnr/pp2pppp/8/2ppP1P1/8/8/PPPP1P1P/RNBQKBNR b KQkq - 0 4")
                .unwrap();

        board_state.make_move((F7, F5).into());

        let legal_moves = generate_moves(&board_state);
        assert!(legal_moves.contains(&(E5, F6).into()));
        assert!(legal_moves.contains(&(G5, F6).into()));
    }

    #[test]
    fn game_with_possible_promotion_for_white_contains_correct_moves() {
        let board_state = BoardState::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1").unwrap();

        let legal_moves = generate_moves(&board_state);

        let expected_moves: Vec<ChessMove> = vec![
            (A7, A8, Piece::knight(PieceColor::White)).into(),
            (A7, A8, Piece::bishop(PieceColor::White)).into(),
            (A7, A8, Piece::rook(PieceColor::White)).into(),
            (A7, A8, Piece::queen(PieceColor::White)).into(),
        ];

        for mov in expected_moves {
            assert!(legal_moves.contains(&mov))
        }
    }

    #[test]
    fn game_with_possible_promotion_for_black_contains_correct_moves() {
        let board_state = BoardState::from_fen("7k/8/8/8/8/8/p7/7K b - - 0 1").unwrap();

        let legal_moves = generate_moves(&board_state);

        let expected_moves: Vec<ChessMove> = vec![
            (A2, A1, Piece::knight(PieceColor::Black)).into(),
            (A2, A1, Piece::bishop(PieceColor::Black)).into(),
            (A2, A1, Piece::rook(PieceColor::Black)).into(),
            (A2, A1, Piece::queen(PieceColor::Black)).into(),
        ];

        for mov in expected_moves {
            assert!(legal_moves.contains(&mov))
        }
    }

    #[test]
    fn game_with_possible_promotion_by_push_and_capture_for_white_contains_correct_moves() {
        let board_state =
            BoardState::from_fen("r4rk1/1pp1P2p/p3bqpQ/4p3/2B5/8/PPP3PP/2KR3R w - - 0 1").unwrap();

        let legal_moves = generate_moves(&board_state);

        let expected_moves: Vec<ChessMove> = vec![
            (E7, E8, Piece::knight(PieceColor::White)).into(),
            (E7, E8, Piece::bishop(PieceColor::White)).into(),
            (E7, E8, Piece::rook(PieceColor::White)).into(),
            (E7, E8, Piece::queen(PieceColor::White)).into(),
            (E7, F8, Piece::knight(PieceColor::White)).into(),
            (E7, F8, Piece::bishop(PieceColor::White)).into(),
            (E7, F8, Piece::rook(PieceColor::White)).into(),
            (E7, F8, Piece::queen(PieceColor::White)).into(),
        ];

        for mov in expected_moves {
            assert!(legal_moves.contains(&mov))
        }
    }

    #[test]
    fn game_with_possible_promotion_by_push_and_capture_for_black_contains_correct_moves() {
        let board_state = BoardState::from_fen("8/8/1k6/8/8/8/3K2p1/5R2 b - - 0 1").unwrap();

        let legal_moves = generate_moves(&board_state);

        let expected_moves: Vec<ChessMove> = vec![
            (G2, G1, Piece::knight(PieceColor::Black)).into(),
            (G2, G1, Piece::bishop(PieceColor::Black)).into(),
            (G2, G1, Piece::rook(PieceColor::Black)).into(),
            (G2, G1, Piece::queen(PieceColor::Black)).into(),
            (G2, F1, Piece::knight(PieceColor::Black)).into(),
            (G2, F1, Piece::bishop(PieceColor::Black)).into(),
            (G2, F1, Piece::rook(PieceColor::Black)).into(),
            (G2, F1, Piece::queen(PieceColor::Black)).into(),
        ];

        for mov in expected_moves {
            assert!(legal_moves.contains(&mov))
        }
    }
}
