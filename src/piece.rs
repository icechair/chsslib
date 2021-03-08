use std::fmt;
pub enum Piece {
    Empty,
    Pawn(bool),
    Rook(bool),
    Knight(bool),
    Bishop(bool),
    Queen(bool),
    King(bool),
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Empty => write!(f, "."),
            Piece::Pawn(true) => write!(f, "P"),
            Piece::Pawn(false) => write!(f, "p"),
            Piece::Rook(true) => write!(f, "R"),
            Piece::Rook(false) => write!(f, "r"),
            Piece::Knight(true) => write!(f, "N"),
            Piece::Knight(false) => write!(f, "n"),
            Piece::Bishop(true) => write!(f, "B"),
            Piece::Bishop(false) => write!(f, "b"),
            Piece::Queen(true) => write!(f, "Q"),
            Piece::Queen(false) => write!(f, "q"),
            Piece::King(true) => write!(f, "K"),
            Piece::King(false) => write!(f, "k"),
        }
    }
}
