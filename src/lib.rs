pub mod bitboard;
pub use bitboard::Bitboard;
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

pub struct Board {
    whites: Bitboard,
    blacks: Bitboard,
    pawns: Bitboard,
    rooks: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            whites: Bitboard::new(0),
            blacks: Bitboard::new(0),
            pawns: Bitboard::new(0),
            rooks: Bitboard::new(0),
            knights: Bitboard::new(0),
            bishops: Bitboard::new(0),
            queens: Bitboard::new(0),
            kings: Bitboard::new(0),
        }
    }

    pub fn get_piece(&self, rank: u64, file: u64) -> Piece {
        let square = bitboard::id(rank, file);
        if self.whites.bit(square) > 0 {
            match square {
                _ if self.pawns.bit(square) > 0 => return Piece::Pawn(true),
                _ if self.rooks.bit(square) > 0 => return Piece::Rook(true),
                _ if self.knights.bit(square) > 0 => return Piece::Knight(true),
                _ if self.bishops.bit(square) > 0 => return Piece::Bishop(true),
                _ if self.queens.bit(square) > 0 => return Piece::Queen(true),
                _ if self.kings.bit(square) > 0 => return Piece::King(true),
                _ => (),
            }
        } else if self.blacks.bit(square) > 0 {
            match square {
                _ if self.pawns.bit(square) > 0 => return Piece::Pawn(false),
                _ if self.rooks.bit(square) > 0 => return Piece::Rook(false),
                _ if self.knights.bit(square) > 0 => return Piece::Knight(false),
                _ if self.bishops.bit(square) > 0 => return Piece::Bishop(false),
                _ if self.queens.bit(square) > 0 => return Piece::Queen(false),
                _ if self.kings.bit(square) > 0 => return Piece::King(false),
                _ => (),
            }
        }
        Piece::Empty
    }

    pub fn parse(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.trim().split(" ").collect();
        if parts.len() != 6 {
            return Err("Not Enough Parts".to_string());
        }
        let placements: Vec<&str> = parts[0].split("/").collect();
        if placements.len() != 8 {
            return Err("Not Enough Placements".to_string());
        }

        let mut board = Self::new();

        for (rank, line) in placements.iter().enumerate() {
            let mut file = 0;
            for c in line.chars() {
                let square = bitboard::id(7 - rank as u64, file as u64);
                if let Some(d) = c.to_digit(10) {
                    file += d;
                    continue;
                }
                match c {
                    'p' => {
                        board.pawns.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'P' => {
                        board.pawns.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    'r' => {
                        board.rooks.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'R' => {
                        board.rooks.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    'n' => {
                        board.knights.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'N' => {
                        board.knights.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    'b' => {
                        board.bishops.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'B' => {
                        board.bishops.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    'q' => {
                        board.queens.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'Q' => {
                        board.queens.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    'k' => {
                        board.kings.toggle_bit(square);
                        board.blacks.toggle_bit(square);
                        file += 1;
                    }
                    'K' => {
                        board.kings.toggle_bit(square);
                        board.whites.toggle_bit(square);
                        file += 1;
                    }
                    _ => return Err(format!("wrong character in placement: {}", c)),
                }
            }
        }

        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board\n")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let piece = match self.get_piece(rank, file) {
                    Piece::Empty => '.',
                    Piece::Pawn(true) => 'P',
                    Piece::Pawn(false) => 'p',
                    Piece::Rook(true) => 'R',
                    Piece::Rook(false) => 'r',
                    Piece::Knight(true) => 'N',
                    Piece::Knight(false) => 'n',
                    Piece::Bishop(true) => 'B',
                    Piece::Bishop(false) => 'b',
                    Piece::Queen(true) => 'Q',
                    Piece::Queen(false) => 'q',
                    Piece::King(true) => 'K',
                    Piece::King(false) => 'k',
                };
                write!(f, "{} ", piece)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h")?;
        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board\n")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let piece = match self.get_piece(rank, file) {
                    Piece::Empty => '.',
                    Piece::Pawn(true) => 'P',
                    Piece::Pawn(false) => 'p',
                    Piece::Rook(true) => 'R',
                    Piece::Rook(false) => 'r',
                    Piece::Knight(true) => 'N',
                    Piece::Knight(false) => 'n',
                    Piece::Bishop(true) => 'B',
                    Piece::Bishop(false) => 'b',
                    Piece::Queen(true) => 'Q',
                    Piece::Queen(false) => 'q',
                    Piece::King(true) => 'K',
                    Piece::King(false) => 'k',
                };
                write!(f, "{} ", piece)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h\n")?;
        writeln!(f, "whites: {}", self.whites)?;
        writeln!(f, "blacks: {}", self.blacks)?;
        writeln!(f, "pawns: {}", self.pawns)?;
        writeln!(f, "rooks: {}", self.rooks)?;
        writeln!(f, "knights: {}", self.knights)?;
        writeln!(f, "bishops: {}", self.bishops)?;
        writeln!(f, "queens: {}", self.queens)?;
        writeln!(f, "kings: {}", self.kings)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let board = Board::parse(&start);
        match board {
            Ok(board) => {
                assert_eq!(board.whites, Bitboard::new(65535));
                assert_eq!(board.blacks, Bitboard::new(18446462598732840960));
                assert_eq!(board.pawns, Bitboard::new(71776119061282560));
                assert_eq!(board.rooks, Bitboard::new(9295429630892703873));
                assert_eq!(board.knights, Bitboard::new(4755801206503243842));
                assert_eq!(board.bishops, Bitboard::new(2594073385365405732));
                assert_eq!(board.queens, Bitboard::new(576460752303423496));
                assert_eq!(board.kings, Bitboard::new(1152921504606846992));
            }
            Err(_) => assert!(false),
        }
    }
}
