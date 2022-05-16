use crate::logic::{Board, Piece};

pub fn blank_board() -> Board {
    [
        [Piece::Blank, Piece::Blank, Piece::Blank],
        [Piece::Blank, Piece::Blank, Piece::Blank],
        [Piece::Blank, Piece::Blank, Piece::Blank],
    ]
}
