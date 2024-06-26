use crate::board_elements::BitBoard;
use crate::board_elements::CastlingRights;
use crate::board_elements::CastlingType;
use crate::board_elements::ChessCell;
use crate::board_elements::ChessMove;
use crate::board_elements::Piece;
use crate::board_elements::PieceColor;
use crate::board_elements::PieceColor::*;
use crate::board_elements::PieceKind::*;
use crate::board_elements::Square;
use crate::chess_board::ChessBoard;
use crate::constants::*;
use crate::evaluation::evaluate;
use crate::evaluation::evaluate_piece;
use crate::fen;
use crate::fen::castling_rights_from_fen;
use crate::fen::en_passant_square_from_fen;
use crate::move_generation::generate_moves;
use crate::move_generation::generate_pseudo_moves_for_piece;
use crate::move_scoring::positional_value_delta;
use crate::ray_attacks::*;
use crate::zobrist_hashing::ZobristOracle;
use crate::GameResult;

#[derive(Clone, PartialEq, Debug)]
pub struct BoardState {
    pub board: ChessBoard,
    pub to_move: PieceColor,
    // Keeps track of all the white pieces
    pub white_bitboard: BitBoard,
    // Keeps track of all the black pieces
    pub black_bitboard: BitBoard,
    pub white_king_location: ChessCell,
    pub black_king_location: ChessCell,
    pub last_move: Option<ChessMove>,
    pub en_passant: Option<ChessCell>,
    pub eval: i32,
    pub castling_rights: CastlingRights,
    pub zobrist_key: u64,
}
impl BoardState {
    #[inline]
    pub fn swap_to_move(&mut self) {
        match self.to_move {
            White => self.to_move = Black,
            Black => self.to_move = White,
        }
    }
    pub fn available_castling_types(&self, color: PieceColor) -> Vec<CastlingType> {
        let mut castling_types = Vec::new();
        match color {
            White => {
                if self.castling_rights.white_king_side_castling {
                    castling_types.push(CastlingType::WhiteKingSide)
                }
                if self.castling_rights.white_queen_side_castling {
                    castling_types.push(CastlingType::WhiteQueenSide)
                }
            }
            Black => {
                if self.castling_rights.black_king_side_castling {
                    castling_types.push(CastlingType::BlackKingSide)
                }
                if self.castling_rights.black_queen_side_castling {
                    castling_types.push(CastlingType::BlackQueenSide)
                }
            }
        }
        castling_types
    }
    pub fn increment_eval(&mut self, eval_increment: i32) {
        self.eval += eval_increment * self.to_move.signum()
    }
    pub fn pov_eval(&self) -> i32 {
        self.eval * self.to_move.signum()
    }
    #[inline]
    fn update_king_position(&mut self, dest: ChessCell) {
        match self.to_move {
            White => self.white_king_location = dest,
            Black => self.black_king_location = dest,
        }
    }
    pub fn kill_en_passant_piece(&mut self, square: ChessCell, eval_increment: &mut i32) {
        let trespasser = self.board.square(square).piece().unwrap();
        *eval_increment += evaluate_piece(trespasser, square.as_index());
        *self.board.square_mut(square) = Square::Empty;
        self.remove_from_bitboard(square);
    }
    // returns captured piece, if any
    pub fn move_piece(&mut self, mov: ChessMove, eval_increment: &mut i32) -> Option<Piece> {
        let moving_piece = self.board.square(mov.start).piece().unwrap();
        let captured_piece = self.board.square(mov.dest).piece();
        if let Some(captured_piece) = captured_piece {
            *eval_increment += evaluate_piece(captured_piece, mov.dest.as_index());
        }

        *self.board.square_mut(mov.start) = Square::Empty;

        if let Some(promoted_piece) = mov.promotion {
            *self.board.square_mut(mov.dest) = Square::Full(promoted_piece);
            *eval_increment += evaluate_piece(promoted_piece, mov.dest.as_index())
                - evaluate_piece(moving_piece, mov.start.as_index());
        } else {
            *self.board.square_mut(mov.dest) = Square::Full(moving_piece);
            *eval_increment += positional_value_delta(moving_piece, mov);
        }

        self.update_bitboards(mov);

        captured_piece
    }
    pub fn make_move(&mut self, mov: ChessMove) {
        // Used for zobrist key incremental updates
        let en_passant_file_before = self.en_passant.map(|cell| cell.1);
        let castling_rights_before = self.castling_rights;

        let start = mov.start;
        let dest = mov.dest;
        self.last_move = Some(mov);
        let mut eval_increment = 0;

        let moving_piece = self.board.square(start).piece().unwrap();

        if moving_piece.kind == King {
            self.castling_rights
                .remove_color_castling_rights(self.to_move);
            self.update_king_position(dest);
            // If king moves two squares, must be castle
            let move_is_castle = start.1.abs_diff(dest.1) == 2;
            if move_is_castle {
                self.resolve_castling(mov, &mut eval_increment);
            }
        }
        // If a rook moves
        if moving_piece.kind == Rook && matches!(start, A1 | A8 | H1 | H8) {
            self.castling_rights.remove_castling_right(start.into())
        }
        // If a rook is potentially captured on its starting square
        if matches!(dest, A1 | A8 | H1 | H8) {
            self.castling_rights.remove_castling_right(dest.into())
        }
        // Check if it was an en passant capture
        if Some(dest) == self.en_passant
            && moving_piece.kind == Pawn
            && start.1.abs_diff(dest.1) == 1
        {
            let en_passant_capture = ChessCell(start.0, dest.1);
            self.kill_en_passant_piece(en_passant_capture, &mut eval_increment);
        }

        // Reset en passant
        self.en_passant = None;

        // Initialize possible en passant
        if moving_piece.kind == Pawn && start.0.abs_diff(dest.0) == 2 {
            let ep_rank = self.to_move.en_passant_rank();
            let en_passant_square = ChessCell(ep_rank, dest.1);
            self.en_passant = Some(en_passant_square);
        }

        let captured_piece = self.move_piece(mov, &mut eval_increment);

        self.increment_eval(eval_increment);

        self.swap_to_move();

        let en_passant_file_after = self.en_passant.map(|cell| cell.1);
        let removed_castling_rights = self.castling_rights.diff_from(castling_rights_before);

        self.set_zobrist_key_from_move(
            mov,
            moving_piece,
            captured_piece,
            en_passant_file_before,
            en_passant_file_after,
            removed_castling_rights,
        );
    }
    fn resolve_castling(&mut self, mov: ChessMove, eval_increment: &mut i32) {
        let (rook_start, rook_dest) = match mov.dest {
            G1 => (H1, F1),
            C1 => (A1, D1),
            G8 => (H8, F8),
            C8 => (A8, D8),
            _ => unreachable!(),
        };
        let rook_move = ChessMove {
            start: rook_start,
            dest: rook_dest,
            promotion: None,
        };
        self.move_piece(rook_move, eval_increment);
    }
    fn remove_from_bitboard(&mut self, square: ChessCell) {
        let opposing_player_bitboard = match self.to_move {
            White => &mut self.black_bitboard,
            Black => &mut self.white_bitboard,
        };
        opposing_player_bitboard.remove_piece(square.as_index());
    }
    fn update_bitboards(&mut self, mov: ChessMove) {
        let (current_player_bitboard, opposing_player_bitboard) = match self.to_move {
            White => (&mut self.white_bitboard, &mut self.black_bitboard),
            Black => (&mut self.black_bitboard, &mut self.white_bitboard),
        };

        current_player_bitboard.remove_piece(mov.start.as_index());
        current_player_bitboard.add_piece(mov.dest.as_index());

        opposing_player_bitboard.remove_piece(mov.dest.as_index());
    }
    pub fn is_terminal(&self) -> bool {
        let moves = generate_moves(self);

        moves.is_empty()
    }

    pub fn terminal_eval(&self) -> i32 {
        match self.square_is_attacked(self.king_location_of(self.to_move), self.to_move.opposite())
        {
            true => i32::MIN / 2,
            false => 0,
        }
    }
    pub fn get_game_winner(&self) -> GameResult {
        let potentially_mated_king = self.king_location_of(self.to_move);
        match self.square_is_attacked(potentially_mated_king, self.to_move.opposite()) {
            true => GameResult::Winner(self.to_move.opposite()),
            false => GameResult::Draw,
        }
    }
    pub fn get_piece_positions(&self, color: PieceColor) -> Vec<ChessCell> {
        let mut piece_positions = Vec::new();
        let mut bitboard = match color {
            White => self.white_bitboard,
            Black => self.black_bitboard,
        };
        while bitboard.0 != 0 {
            let position = bitboard.0.trailing_zeros();
            let cell = ChessCell::from_index(position as usize);
            piece_positions.push(cell);
            bitboard.0 ^= 1 << position;
        }
        piece_positions
    }
    pub fn king_location_of(&self, color: PieceColor) -> ChessCell {
        match color {
            White => self.white_king_location,
            Black => self.black_king_location,
        }
    }
    fn set_zobrist_key_from_scratch(&mut self) {
        let mut zobrist_key: u64 = 0;

        let all_piece_positions = self
            .get_piece_positions(White)
            .into_iter()
            .chain(self.get_piece_positions(Black));

        for piece_position in all_piece_positions {
            let piece = self.board.square(piece_position).piece().unwrap();
            zobrist_key ^= ZobristOracle::piece_bitstring(piece, piece_position.as_index());
        }

        let all_castling_rights = self
            .available_castling_types(White)
            .into_iter()
            .chain(self.available_castling_types(Black));

        for castling_right in all_castling_rights {
            zobrist_key ^= ZobristOracle::castling_right_bitstring(castling_right);
        }

        if let Some(en_passant) = self.en_passant {
            let file = en_passant.1;
            zobrist_key ^= ZobristOracle::en_passant_bitstring(file);
        }

        if self.to_move == Black {
            zobrist_key ^= ZobristOracle::black_to_move_bitstring();
        }

        self.zobrist_key = zobrist_key;
    }

    fn set_zobrist_key_from_move(
        &mut self,
        mov: ChessMove,
        moving_piece: Piece,
        captured_piece: Option<Piece>,
        previous_en_passant_file: Option<usize>,
        new_en_passant_file: Option<usize>,
        removed_castling_rights: Vec<CastlingType>,
    ) {
        let mut incremented_zobrist_key = 0;

        incremented_zobrist_key ^= ZobristOracle::black_to_move_bitstring();

        incremented_zobrist_key ^=
            ZobristOracle::piece_bitstring(moving_piece, mov.start.as_index());
        incremented_zobrist_key ^=
            ZobristOracle::piece_bitstring(moving_piece, mov.dest.as_index());

        if let Some(captured_piece) = captured_piece {
            incremented_zobrist_key ^=
                ZobristOracle::piece_bitstring(captured_piece, mov.dest.as_index());
        }

        if let Some(old_file) = previous_en_passant_file {
            incremented_zobrist_key ^= ZobristOracle::en_passant_bitstring(old_file);
        }

        if let Some(new_file) = new_en_passant_file {
            incremented_zobrist_key ^= ZobristOracle::en_passant_bitstring(new_file);
        }

        for castling_right in removed_castling_rights {
            incremented_zobrist_key ^= ZobristOracle::castling_right_bitstring(castling_right);
        }

        self.zobrist_key ^= incremented_zobrist_key;
    }
    // First, a list of all enemy pieces that x-ray the target is generated with a lookup table.
    // If this list is empty, the square is safe.
    // Returns true if any of the enemy pieces attack the target.
    pub fn square_is_attacked(&self, target_square: ChessCell, attacker: PieceColor) -> bool {
        let ray_attackers = self.ray_attackers(target_square, attacker);
        if ray_attackers.is_empty() {
            return false;
        }
        let mut enemy_moves: Vec<ChessMove> = Vec::new();
        for (piece, position) in ray_attackers {
            // TODO: Fix looking at already checked moves
            generate_pseudo_moves_for_piece(piece, self, position, &mut enemy_moves);
            let square_is_attacked = enemy_moves
                .iter()
                .map(|mov| mov.dest)
                .any(|attacked_square| attacked_square == target_square);
            if square_is_attacked {
                return true;
            }
            enemy_moves.clear()
        }
        false
    }
    // Given an arbitrary position, determine if the position is legal given that the player next to move is self.to_move.
    // This method does not make any assumptions about how the move was made.
    pub fn is_valid_move(&self) -> bool {
        let king_location = match self.to_move.opposite() {
            White => self.white_king_location,
            Black => self.black_king_location,
        };
        !self.square_is_attacked(king_location, self.to_move)
    }
    // Searches a lookup table and returns all pieces that attack the target assuming an empty board.
    pub fn ray_attackers(
        &self,
        target_square: ChessCell,
        color: PieceColor,
    ) -> Vec<(Piece, ChessCell)> {
        let target_idx = target_square.as_index();
        let mut ray_attackers: Vec<(Piece, ChessCell)> = Vec::new();
        for attacker in self.get_piece_positions(color) {
            let attacker_idx = attacker.as_index();
            let attacking_square = self.board.square(attacker);
            let attacking_piece = attacking_square.piece().unwrap();
            let attacked_squares = match (attacking_piece.color, attacking_piece.kind) {
                (White, Pawn) => WHITE_PAWN_RAY_ATTACKS[attacker_idx],
                (Black, Pawn) => BLACK_PAWN_RAY_ATTACKS[attacker_idx],
                (_, Knight) => KNIGHT_RAY_ATTACKS[attacker_idx],
                (_, Bishop) => BISHOP_RAY_ATTACKS[attacker_idx],
                (_, Rook) => ROOK_RAY_ATTACKS[attacker_idx],
                (_, Queen) => QUEEN_RAY_ATTACKS[attacker_idx],
                (_, King) => KING_RAY_ATTACKS[attacker_idx],
            };
            if attacked_squares.contains(&target_idx) {
                ray_attackers.push((attacking_piece, attacker));
            }
        }
        ray_attackers
    }
    pub fn empty_game() -> BoardState {
        let board = ChessBoard::empty();
        let to_move = White;
        let white_bitboard = BitBoard(0);
        let black_bitboard = BitBoard(0);
        let white_king_location = ChessCell(100, 100);
        let black_king_location = ChessCell(100, 100);
        let castling_rights = CastlingRights::all_castling_rights();
        BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            en_passant: None,
            eval: 0,
            castling_rights,
            zobrist_key: 0,
        }
    }
    pub fn new_game() -> BoardState {
        let board = ChessBoard::new();
        let to_move = White;
        let white_bitboard = WHITE_STARTING_BITBOARD;
        let black_bitboard = BLACK_STARTING_BITBOARD;
        let white_king_location = E1;
        let black_king_location = E8;
        let castling_rights = CastlingRights::all_castling_rights();
        let _en_passant_square: Option<()> = None;
        let _pawn_promotion: Option<()> = None;
        let mut board_state = BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            en_passant: None,
            eval: 0,
            castling_rights,
            zobrist_key: 0,
        };
        board_state.eval = evaluate(&board_state);
        board_state.set_zobrist_key_from_scratch();
        board_state
    }
    pub fn from_fen(fen: &str) -> Result<BoardState, &str> {
        let fen_parts: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fen_parts.len() != 6 {
            return Err("Failed to parse FEN string: FEN string did not have length 6");
        }
        let fen_board = fen_parts[0];
        let board = fen::board_from_fen(fen_board)?;
        let (white_king_location, black_king_location) = find_kings(&board)?;
        let fen_to_move = fen_parts[1];
        let to_move = fen::to_move_from_fen(fen_to_move)?;
        let fen_castling_rights = fen_parts[2];
        let castling_rights = castling_rights_from_fen(fen_castling_rights)?;
        let fen_en_passant_square = fen_parts[3];
        let _en_passant_square = en_passant_square_from_fen(fen_en_passant_square)?;
        let _halfmove_clock = fen_parts[4];
        let _fullmove_clock = fen_parts[5];
        if white_king_location.0 > BOARD_END
            || white_king_location.1 > BOARD_END
            || black_king_location.0 > BOARD_END
            || black_king_location.1 > BOARD_END
        {
            return Err("Failed to parse FEN string: Both kings were not on the board");
        }
        let (white_bitboard, black_bitboard) = get_bitboards(&board);
        let mut board_state = BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            en_passant: _en_passant_square,
            eval: 0,
            castling_rights,
            zobrist_key: 0,
        };
        board_state.eval = evaluate(&board_state) * board_state.to_move.signum();
        board_state.set_zobrist_key_from_scratch();
        Ok(board_state)
    }
}

// Returns (white_bitboard, black_bitboard)
pub fn get_bitboards(board: &ChessBoard) -> (BitBoard, BitBoard) {
    let mut white_bitboard: u64 = 0;
    let mut black_bitboard: u64 = 0;
    let mut position: u64 = 0;
    for rank in BOARD_START..=BOARD_END {
        for file in BOARD_START..=BOARD_END {
            if let Square::Full(piece) = board.square(ChessCell(rank, file)) {
                match piece.color {
                    White => white_bitboard |= 1 << position,
                    Black => black_bitboard |= 1 << position,
                }
            }
            position += 1;
        }
    }
    (BitBoard(white_bitboard), BitBoard(black_bitboard))
}
// Returns (white_king_location, black_king_location)
fn find_kings(board: &ChessBoard) -> Result<(ChessCell, ChessCell), &'static str> {
    let mut white_king_location: Option<ChessCell> = None;
    let mut black_king_location: Option<ChessCell> = None;
    for rank in RANK_1..=RANK_8 {
        for file in A_FILE..=H_FILE {
            let square = board.square(ChessCell(rank, file));
            if let Square::Full(piece) = square {
                if piece.kind == King {
                    match piece.color {
                        White => white_king_location = Some(ChessCell(rank, file)),
                        Black => black_king_location = Some(ChessCell(rank, file)),
                    }
                }
            }
        }
    }
    if white_king_location.is_none() || black_king_location.is_none() {
        return Err("Failed to parse FEN string: Could not find both kings");
    }
    Ok((white_king_location.unwrap(), black_king_location.unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn parse_chess_cell_from_valid_str_succeeds() {
        let parsed_cell: ChessCell = ChessCell::from_str("b4").unwrap();
        let cell = ChessCell(3 + BOARD_START, 1 + BOARD_START);
        assert_eq!(parsed_cell, cell);
    }
    #[test]
    fn create_board_from_valid_fen_succeeds() {
        let fen_board_state: BoardState =
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        let new_board_state: BoardState = BoardState::new_game();
        assert_eq!(fen_board_state.board, new_board_state.board);
        assert_eq!(fen_board_state.to_move, new_board_state.to_move);
        assert_eq!(
            fen_board_state.white_bitboard,
            new_board_state.white_bitboard
        );
        assert_eq!(
            fen_board_state.black_bitboard,
            new_board_state.black_bitboard
        );
        assert_eq!(
            fen_board_state.white_king_location,
            new_board_state.white_king_location
        );
        assert_eq!(
            fen_board_state.black_king_location,
            new_board_state.black_king_location
        );
        /*
        assert_eq!(
            fen_board_state.castling_rights,
            new_board_state.castling_rights
        );

        assert_eq!(
            fen_board_state.en_passant_square,
            new_board_state.en_passant_square
        );
        assert_eq!(
            fen_board_state.pawn_promotion,
            new_board_state.pawn_promotion
        );
        */
    }
    #[test]
    #[should_panic]
    fn create_board_from_fen_with_invalid_length_panics() {
        BoardState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR KQkq e3 0 1").unwrap();
    }
    #[test]
    fn get_bitboards_returns_the_right_bitboards() {
        let board_state = BoardState::new_game();
        let (white_bitboard, black_bitboard) = get_bitboards(&board_state.board);
        assert_eq!(white_bitboard.0, 0xFFFF);
        assert_eq!(black_bitboard.0, 0xFFFF << 48);
    }
    #[test]
    fn get_piece_positions_for_starting_position() {
        let board_state = BoardState::new_game();
        let white_positions = board_state.get_piece_positions(White);
        let black_positions = board_state.get_piece_positions(Black);
        for rank in RANK_1..=RANK_2 {
            for file in A_FILE..=H_FILE {
                assert!(white_positions.contains(&ChessCell(rank, file)));
            }
        }
        for rank in RANK_7..=RANK_8 {
            for file in A_FILE..=H_FILE {
                assert!(black_positions.contains(&ChessCell(rank, file)));
            }
        }
    }
    #[test]
    fn get_piece_positions_for_bongcloud_position() {
        let board_state =
            BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 1 2")
                .unwrap();
        let white_positions = board_state.get_piece_positions(White);
        let white_bongcloud_positions = [
            A1, B1, C1, D1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, E4,
        ];
        for position in white_positions {
            assert!(white_bongcloud_positions.contains(&position))
        }
        let black_positions = board_state.get_piece_positions(Black);
        let black_bongcloud_positions = [
            A8, B8, C8, D8, E8, F8, G8, H8, A7, B7, C7, D7, F7, G7, H7, E5,
        ];
        for position in black_positions {
            assert!(black_bongcloud_positions.contains(&position))
        }
    }
    #[test]
    fn is_empty_or_enemy_of_tests() {
        let board_state = BoardState::new_game();
        let board = board_state.board;
        let white_king_square = board.square(E1);
        assert!(white_king_square.is_empty_or_enemy_of(Black));
        assert!(!white_king_square.is_empty_or_enemy_of(White));
        let c6 = board.square(C6);
        assert!(c6.is_empty_or_enemy_of(White));
    }

    #[test]
    fn parsing_fen_with_en_passant_has_correct_position() {
        let board_state =
            BoardState::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3")
                .unwrap();

        assert_eq!(board_state.en_passant, Some(F6));
    }

    #[test]
    fn white_promoting_to_queen_in_good_scenario_leads_to_overwhelming_victory_eval() {
        let mut board_state = BoardState::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1").unwrap();

        assert!(generate_moves(&board_state).contains(&(A7, A8, Piece::queen(White)).into()));

        let eval_before = board_state.eval;

        board_state.make_move((A7, A8, Piece::queen(White)).into());

        let eval_after = board_state.eval;

        assert!((eval_after - eval_before) > 600);
    }

    #[test]
    fn black_promoting_to_queen_in_good_scenario_leads_to_overwhelming_victory_eval() {
        let mut board_state = BoardState::from_fen("7k/8/8/8/8/8/p7/7K b - - 0 1").unwrap();

        assert!(generate_moves(&board_state).contains(&(A2, A1, Piece::queen(Black)).into()));

        let eval_before = board_state.eval;

        board_state.make_move((A2, A1, Piece::queen(Black)).into());

        let eval_after = board_state.eval;

        assert!((eval_after - eval_before) < -600);
    }

    #[test]
    fn eval_in_drawish_game_is_close_to_zero() {
        let board_state = BoardState::from_fen("7k/8/8/8/8/8/8/7K w - - 0 1").unwrap();

        assert!(i32::abs(board_state.eval) < 50)
    }

    #[test]
    fn making_moves_and_returning_to_same_position_leads_to_identical_zobrist_key() {
        let mut board_state = BoardState::new_game();

        let starting_zobrist_key = board_state.zobrist_key;
        board_state.make_move((B1, C3).into());
        board_state.make_move((G8, F6).into());
        board_state.make_move((C3, B1).into());
        board_state.make_move((F6, G8).into());
        let zobrist_key_after_return_to_starting_position = board_state.zobrist_key;

        assert_eq!(
            starting_zobrist_key,
            zobrist_key_after_return_to_starting_position
        );
    }

    #[test]
    fn making_move_changes_zobrist_key() {
        let mut board_state = BoardState::new_game();
        let starting_zobrist_key = board_state.zobrist_key;

        board_state.make_move((E2, E4).into());

        let zobrist_key_after_move = board_state.zobrist_key;
        assert_ne!(starting_zobrist_key, zobrist_key_after_move);
    }

    #[test]
    fn returning_to_same_board_position_but_flipped_to_move_is_different_zobrist_key() {
        let mut board_state =
            BoardState::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
                .unwrap();
        let zobrist_key_before = board_state.zobrist_key;

        board_state.make_move((D1, G4).into());
        board_state.make_move((C8, F5).into());
        board_state.make_move((G4, E2).into());
        board_state.make_move((F5, C8).into());
        board_state.make_move((E2, D1).into());

        let zobrist_key_after = board_state.zobrist_key;
        assert_ne!(zobrist_key_before, zobrist_key_after);
    }
}
