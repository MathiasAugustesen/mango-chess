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
use crate::constants::*;
use crate::evaluation::evaluate;
use crate::evaluation::evaluate_piece;
use crate::fen;
use crate::fen::castling_rights_from_fen;
use crate::fen::en_passant_square_from_fen;
use crate::move_generation::generate_pseudo_moves_for_piece;
use crate::move_ordering::positional_value_delta;
use crate::ray_attacks::*;
use crate::GameResult;
#[derive(Debug, Clone, PartialEq)]
struct EntropyStack {
    stack: Vec<MoveEntropy>,
}
impl EntropyStack {
    pub fn new() -> EntropyStack {
        EntropyStack {
            stack: vec![MoveEntropy::default()],
        }
    }
    pub fn pop(&mut self) -> MoveEntropy {
        self.stack.pop().expect("Tried to unmake unexisting move")
    }
    pub fn top(&self) -> MoveEntropy {
        *self.stack.last().unwrap()
    }
    pub fn top_mut(&mut self) -> &mut MoveEntropy {
        self.stack.last_mut().unwrap()
    }
    pub fn last_move(&self) -> Option<ChessMove> {
        self.top().last_move
    }
    pub fn last_capture(&self) -> Option<Piece> {
        self.top().last_capture
    }
    pub fn eval(&self) -> i32 {
        self.top().eval
    }
    pub fn castling_rights(&self) -> CastlingRights {
        self.top().castling_rights
    }
    pub fn set_last_move(&mut self, mov: ChessMove) {
        self.top_mut().last_move = Some(mov);
    }
    pub fn set_last_capture(&mut self, captured_piece: Option<Piece>) {
        self.top_mut().last_capture = captured_piece
    }
    pub fn push(&mut self, mov: ChessMove, capture: Option<Piece>) {
        self.stack.push(MoveEntropy::new(
            mov,
            capture,
            self.eval(),
            self.castling_rights(),
        ))
    }
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct MoveEntropy {
    pub last_move: Option<ChessMove>,
    pub last_capture: Option<Piece>,
    pub eval: i32,
    pub castling_rights: CastlingRights,
}
impl MoveEntropy {
    pub fn new(
        mov: ChessMove,
        capture: Option<Piece>,
        eval: i32,
        castling_rights: CastlingRights,
    ) -> MoveEntropy {
        MoveEntropy {
            last_move: Some(mov),
            last_capture: capture,
            eval,
            castling_rights,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct ChessBoard([Square; 144]);
impl std::fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (BOARD_START..=BOARD_END).rev() {
            writeln!(f)?;
            for file in BOARD_START..=BOARD_END {
                write!(f, "{} ", self.square(ChessCell(rank, file)))?
            }
        }
        Ok(())
    }
}
impl ChessBoard {
    #[inline]
    pub fn square(&self, square: ChessCell) -> &Square {
        &self.0[square.0 * 12 + square.1]
    }
    #[inline]
    pub fn square_mut(&mut self, square: ChessCell) -> &mut Square {
        &mut self.0[square.0 * 12 + square.1]
    }
    pub fn empty() -> ChessBoard {
        let mut chess_board = ChessBoard([Square::Aether; 144]);
        for rank in RANK_1..=RANK_8 {
            for file in A_FILE..=H_FILE {
                *chess_board.square_mut(ChessCell(rank, file)) = Square::Empty;
            }
        }
        chess_board
    }
    pub fn new() -> ChessBoard {
        let mut chess_board = ChessBoard::empty();
        // Arrange pawns for both sides
        for file in A_FILE..=H_FILE {
            *chess_board.square_mut(ChessCell(RANK_2, file)) = Piece::pawn(White).into();
            *chess_board.square_mut(ChessCell(RANK_7, file)) = Piece::pawn(Black).into();
        }
        // Arrange white pieces
        *chess_board.square_mut(ChessCell(RANK_1, A_FILE)) = Piece::rook(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, B_FILE)) = Piece::knight(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, C_FILE)) = Piece::bishop(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, D_FILE)) = Piece::queen(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, E_FILE)) = Piece::king(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, F_FILE)) = Piece::bishop(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, G_FILE)) = Piece::knight(White).into();
        *chess_board.square_mut(ChessCell(RANK_1, H_FILE)) = Piece::rook(White).into();
        // Arrange black pieces
        *chess_board.square_mut(ChessCell(RANK_8, A_FILE)) = Piece::rook(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, B_FILE)) = Piece::knight(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, C_FILE)) = Piece::bishop(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, D_FILE)) = Piece::queen(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, E_FILE)) = Piece::king(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, F_FILE)) = Piece::bishop(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, G_FILE)) = Piece::knight(Black).into();
        *chess_board.square_mut(ChessCell(RANK_8, H_FILE)) = Piece::rook(Black).into();
        chess_board
    }
}

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
    /*
    pub en_passant_square: Option<ChessCell>,
    pub pawn_promotion: Option<ChessCell>,
    pub castling_rights: CastlingRights,
    */
    entropy_stack: EntropyStack,
}
impl BoardState {
    pub fn empty_game() -> BoardState {
        let board = ChessBoard::empty();
        let to_move = White;
        let white_bitboard = BitBoard(0);
        let black_bitboard = BitBoard(0);
        let white_king_location = ChessCell(100, 100);
        let black_king_location = ChessCell(100, 100);
        let _en_passant_square: Option<()> = None;
        let _pawn_promotion: Option<()> = None;
        let entropy_stack = EntropyStack::new();
        return BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            entropy_stack,
        };
    }
    pub fn new_game() -> BoardState {
        let board = ChessBoard::new();
        let to_move = White;
        let white_bitboard = WHITE_STARTING_BITBOARD;
        let black_bitboard = BLACK_STARTING_BITBOARD;
        let white_king_location = ChessCell(RANK_1, E_FILE);
        let black_king_location = ChessCell(RANK_8, E_FILE);
        let _castling_rights = CastlingRights::all_castling_rights();
        let _en_passant_square: Option<()> = None;
        let _pawn_promotion: Option<()> = None;
        let entropy_stack = EntropyStack::new();

        let mut board_state = BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            entropy_stack,
        };
        board_state.set_eval(evaluate(&board_state));
        board_state.set_castling_rights(CastlingRights::all_castling_rights());
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
            entropy_stack: EntropyStack::new(),
        };
        board_state.set_eval(evaluate(&board_state));
        board_state.set_castling_rights(castling_rights);
        Ok(board_state)
    }
    #[inline]
    pub fn swap_to_move(&mut self) {
        match self.to_move {
            White => self.to_move = Black,
            Black => self.to_move = White,
        }
    }
    #[inline]
    pub fn clear_en_passant_square(&mut self) {
        //self.en_passant_square = None;
    }
    #[inline]
    pub fn last_move(&self) -> Option<ChessMove> {
        self.entropy_stack.last_move()
    }
    #[inline]
    pub fn last_capture(&self) -> Option<Piece> {
        self.entropy_stack.last_capture()
    }
    #[inline]
    pub fn eval(&self) -> i32 {
        self.entropy_stack.eval() * self.to_move.relative_value()
    }
    #[inline]
    pub fn castling_rights(&self) -> CastlingRights {
        self.entropy_stack.castling_rights()
    }
    pub fn available_castling_types(&self, color: PieceColor) -> Vec<CastlingType> {
        let mut castling_types = Vec::new();
        match color {
            White => {
                if self.castling_rights().white_king_side_castling {
                    castling_types.push(CastlingType::WhiteKingSide)
                }
                if self.castling_rights().white_queen_side_castling {
                    castling_types.push(CastlingType::WhiteQueenSide)
                }
            }
            Black => {
                if self.castling_rights().black_king_side_castling {
                    castling_types.push(CastlingType::BlackKingSide)
                }
                if self.castling_rights().black_queen_side_castling {
                    castling_types.push(CastlingType::BlackQueenSide)
                }
            }
        }
        castling_types
    }
    #[inline]
    pub fn set_last_move(&mut self, mov: ChessMove) {
        self.entropy_stack.set_last_move(mov)
    }
    #[inline]
    pub fn set_last_capture(&mut self, captured_piece: Option<Piece>) {
        self.entropy_stack.set_last_capture(captured_piece)
    }
    pub fn set_eval(&mut self, eval: i32) {
        self.entropy_stack.top_mut().eval = eval
    }
    pub fn increment_eval(&mut self, eval_increment: i32) {
        self.entropy_stack.top_mut().eval += eval_increment * self.to_move.relative_value()
    }
    pub fn set_castling_rights(&mut self, castling_rights: CastlingRights) {
        self.entropy_stack.top_mut().castling_rights = castling_rights
    }
    pub fn remove_castling_right(&mut self, castling_type: CastlingType) {
        self.entropy_stack
            .top_mut()
            .castling_rights
            .remove_castling_right(castling_type)
    }
    pub fn remove_color_castling_rights(&mut self, color: PieceColor) {
        self.entropy_stack
            .top_mut()
            .castling_rights
            .remove_color_castling_rights(color)
    }
    pub fn make_move(&mut self, mov: ChessMove) {
        let start = mov.start;
        let dest = mov.dest;
        let mut eval_increment = 0;

        let moving_piece = self.board.square(start).piece().unwrap();

        *self.board.square_mut(start) = Square::Empty;

        let attacked_square = self.board.square(dest);

        if let Some(attacked_piece) = attacked_square.piece() {
            eval_increment += evaluate_piece(attacked_piece, dest.as_index());
        }

        self.entropy_stack.push(mov, attacked_square.piece());

        *self.board.square_mut(dest) = Square::Full(moving_piece);

        eval_increment += positional_value_delta(moving_piece, mov);

        self.increment_eval(eval_increment);

        if moving_piece.kind == King {
            match moving_piece.color {
                White => self.white_king_location = dest,
                Black => self.black_king_location = dest,
            }
            // King moved two squares, must be castle
            if (start.1 as i8 - dest.1 as i8).abs() == 2 {
                let starting_rank = match self.to_move {
                    White => RANK_1,
                    Black => RANK_8,
                };
                let (rook_start, rook_dest) = match dest.1 {
                    C_FILE => (
                        ChessCell(starting_rank, A_FILE),
                        ChessCell(starting_rank, D_FILE),
                    ),
                    G_FILE => (
                        ChessCell(starting_rank, H_FILE),
                        ChessCell(starting_rank, F_FILE),
                    ),
                    _ => unreachable!(),
                };
                *self.board.square_mut(rook_dest) = *self.board.square(rook_start);
                *self.board.square_mut(rook_start) = Square::Empty;
                let rook_mov = ChessMove {
                    start: rook_start,
                    dest: rook_dest,
                };
                self.update_bitboards(rook_mov)
            }
            self.remove_color_castling_rights(self.to_move);
        }
        if moving_piece.kind == Rook {
            match start {
                ChessCell(RANK_1, A_FILE) => {
                    self.remove_castling_right(CastlingType::WhiteQueenSide)
                }
                ChessCell(RANK_1, H_FILE) => {
                    self.remove_castling_right(CastlingType::WhiteKingSide)
                }
                ChessCell(RANK_8, A_FILE) => {
                    self.remove_castling_right(CastlingType::BlackQueenSide)
                }
                ChessCell(RANK_8, H_FILE) => {
                    self.remove_castling_right(CastlingType::BlackKingSide)
                }
                _ => {}
            };
        }
        if let Some(captured_piece) = self.board.square(dest).piece() {
            if captured_piece.kind == Rook {
                match dest {
                    ChessCell(RANK_1, A_FILE) => {
                        self.remove_castling_right(CastlingType::WhiteQueenSide)
                    }
                    ChessCell(RANK_1, H_FILE) => {
                        self.remove_castling_right(CastlingType::WhiteKingSide)
                    }
                    ChessCell(RANK_8, A_FILE) => {
                        self.remove_castling_right(CastlingType::BlackQueenSide)
                    }
                    ChessCell(RANK_8, H_FILE) => {
                        self.remove_castling_right(CastlingType::BlackKingSide)
                    }
                    _ => {}
                }
            }
        }

        self.update_bitboards(mov);

        self.swap_to_move();
    }
    pub fn unmake_move(&mut self) {
        self.swap_to_move();
        let reverse_move = self.last_move().unwrap().reverse();
        let start = reverse_move.start;
        let dest = reverse_move.dest;
        let moving_piece = self.board.square(start).piece().unwrap();

        *self.board.square_mut(start) = Square::Empty;

        *self.board.square_mut(dest) = Square::Full(moving_piece);
        self.downgrade_bitboards(reverse_move, self.last_capture());

        if let Some(captured_piece) = self.last_capture() {
            *self.board.square_mut(reverse_move.start) = Square::Full(captured_piece);
        }
        if moving_piece.kind == King {
            match moving_piece.color {
                White => self.white_king_location = dest,
                Black => self.black_king_location = dest,
            }
            // King moved two squares, must be castle
            if (start.1 as i8 - dest.1 as i8).abs() == 2 {
                let starting_rank = match self.to_move {
                    White => RANK_1,
                    Black => RANK_8,
                };
                // Uncastling the rook
                let (rook_dest, rook_start) = match start.1 {
                    C_FILE => (
                        ChessCell(starting_rank, A_FILE),
                        ChessCell(starting_rank, D_FILE),
                    ),
                    G_FILE => (
                        ChessCell(starting_rank, H_FILE),
                        ChessCell(starting_rank, F_FILE),
                    ),
                    _ => {
                        dbg!(reverse_move);
                        panic!()
                    }
                };
                *self.board.square_mut(rook_dest) = *self.board.square(rook_start);
                *self.board.square_mut(rook_start) = Square::Empty;
                let rook_mov = ChessMove {
                    start: rook_start,
                    dest: rook_dest,
                };
                self.downgrade_bitboards(rook_mov, None);
            }
        }

        self.entropy_stack.pop();
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
    pub fn downgrade_bitboards(&mut self, mov: ChessMove, last_capture: Option<Piece>) {
        let (current_player_bitboard, opposing_player_bitboard) = match self.to_move {
            White => (&mut self.white_bitboard, &mut self.black_bitboard),
            Black => (&mut self.black_bitboard, &mut self.white_bitboard),
        };
        current_player_bitboard.remove_piece(mov.start.as_index());
        current_player_bitboard.add_piece(mov.dest.as_index());

        if last_capture.is_some() {
            opposing_player_bitboard.add_piece(mov.start.as_index());
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
    // First, a list of all enemy pieces that x-ray the target is generated with a lookup table.
    // If this list is empty, the square is safe.
    // Returns true if any of the enemy pieces attack the target.
    pub fn square_is_attacked(&self, target_square: ChessCell, attacker: PieceColor) -> bool {
        let ray_attackers = self.ray_attackers(target_square, attacker);
        if ray_attackers.len() == 0 {
            return false;
        }
        /*
        This block perform the regular 'check every piece' way of validating pseudo moves.
        let mut ray_attackers: Vec<(Piece, ChessCell)> = Vec::new();
        let enemy_pieces = self.get_piece_positions(self.to_move);
        for piece_position in enemy_pieces {
            let piece = self.board[piece_position.0][piece_position.1].piece();
            ray_attackers.push((piece, piece_position));
        }
        */
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
    use std::str::FromStr;

    use super::*;
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
        assert_eq!(fen_board_state.entropy_stack, new_board_state.entropy_stack);
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
            ChessCell(RANK_1, A_FILE),
            ChessCell(RANK_1, B_FILE),
            ChessCell(RANK_1, C_FILE),
            ChessCell(RANK_1, D_FILE),
            ChessCell(RANK_1, F_FILE),
            ChessCell(RANK_1, G_FILE),
            ChessCell(RANK_1, H_FILE),
            ChessCell(RANK_2, A_FILE),
            ChessCell(RANK_2, B_FILE),
            ChessCell(RANK_2, C_FILE),
            ChessCell(RANK_2, D_FILE),
            ChessCell(RANK_2, E_FILE),
            ChessCell(RANK_2, F_FILE),
            ChessCell(RANK_2, G_FILE),
            ChessCell(RANK_2, H_FILE),
            ChessCell(RANK_4, E_FILE),
        ];
        for position in white_positions {
            assert!(white_bongcloud_positions.contains(&position))
        }
        let black_positions = board_state.get_piece_positions(Black);
        let black_bongcloud_positions = [
            ChessCell(RANK_8, A_FILE),
            ChessCell(RANK_8, B_FILE),
            ChessCell(RANK_8, C_FILE),
            ChessCell(RANK_8, D_FILE),
            ChessCell(RANK_8, E_FILE),
            ChessCell(RANK_8, F_FILE),
            ChessCell(RANK_8, G_FILE),
            ChessCell(RANK_8, H_FILE),
            ChessCell(RANK_7, A_FILE),
            ChessCell(RANK_7, B_FILE),
            ChessCell(RANK_7, C_FILE),
            ChessCell(RANK_7, D_FILE),
            ChessCell(RANK_7, F_FILE),
            ChessCell(RANK_7, G_FILE),
            ChessCell(RANK_7, H_FILE),
            ChessCell(RANK_5, E_FILE),
        ];
        for position in black_positions {
            assert!(black_bongcloud_positions.contains(&position))
        }
    }
    #[test]
    fn is_empty_or_enemy_of_works() {
        let board_state = BoardState::new_game();
        let board = board_state.board;
        let white_king_square = board.square(ChessCell(RANK_1, E_FILE));
        assert_eq!(white_king_square.is_empty_or_enemy_of(Black), true);
        assert_eq!(white_king_square.is_empty_or_enemy_of(White), false);
        let c6 = board.square(ChessCell(RANK_6, C_FILE));
        assert_eq!(c6.is_empty_or_enemy_of(White), true);
    }
    #[test]
    fn make_unmake_on_new_board_restores_board_state() {
        let mut board_state = BoardState::new_game();
        let mov = ChessMove {
            start: ChessCell(RANK_2, E_FILE),
            dest: ChessCell(RANK_4, E_FILE),
        };
        board_state.make_move(mov);
        board_state.unmake_move();
        assert_eq!(board_state, BoardState::new_game())
    }
    #[test]
    fn make_unmake_on_board_with_capture_restores_board_state() {
        let mut board_state =
            BoardState::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
                .unwrap();
        let mov = ChessMove {
            start: ChessCell(RANK_4, E_FILE),
            dest: ChessCell(RANK_5, D_FILE),
        };
        board_state.make_move(mov);
        board_state.unmake_move();
        assert_eq!(
            board_state,
            BoardState::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
                .unwrap()
        )
    }
}
