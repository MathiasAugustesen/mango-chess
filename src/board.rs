use crate::constants::*;
use crate::fen;
use crate::fen::castling_rights_from_fen;
use crate::fen::en_passant_square_from_fen;
use crate::move_generation::generate_pseudo_moves_for_piece;
use crate::ray_attacks::*;
use std::str::FromStr;
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
            Square::Empty => panic!("Expected piece to be present at square but square was Empty"),
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
}
impl TryFrom<char> for Piece {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece = match value {
            'P' => Piece::pawn(White),
            'N' => Piece::knight(White),
            'B' => Piece::bishop(White),
            'R' => Piece::rook(White),
            'Q' => Piece::queen(White),
            'K' => Piece::king(White),
            'p' => Piece::pawn(Black),
            'n' => Piece::knight(Black),
            'b' => Piece::bishop(Black),
            'r' => Piece::rook(Black),
            'q' => Piece::queen(Black),
            'k' => Piece::king(Black),
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
#[derive(Clone, PartialEq, Debug)]
pub struct BoardState {
    pub board: [[Square; 12]; 12],
    pub to_move: PieceColor,
    // Keeps track of all the white pieces
    pub white_bitboard: u64,
    // Keeps track of all the black pieces
    pub black_bitboard: u64,
    pub white_king_location: ChessCell,
    pub black_king_location: ChessCell,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<ChessCell>,
    pub last_move: Option<(ChessCell, ChessCell)>,
    pub pawn_promotion: Option<ChessCell>,
}
impl BoardState {
    pub fn empty_game() -> BoardState {
        let board = empty_board();
        let to_move = White;
        let white_bitboard: u64 = 0;
        let black_bitboard: u64 = 0;
        let white_king_location = ChessCell(100, 100);
        let black_king_location = ChessCell(100, 100);
        let castling_rights = CastlingRights::no_castling_rights();
        let en_passant_square = None;
        let last_move = None;
        let pawn_promotion = None;
        return BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            castling_rights,
            en_passant_square,
            last_move,
            pawn_promotion,
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
        let castling_rights = CastlingRights::all_castling_rights();
        let en_passant_square = None;
        let last_move = None;
        let pawn_promotion = None;
        BoardState {
            board,
            to_move,
            white_bitboard,
            black_bitboard,
            white_king_location,
            black_king_location,
            castling_rights,
            en_passant_square,
            last_move,
            pawn_promotion,
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
        let castling_rights = castling_rights_from_fen(fen_castling_rights)?;
        let fen_en_passant_square = fen_parts[3];
        let en_passant_square = en_passant_square_from_fen(fen_en_passant_square)?;
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
            castling_rights,
            en_passant_square,
            last_move: None,
            pawn_promotion: None,
        };
        Ok(board_state)
    }
    pub fn swap_to_move(&mut self) {
        match self.to_move {
            White => self.to_move = Black,
            Black => self.to_move = White,
        }
    }
    pub fn clear_en_passant_square(&mut self) {
        self.en_passant_square = None;
    }
    pub fn move_piece(&mut self, position: ChessCell, destination: ChessCell) {
        if let Square::Full(piece) = self.board[position.0][position.1] {
            self.board[position.0][position.1] = Square::Empty;
            self.board[destination.0][destination.1] = Square::Full(piece);
            match piece.color() {
                White => {
                    if piece.kind() == King {
                        self.white_king_location = destination;
                    }
                    // Flips the previous position, removing it.
                    self.white_bitboard ^= 1 << position.as_index();
                    // Logical OR adds the new position to the bitboard
                    self.white_bitboard |= 1 << destination.as_index();
                }
                Black => {
                    if piece.kind() == King {
                        self.black_king_location = destination;
                    }
                    self.black_bitboard ^= 1 << position.as_index();
                    self.black_bitboard |= 1 << destination.as_index();
                }
            }
        }
    }
    pub fn get_piece_positions(&self, color: PieceColor) -> Vec<ChessCell> {
        let mut piece_positions = Vec::new();
        let mut bitboard = match color {
            White => self.white_bitboard,
            Black => self.black_bitboard,
        };
        while bitboard != 0 {
            let position = bitboard.trailing_zeros();
            let cell = ChessCell::from_index(position as usize);
            piece_positions.push(cell);
            bitboard ^= 1 << position;
        }
        piece_positions
    }
    // Given an arbitrary position, determine if the position is legal given that the last move was played by self.to_move.
    // This method does not make any assumptions about how the move was made
    //
    // First, a list of all enemy pieces that x-ray the king is generated with a lookup table. If this list is empty, the king is safe.
    // afterwards,
    pub fn is_valid_move(&self) -> bool {
        let king_location = match self.to_move {
            White => self.white_king_location,
            Black => self.black_king_location,
        };
        let ray_attackers = self.ray_attackers(king_location.as_index());
        if ray_attackers.len() == 0 {
            return true;
        }
        let mut enemy_moves: Vec<(ChessCell, ChessCell)> = Vec::new();
        for (piece, position) in ray_attackers {
            generate_pseudo_moves_for_piece(piece, self, position, &mut enemy_moves);
            let king_is_attacked = enemy_moves
                .iter()
                .map(|mov| mov.1)
                .any(|attacked_square| attacked_square == king_location);
            if king_is_attacked {
                return false;
            }
        }
        true
    }
    // This function will search a lookup table and check if the piece is in an x-ray attack.
    // Primarily used for seeing if the king is in check, but works for any square on the board.
    pub fn ray_attackers(&self, piece_location_index: usize) -> Vec<(Piece, ChessCell)> {
        let mut ray_attackers: Vec<(Piece, ChessCell)> = Vec::new();
        let enemy_color = self.to_move.opposite();
        for piece_position in self.get_piece_positions(enemy_color) {
            let attacking_piece = self.board[piece_position.0][piece_position.1].piece();
            let attacked_squares = match (attacking_piece.color(), attacking_piece.kind()) {
                (White, Pawn) => WHITE_PAWN_RAY_ATTACKS[piece_position.as_index()],
                (Black, Pawn) => BLACK_PAWN_RAY_ATTACKS[piece_position.as_index()],
                (_, Knight) => KNIGHT_RAY_ATTACKS[piece_position.as_index()],
                (_, Bishop) => BISHOP_RAY_ATTACKS[piece_position.as_index()],
                (_, Rook) => ROOK_RAY_ATTACKS[piece_position.as_index()],
                (_, Queen) => QUEEN_RAY_ATTACKS[piece_position.as_index()],
                (_, King) => KING_RAY_ATTACKS[piece_position.as_index()],
            };
            if attacked_squares.contains(&piece_location_index) {
                ray_attackers.push((attacking_piece, piece_position));
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
pub fn get_bitboards(board: &[[Square; 12]; 12]) -> (u64, u64) {
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
    (white_bitboard, black_bitboard)
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
        assert_eq!(
            fen_board_state.castling_rights,
            new_board_state.castling_rights
        );
        assert_eq!(
            fen_board_state.en_passant_square,
            new_board_state.en_passant_square
        );
        assert_eq!(fen_board_state.last_move, new_board_state.last_move);
        assert_eq!(
            fen_board_state.pawn_promotion,
            new_board_state.pawn_promotion
        );
    }
    #[test]
    #[should_panic]
    fn create_board_from_fen_with_invalid_length_panics() {
        let board_state: BoardState =
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR KQkq e3 0 1")
                .unwrap();
    }
    #[test]
    fn get_bitboards_returns_the_right_bitboards() {
        let board_state = BoardState::new_game();
        let (white_bitboard, black_bitboard) = get_bitboards(&board_state.board);
        assert_eq!(white_bitboard, 0xFFFF);
        assert_eq!(black_bitboard, 0xFFFF << 48);
    }
    #[test]
    fn get_piece_positions_for_starting_position() {
        let board_state = BoardState::new_game();
        let (white_bitboard, black_bitboard) = get_bitboards(&board_state.board);
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
        let (white_bitboard, black_bitboard) = get_bitboards(&board_state.board);
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
}
