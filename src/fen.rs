use std::str::FromStr;

use crate::board::ChessBoard;
use crate::board::CastlingRights;
use crate::board::ChessCell;
use crate::board::Piece;
use crate::board::PieceColor;
use crate::board::PieceColor::{Black, White};
use crate::board::Square;
use crate::constants::*;
pub fn board_from_fen(fen_board: &str) -> Result<ChessBoard, &str> {
    let mut board = ChessBoard::empty();
    let fen_ranks: Vec<&str> = fen_board.split("/").collect();

    if fen_ranks.len() != 8 {
        return Err("Failed to parse FEN string: piece placements did not contain 8 ranks.");
    }
    let mut board_rank = RANK_1;
    let mut board_file = A_FILE;
    for fen_rank in fen_ranks.iter().rev() {
        for square in fen_rank.chars() {
            if square.is_digit(10) {
                let skipped_squares = square.to_digit(10).unwrap() as usize;
                if board_file + skipped_squares > 10 {
                    return Err("Failed to parse FEN string: Rank went out of bounds");
                }
                board_file += skipped_squares;
            } else {
                let piece = Piece::try_from(square);
                if piece.is_err() {
                    return Err("Failed to parse FEN string: Character was not a chess piece");
                }
                let piece = piece.unwrap();
                *board.square_mut(ChessCell(board_rank, board_file)) = Square::Full(piece);
                board_file += 1;
            }
        }
        if board_file != BOARD_END + 1 {
            return Err("Failed to parse FEN string: Rank did not have length 8");
        }
        board_rank += 1;
        board_file = BOARD_START;
    }
    Ok(board)
}
pub fn castling_rights_from_fen(fen_castling_rights: &str) -> Result<CastlingRights, &'static str> {
    if fen_castling_rights.len() > 4 {
        return Err("Failed to parse FEN string: Castling rights string was too long");
    }
    if !fen_castling_rights
        .chars()
        .all(|c| matches!(c, 'q' | 'k' | 'Q' | 'K' | '-'))
    {
        return Err("Failed to parse FEN string: Castling rights contained invalid characters");
    }
    Ok(CastlingRights {
        white_king_side_castling: fen_castling_rights.contains("K"),
        white_queen_side_castling: fen_castling_rights.contains("Q"),
        black_king_side_castling: fen_castling_rights.contains("k"),
        black_queen_side_castling: fen_castling_rights.contains("q"),
    })
}
pub fn to_move_from_fen(fen_to_move: &str) -> Result<PieceColor, &'static str> {
    match fen_to_move {
        "w" => Ok(White),
        "b" => Ok(Black),
        _ => return Err("Failed to parse FEN string: player to move was not black or white"),
    }
}
pub fn en_passant_square_from_fen(
    fen_en_passant_square: &str,
) -> Result<Option<ChessCell>, &'static str> {
    if fen_en_passant_square == "-" {
        return Ok(None);
    } else if ChessCell::from_str(fen_en_passant_square).is_ok() {
        Ok(Some(ChessCell::from_str(fen_en_passant_square).unwrap()))
    } else {
        return Err("Failed to parse FEN string: En passant value was not valid FEN");
    }
}
