use crate::constants::*;
use crate::fen;
use crate::fen::castling_rights_from_fen;
use crate::fen::en_passant_square_from_fen;
use crate::move_generation::generate_pseudo_moves_for_piece;
use crate::ray_attacks::*;
use crate::ChessMove;
use std::str::FromStr;
use std::thread::current;
use PieceColor::*;
use PieceKind::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
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
    pub fn piece(self) -> Piece {
        match self {
            Square::Full(piece) => piece,
            Square::Empty => {
                panic!("Expected piece to be present at square but square was Empty")
            }
            Square::Aether => {
                panic!("Expected piece to be present at square but square was Aether")
            }
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
    const fn pawn(color: PieceColor) -> Piece {
        Piece { color, kind: Pawn }
    }
    const fn knight(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Knight,
        }
    }
    const fn bishop(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Bishop,
        }
    }
    const fn rook(color: PieceColor) -> Piece {
        Piece { color, kind: Rook }
    }
    const fn queen(color: PieceColor) -> Piece {
        Piece { color, kind: Queen }
    }
    const fn king(color: PieceColor) -> Piece {
        Piece { color, kind: King }
    }
    pub const fn index(self) -> usize {
        self.kind.index()
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
pub enum CastlingType {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}
#[derive(Clone, PartialEq, Debug)]
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
    pub fn remove_white_castling_rights(&mut self) {
        self.white_king_side_castling = false;
        self.white_queen_side_castling = false;
    }
    pub fn remove_black_castling_rights(&mut self) {
        self.black_king_side_castling = false;
        self.black_queen_side_castling = false;
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
#[derive(Debug, Clone, PartialEq)]
struct EntropyStack {
    stack: Vec<MoveEntropy>,
}
impl EntropyStack {
    pub fn new() -> EntropyStack {
        EntropyStack { stack: vec![MoveEntropy::default()] }
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
    pub fn set_last_move(&mut self, mov: ChessMove) {
        self.top_mut().last_move = Some(mov);
    }
    pub fn set_last_capture(&mut self, captured_piece: Option<Piece>) {
        self.top_mut().last_capture = captured_piece
    }
    pub fn push(&mut self, mov: ChessMove, capture: Option<Piece>) {
        self.stack.push(MoveEntropy::new(mov, capture))
    }
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct MoveEntropy {
    pub last_move: Option<ChessMove>,
    pub last_capture: Option<Piece>,
}
impl MoveEntropy {
    pub fn new(mov: ChessMove, capture: Option<Piece>) -> MoveEntropy {
        MoveEntropy { last_move: Some(mov), last_capture: capture }
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct BoardState {
    board: [[Square; 12]; 12],
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
        let board = empty_board();
        let to_move = White;
        let white_bitboard = BitBoard(0);
        let black_bitboard = BitBoard(0);
        let white_king_location = ChessCell(100, 100);
        let black_king_location = ChessCell(100, 100);
        let _castling_rights = CastlingRights::no_castling_rights();
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
        let mut board = empty_board();
        // Arrange pawns for both sides
        for file in A_FILE..=H_FILE {
            board[RANK_2][file] = Piece::pawn(White).into();
            board[RANK_7][file] = Piece::pawn(Black).into();
        }
        // Arrange white pieces
        board[RANK_1][A_FILE] = Piece::rook(White).into();
        board[RANK_1][B_FILE] = Piece::knight(White).into();
        board[RANK_1][C_FILE] = Piece::bishop(White).into();
        board[RANK_1][D_FILE] = Piece::queen(White).into();
        board[RANK_1][E_FILE] = Piece::king(White).into();
        board[RANK_1][F_FILE] = Piece::bishop(White).into();
        board[RANK_1][G_FILE] = Piece::knight(White).into();
        board[RANK_1][H_FILE] = Piece::rook(White).into();
        // Arrange black pieces
        board[RANK_8][A_FILE] = Piece::rook(Black).into();
        board[RANK_8][B_FILE] = Piece::knight(Black).into();
        board[RANK_8][C_FILE] = Piece::bishop(Black).into();
        board[RANK_8][D_FILE] = Piece::queen(Black).into();
        board[RANK_8][E_FILE] = Piece::king(Black).into();
        board[RANK_8][F_FILE] = Piece::bishop(Black).into();
        board[RANK_8][G_FILE] = Piece::knight(Black).into();
        board[RANK_8][H_FILE] = Piece::rook(Black).into();
        let to_move = White;
        let white_bitboard = WHITE_STARTING_BITBOARD;
        let black_bitboard = BLACK_STARTING_BITBOARD;
        let white_king_location = ChessCell(RANK_1, E_FILE);
        let black_king_location = ChessCell(RANK_8, E_FILE);
        let _castling_rights = CastlingRights::all_castling_rights();
        let _en_passant_square: Option<()> = None;
        let _pawn_promotion: Option<()> = None;
        let entropy_stack = EntropyStack::new();
        BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            entropy_stack
        }
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
        let _castling_rights = castling_rights_from_fen(fen_castling_rights)?;
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
        let board_state = BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            entropy_stack: EntropyStack::new()
        };
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
    pub fn last_move(&self) -> Option<ChessMove> {
        self.entropy_stack.last_move()
    }
    pub fn last_capture(&self) -> Option<Piece> {
        self.entropy_stack.last_capture()
    }
    pub fn set_last_move(&mut self, mov: ChessMove) {
        self.entropy_stack.set_last_move(mov)
    }
    pub fn set_last_capture(&mut self, captured_piece: Option<Piece>) {
        self.entropy_stack.set_last_capture(captured_piece)
    }
    #[inline]
    pub fn square(&self, square: ChessCell) -> &Square{
        &self.board[square.0][square.1]
    }
    #[inline]
    pub fn square_mut(&mut self, square: ChessCell) -> &mut Square {
        &mut self.board[square.0][square.1]
    }
    pub fn make_move(&mut self, mov: ChessMove) {
        self.entropy_stack.push(mov, None);
        let start = mov.start;
        let dest = mov.dest;
        let piece = self.square(start).piece();
        *self.square_mut(start) = Square::Empty;

        let attacked_square = self.square(dest);
        if attacked_square.has_piece() {
            self.set_last_capture(Some(attacked_square.piece()));
        } else {
            self.set_last_capture(None);
        }
        *self.square_mut(dest) = Square::Full(piece);

        if piece.kind() == King {
            match piece.color() {
                White => self.white_king_location = dest,
                Black => self.black_king_location = dest,
            }
        }

        self.update_bitboards(mov);

        self.swap_to_move();
        dbg!(&self.entropy_stack);
    }
    pub fn unmake_move(&mut self) {
        self.swap_to_move();
        let reverse_move = self.last_move().unwrap().reverse();
        let start = reverse_move.start;
        let dest = reverse_move.dest;
        let moving_piece = self.square(start).piece();
        *self.square_mut(start) = Square::Empty;
        *self.square_mut(dest) = Square::Full(moving_piece);
        self.downgrade_bitboards(reverse_move, self.last_capture());

        if let Some(captured_piece) = self.last_capture() {
            *self.square_mut(reverse_move.start) = Square::Full(captured_piece);
        }
        if moving_piece.kind() == King {
            match moving_piece.color() {
                White => self.white_king_location = dest,
                Black => self.black_king_location = dest,
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
    // Given an arbitrary position, determine if the position is legal given that the player next to move is self.to_move.
    // This method does not make any assumptions about how the move was made
    //
    // First, a list of all enemy pieces that x-ray the king is generated with a lookup table. If this list is empty, the king is safe.
    // For all pieces that x-ray the king, the moves of that piece are generated, and will return true if any piece attacks the king.
    pub fn is_valid_move(&self) -> bool {
        let king_location = match self.to_move.opposite() {
            White => self.white_king_location,
            Black => self.black_king_location,
        };
        /*
        This block perform the regular 'check every piece' way of validating pseudo moves.
        let mut ray_attackers: Vec<(Piece, ChessCell)> = Vec::new();
        let enemy_pieces = self.get_piece_positions(self.to_move);
        for piece_position in enemy_pieces {
            let piece = self.board[piece_position.0][piece_position.1].piece();
            ray_attackers.push((piece, piece_position));
        }
        */
        let ray_attackers = self.ray_attackers(king_location, self.to_move);
        if ray_attackers.len() == 0 {
            return true;
        }

        let mut enemy_moves: Vec<ChessMove> = Vec::new();
        for (piece, position) in ray_attackers {
            generate_pseudo_moves_for_piece(piece, self, position, &mut enemy_moves);
            let king_is_attacked = enemy_moves
                .iter()
                .map(|mov| mov.dest)
                .any(|attacked_square| attacked_square == king_location);
            if king_is_attacked {
                return false;
            }
        }
        true
    }
    // This function will search a lookup table and check if the given piece location is in an x-ray attack by the specified color.
    // Currently used for seeing if the king is in check, but works for any square on the board.
    pub fn ray_attackers(
        &self,
        target_square: ChessCell,
        color: PieceColor,
    ) -> Vec<(Piece, ChessCell)> {
        let target_idx = target_square.as_index();
        let mut ray_attackers: Vec<(Piece, ChessCell)> = Vec::new();
        for attacker in self.get_piece_positions(color) {
            let attacker_idx = attacker.as_index();
            let attacking_square = self.board[attacker.0][attacker.1];
            let attacking_piece = attacking_square.piece();
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
pub fn empty_board() -> [[Square; 12]; 12] {
    let mut board = [[Square::Aether; 12]; 12];
    for rank in RANK_1..=RANK_8 {
        for file in A_FILE..=H_FILE {
            board[rank][file] = Square::Empty;
        }
    }
    board
}
// Returns (white_bitboard, black_bitboard)
pub fn get_bitboards(board: &[[Square; 12]; 12]) -> (BitBoard, BitBoard) {
    let mut white_bitboard: u64 = 0;
    let mut black_bitboard: u64 = 0;
    let mut position: u64 = 0;
    for rank in BOARD_START..=BOARD_END {
        for file in BOARD_START..=BOARD_END {
            if let Square::Full(piece) = board[rank][file] {
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
fn find_kings(board: &[[Square; 12]; 12]) -> Result<(ChessCell, ChessCell), &'static str> {
    let mut white_king_location: Option<ChessCell> = None;
    let mut black_king_location: Option<ChessCell> = None;
    for rank in RANK_1..=RANK_8 {
        for file in A_FILE..=H_FILE {
            let square = board[rank][file];
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
        let white_king_square = board[RANK_1][E_FILE];
        assert_eq!(white_king_square.is_empty_or_enemy_of(Black), true);
        assert_eq!(white_king_square.is_empty_or_enemy_of(White), false);
        let c6 = board[RANK_6][C_FILE];
        assert_eq!(c6.is_empty_or_enemy_of(White), true);
    }
    #[test]
    fn unmake_move_restores_board_state() {
        let mut board_state = BoardState::new_game();
        let mov = ChessMove {
            start: ChessCell(RANK_2, E_FILE),
            dest: ChessCell(RANK_4, E_FILE)
        };
        board_state.make_move(mov);
        board_state.unmake_move();
        assert_eq!(board_state, BoardState::new_game())
    }
}
