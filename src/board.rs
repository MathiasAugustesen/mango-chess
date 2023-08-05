use crate::constants::*;
use crate::evaluation::evaluate;
use crate::evaluation::evaluate_piece;
use crate::fen;
use crate::fen::castling_rights_from_fen;
use crate::fen::en_passant_square_from_fen;
use crate::move_generation::generate_pseudo_moves_for_piece;
use crate::move_ordering::positional_value_delta;
use crate::ray_attacks::*;
use crate::ChessMove;
use crate::GameResult;
use std::str::FromStr;
use PieceColor::*;
use PieceKind::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}
impl std::fmt::Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            White => write!(f, "white"),
            Black => write!(f, "black"),
        }
    }
}
impl PieceColor {
    pub fn opposite(self) -> PieceColor {
        match self {
            White => Black,
            Black => White,
        }
    }
    pub fn relative_value(self) -> i32 {
        match self {
            White => 1,
            Black => -1,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceKind {
    pub const fn index(self) -> usize {
        match self {
            Pawn => 0,
            Knight => 1,
            Bishop => 2,
            Rook => 3,
            Queen => 4,
            King => 5,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Full(Piece),
    Aether,
}
impl Square {
    pub fn is_aether(self) -> bool {
        self == Square::Aether
    }
    pub fn is_inside_board(self) -> bool {
        self != Square::Aether
    }
    pub fn is_empty(self) -> bool {
        self == Square::Empty
    }
    pub fn is_color(self, color: PieceColor) -> bool {
        match self {
            Square::Full(Piece {
                color: square_color,
                ..
            }) => color == square_color,
            _ => false,
        }
    }
    pub fn is_enemy_of(self, color: PieceColor) -> bool {
        match self {
            Square::Full(Piece {
                color: square_color,
                ..
            }) => color != square_color,
            _ => false,
        }
    }
    pub fn has_piece(self) -> bool {
        match self {
            Square::Full(_) => true,
            _ => false,
        }
    }
    pub fn is_empty_or_enemy_of(self, color: PieceColor) -> bool {
        match self {
            Square::Full(Piece {
                color: square_color,
                ..
            }) => color != square_color,
            Square::Empty => true,
            _ => false,
        }
    }
    pub fn piece(self) -> Option<Piece> {
        match self {
            Square::Full(piece) => Some(piece),
            Square::Empty => None,
            Square::Aether => panic!("Tried to access piece from Aether"),
        }
    }
}
impl From<Piece> for Square {
    fn from(value: Piece) -> Self {
        Square::Full(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind,
}
impl Piece {
    pub fn color(self) -> PieceColor {
        self.color
    }
    pub fn kind(self) -> PieceKind {
        self.kind
    }
    fn pawn(color: PieceColor) -> Piece {
        Piece { color, kind: Pawn }
    }
    fn knight(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Knight,
        }
    }
    fn bishop(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Bishop,
        }
    }
    fn rook(color: PieceColor) -> Piece {
        Piece { color, kind: Rook }
    }
    fn queen(color: PieceColor) -> Piece {
        Piece { color, kind: Queen }
    }
    fn king(color: PieceColor) -> Piece {
        Piece { color, kind: King }
    }
    #[inline]
    pub fn index(self) -> usize {
        self.kind.index()
    }
    pub fn value(self) -> i32 {
        match self.kind {
            Pawn => 100,
            Knight => 300,
            Bishop => 325,
            Rook => 500,
            Queen => 900,
            King => 10000,
        }
    }
}
impl TryFrom<char> for Piece {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let color = match value.is_lowercase() {
            true => Black,
            false => White,
        };
        let piece = match value.to_ascii_lowercase() {
            'p' => Piece::pawn(color),
            'n' => Piece::knight(color),
            'b' => Piece::bishop(color),
            'r' => Piece::rook(color),
            'q' => Piece::queen(color),
            'k' => Piece::king(color),
            _ => return Err("Character is not a valid chess piece"),
        };
        Ok(piece)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CastlingType {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}
impl From<ChessCell> for CastlingType {
    fn from(value: ChessCell) -> Self {
        match value {
            A1 => CastlingType::WhiteQueenSide,
            H1 => CastlingType::WhiteKingSide,
            A8 => CastlingType::BlackQueenSide,
            H8 => CastlingType::BlackKingSide,
            _ => unreachable!(),
        }
    }
}
impl CastlingType {
    pub fn color_king_side(color: PieceColor) -> CastlingType {
        match color {
            White => CastlingType::WhiteKingSide,
            Black => CastlingType::BlackKingSide,
        }
    }
    pub fn color_queen_side(color: PieceColor) -> CastlingType {
        match color {
            White => CastlingType::WhiteQueenSide,
            Black => CastlingType::BlackQueenSide,
        }
    }
    pub fn direction(self) -> i8 {
        match self {
            CastlingType::WhiteKingSide | CastlingType::BlackKingSide => 1,
            CastlingType::WhiteQueenSide | CastlingType::BlackQueenSide => -1,
        }
    }
}
#[derive(Default, Clone, PartialEq, Debug, Copy)]
pub struct CastlingRights {
    pub white_king_side_castling: bool,
    pub white_queen_side_castling: bool,
    pub black_king_side_castling: bool,
    pub black_queen_side_castling: bool,
}

impl CastlingRights {
    pub fn no_castling_rights() -> CastlingRights {
        CastlingRights {
            white_king_side_castling: false,
            white_queen_side_castling: false,
            black_king_side_castling: false,
            black_queen_side_castling: false,
        }
    }
    pub fn all_castling_rights() -> CastlingRights {
        CastlingRights {
            white_king_side_castling: true,
            white_queen_side_castling: true,
            black_king_side_castling: true,
            black_queen_side_castling: true,
        }
    }
    pub fn remove_castling_right(&mut self, castling_type: CastlingType) {
        match castling_type {
            CastlingType::WhiteKingSide => self.white_king_side_castling = false,
            CastlingType::WhiteQueenSide => self.white_queen_side_castling = false,
            CastlingType::BlackKingSide => self.black_king_side_castling = false,
            CastlingType::BlackQueenSide => self.black_queen_side_castling = false,
        }
    }
    pub fn remove_color_castling_rights(&mut self, color: PieceColor) {
        match color {
            White => {
                self.white_king_side_castling = false;
                self.white_queen_side_castling = false;
            }
            Black => {
                self.black_king_side_castling = false;
                self.black_queen_side_castling = false;
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);
impl BitBoard {
    pub fn remove_piece(&mut self, index: usize) {
        let mask = !(1 << index);
        self.0 &= mask;
    }
    pub fn add_piece(&mut self, index: usize) {
        let mask = 1 << index;
        self.0 |= mask;
    }
    pub fn bits_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}
impl Iterator for BitBoard {
    type Item = ChessCell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let position = self.0.leading_zeros() as usize;
        let cell = ChessCell::from_index(position);
        self.0 ^= 1 << position;
        Some(cell)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct ChessBoard([Square; 144]);
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
pub struct BoardData {
    pub to_move: PieceColor,
    pub white_bitboard: BitBoard,
    pub black_bitboard: BitBoard,
    pub white_king_location: ChessCell,
    pub black_king_location: ChessCell,
    pub last_move: Option<ChessMove>,
    pub last_capture: Option<Piece>,
    pub eval: i32,
    pub castling_rights: CastlingRights,
}
impl BoardData {
    #[inline]
    pub fn relative_eval(&self) -> i32 {
        self.eval * self.to_move.relative_value()
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
        self.eval += eval_increment * self.to_move.relative_value()
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
    pub fn king_location_of(&self, color: PieceColor) -> ChessCell {
        match color {
            White => self.white_king_location,
            Black => self.black_king_location,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct BoardState {
    pub board: ChessBoard,
    data: Vec<BoardData>,
}

impl BoardState {
    pub fn empty_game() -> BoardState {
        let board = ChessBoard::empty();

        let to_move = White;
        let white_bitboard = BitBoard(0);
        let black_bitboard = BitBoard(0);
        let white_king_location = ChessCell(100, 100);
        let black_king_location = ChessCell(100, 100);

        let data = BoardData {
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            last_capture: None,
            eval: 0,
            castling_rights: CastlingRights::no_castling_rights(),
        };
        return BoardState {
            board,
            data: vec![data],
        };
    }
    pub fn new_game() -> BoardState {
        let board = ChessBoard::new();
        let to_move = White;
        let white_bitboard = WHITE_STARTING_BITBOARD;
        let black_bitboard = BLACK_STARTING_BITBOARD;
        let white_king_location = ChessCell(RANK_1, E_FILE);
        let black_king_location = ChessCell(RANK_8, E_FILE);
        let castling_rights = CastlingRights::all_castling_rights();

        let data = BoardData {
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            last_capture: None,
            eval: 0,
            castling_rights,
        };
        let mut board_state = BoardState {
            board,
            data: vec![data],
        };
        board_state.data_mut().eval = evaluate(&board_state);
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
        let data = BoardData {
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            last_move: None,
            last_capture: None,
            eval: 0,
            castling_rights,
        };

        let mut board_state = BoardState {
            board,
            data: vec![data],
        };
        board_state.data_mut().eval = evaluate(&board_state);
        Ok(board_state)
    }
    pub fn move_piece(&mut self, mov: ChessMove) {
        let start = mov.start;
        let dest = mov.dest;
        let mut eval_increment = 0;

        let moving_piece = self.board.square(start).piece().unwrap();
        let target_square = *self.board.square(dest);
        
        if let Some(captured_piece) = target_square.piece() {
            eval_increment += evaluate_piece(captured_piece, dest.as_index());
        }
        eval_increment += positional_value_delta(moving_piece, mov);

        *self.board.square_mut(dest) = *self.board.square(start);
        *self.board.square_mut(start) = Square::Empty;

        self.data_mut().update_bitboards(mov);

        self.data_mut().increment_eval(eval_increment)
    }
    pub fn data(&self) -> &BoardData {
        self.data.last().unwrap()
    }
    pub fn data_mut(&mut self) -> &mut BoardData {
        self.data.last_mut().unwrap()
    }
    pub fn pop_data(&mut self) {
        self.data.pop().unwrap();
    }
    pub fn push_new(&mut self) {
        let previous = self.data();

        let board_data = BoardData {
            to_move: previous.to_move.opposite(),
            white_bitboard: previous.white_bitboard,
            black_bitboard: previous.black_bitboard,
            white_king_location: previous.white_king_location,
            black_king_location: previous.black_king_location,
            last_move: None,
            last_capture: None,
            eval: previous.eval,
            castling_rights: previous.castling_rights,
        };
        self.data.push(board_data)
    }
    // Will also increment eval if the rook moves
    fn resolve_castling(&mut self, mov: ChessMove) {
        let ChessMove { start, dest } = mov;
        if (start.1 as i8 - dest.1 as i8).abs() == 2 {

            let (rook_start, rook_dest) = match dest {
                G1 => (H1, F1),
                C1 => (A1, D1),
                G8 => (H8, F8),
                C8 => (A8, D8),
                _ => unreachable!(),
            };
            let rook_move = ChessMove {
                start: rook_start,
                dest: rook_dest,
            };
            self.move_piece(rook_move);
        }
        let data = self.data_mut();
        let to_move = data.to_move;
        data
            .castling_rights
            .remove_color_castling_rights(to_move);
    }
    pub fn make_move(&mut self, mov: ChessMove) {
        let ChessMove { start, dest } = mov;
        self.push_new();
        let last_capture = self.board.square(dest).piece();

            let data = self.data_mut();
            data.last_move = Some(mov);
            data.last_capture = last_capture;

        let moving_piece = self.board.square(start).piece().unwrap();

        if moving_piece.kind == King {
            match moving_piece.color {
                White => self.data_mut().white_king_location = dest,
                Black => self.data_mut().black_king_location = dest,
            }
            self.resolve_castling(mov);
        }
        if moving_piece.kind == Rook && matches!(start, A1 | A8 | H1 | H8) {
            self.data_mut()
                .castling_rights
                .remove_castling_right(start.into())
        }
        if matches!(dest, A1 | A8 | H1 | H8) {
            self.data_mut()
                .castling_rights
                .remove_castling_right(dest.into())
        }
        self.move_piece(mov);

    }
    pub fn unmake_move(&mut self) {
        let data = self.data();
        let last_capture = data.last_capture;
        let reverse_move = data.last_move.unwrap().reverse();
        let start = reverse_move.start;
        let dest = reverse_move.dest;


        let moving_piece = self.board.square(start).piece().unwrap();

        self.move_piece(reverse_move);


        if let Some(captured_piece) = last_capture {
            *self.board.square_mut(start) = Square::Full(captured_piece);
        }
        if moving_piece.kind() == King && (start.1 as i8 - dest.1 as i8).abs() == 2 {
            // King moved two squares, must be castle
            let (rook_start, rook_dest) = match start {
                C1 => (D1, A1),
                G1 => (F1, H1),
                C8 => (D8, A8),
                G8 => (F8, H8),
                _ => unreachable!()
            };
            let rook_move = ChessMove {
                start: rook_start,
                dest: rook_dest
            };
            self.move_piece(rook_move)
        }

        self.pop_data();
    }
    pub fn get_game_winner(&self) -> GameResult {
        let data = self.data();
        let potentially_mated_king = data.king_location_of(data.to_move);
        match self.square_is_attacked(potentially_mated_king, data.to_move.opposite()) {
            true => GameResult::Winner(data.to_move.opposite()),
            false => GameResult::Draw,
        }
    }
    pub fn get_piece_positions(&self, color: PieceColor) -> Vec<ChessCell> {
        let mut piece_positions = Vec::new();
        let mut bitboard = match color {
            White => self.data().white_bitboard,
            Black => self.data().black_bitboard,
        };
        while bitboard.0 != 0 {
            let position = bitboard.0.trailing_zeros();
            let cell = ChessCell::from_index(position as usize);
            piece_positions.push(cell);
            bitboard.0 ^= 1 << position;
        }
        piece_positions
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
        let data = self.data();
        let king_location = data.king_location_of(data.to_move.opposite());
        !self.square_is_attacked(king_location, data.to_move)
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
            if attacking_square.piece().is_none() {
                dbg!(self);
            }
            let attacking_piece = attacking_square.piece().unwrap();
            let attacked_squares = match (attacking_piece.color(), attacking_piece.kind()) {
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
// ChessCell represents a valid algebraic square on the board
// The format is row, col, or rank, file in chess terms.
// b4 will therefore translate to ChessCell(3, 5)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessCell(pub usize, pub usize);
impl From<(usize, usize)> for ChessCell {
    fn from(value: (usize, usize)) -> Self {
        ChessCell(value.0, value.1)
    }
}

impl ChessCell {
    pub fn as_index(self) -> usize {
        let rank_index = self.0 - BOARD_START;
        let file_index = self.1 - BOARD_START;
        rank_index * 8 + file_index
    }
    pub fn from_index(index: usize) -> ChessCell {
        let rank = index / 8 + BOARD_START;
        let file = index % 8 + BOARD_START;
        ChessCell(rank, file)
    }
    pub fn is_aether(self) -> bool {
        self.0 <= 1 || self.0 >= 10 || self.1 <= 1 || self.1 >= 10
    }
}
impl FromStr for ChessCell {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err("Failed to parse ChessCell from string: Length was not 2");
        }
        let col = s.chars().next().unwrap();
        let row = s.chars().nth(1).unwrap();

        let file = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Failed to parse ChessCell from string: Invalid file"),
        };
        let row = row.to_digit(10);
        if row.is_none() {
            return Err("Failed to parse ChessCell from string: rank was not a valid number");
        }
        let rank = BOARD_END - row.unwrap() as usize;
        Ok(ChessCell(rank, file + BOARD_START))
    }
}
impl std::fmt::Display for ChessCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = self.0 - 1;
        let file = match self.1 - 1 {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("Tried to print square that was outside the board"),
        };
        write!(f, "{}{}", file, rank)
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
                if piece.kind() == King {
                    match piece.color() {
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
        let fen_data = fen_board_state.data();
        let new_data = new_board_state.data();
        assert_eq!(fen_data, new_data);
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
        assert_eq!(board_state.board, BoardState::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .unwrap().board);
        assert_eq!(
            board_state,
            BoardState::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
                .unwrap()
        )
    }
}
