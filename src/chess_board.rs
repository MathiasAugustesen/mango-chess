use crate::board_elements::ChessCell;
use crate::board_elements::Piece;
use crate::board_elements::PieceColor::*;
use crate::board_elements::Square;
use crate::constants::*;

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
