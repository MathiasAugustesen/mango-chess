use std::str::FromStr;

use crate::constants::*;
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
impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Square::Empty => write!(f, " "),
            Square::Aether => write!(f, "Aether is not meant for printing"),
            Square::Full(piece) => write!(f, "{}", piece),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}
impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match (self.color, self.kind) {
            (White, Pawn) => '♙',
            (White, Knight) => '♘',
            (White, Bishop) => '♗',
            (White, Rook) => '♖',
            (White, Queen) => '♕',
            (White, King) => '♔',
            (Black, Pawn) => '♟',
            (Black, Knight) => '♞',
            (Black, Bishop) => '♝',
            (Black, Rook) => '♜',
            (Black, Queen) => '♛',
            (Black, King) => '♚',
        };
        write!(f, "{symbol}")

    }
}
impl Piece {
    pub fn pawn(color: PieceColor) -> Piece {
        Piece { color, kind: Pawn }
    }
    pub fn knight(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Knight,
        }
    }
    pub fn bishop(color: PieceColor) -> Piece {
        Piece {
            color,
            kind: Bishop,
        }
    }
    pub fn rook(color: PieceColor) -> Piece {
        Piece { color, kind: Rook }
    }
    pub fn queen(color: PieceColor) -> Piece {
        Piece { color, kind: Queen }
    }
    pub fn king(color: PieceColor) -> Piece {
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    pub start: ChessCell,
    pub dest: ChessCell,
}
impl std::fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.dest)
    }
}
impl From<(ChessCell, ChessCell)> for ChessMove {
    fn from(cells: (ChessCell, ChessCell)) -> ChessMove {
        ChessMove {
            start: cells.0,
            dest: cells.1,
        }
    }
}
impl From<CastlingType> for ChessMove {
    fn from(value: CastlingType) -> Self {
        let (start, dest) = match value {
            CastlingType::WhiteKingSide => ((RANK_1, E_FILE), (RANK_1, G_FILE)),
            CastlingType::WhiteQueenSide => ((RANK_1, E_FILE), (RANK_1, C_FILE)),
            CastlingType::BlackKingSide => ((RANK_8, E_FILE), (RANK_8, G_FILE)),
            CastlingType::BlackQueenSide => ((RANK_8, E_FILE), (RANK_8, C_FILE)),
        };
        ChessMove {
            start: ChessCell::from(start),
            dest: ChessCell::from(dest),
        }
    }
}
impl ChessMove {
    pub fn reverse(self) -> ChessMove {
        ChessMove {
            start: self.dest,
            dest: self.start,
        }
    }
}
