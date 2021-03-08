pub mod bitboard;
pub mod piece;
pub use bitboard::Bitboard;
pub use bitboard::Square;
pub use piece::Piece;
use std::fmt;

pub struct Castling(u8);
impl Castling {
    pub fn new(v: u8) -> Self {
        return Self(v);
    }
    pub fn can_white_king(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn set_white_king(&mut self) {
        self.0 |= 1
    }
    pub fn can_white_queen(&self) -> bool {
        self.0 & 2 == 2
    }
    pub fn set_white_queen(&mut self) {
        self.0 |= 2
    }
    pub fn can_black_king(&self) -> bool {
        self.0 & 4 == 4
    }
    pub fn set_black_king(&mut self) {
        self.0 |= 4
    }
    pub fn can_black_queen(&self) -> bool {
        self.0 & 8 == 8
    }
    pub fn set_black_queen(&mut self) {
        self.0 |= 8
    }
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
    white_active: bool,
    castling: Castling,
    en_passant: Bitboard,
    halfmove: u64,
    fullmove: u64,
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
            white_active: true,
            castling: Castling::new(0),
            en_passant: Bitboard::new(0),
            halfmove: 0,
            fullmove: 1,
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

        match parts[1] {
            "w" => board.white_active = true,
            "b" => board.white_active = false,
            _ => {
                return Err(format!("wrong character in active: {}", parts[1]));
            }
        }

        if parts[2].contains("k") {
            board.castling.set_black_king()
        }
        if parts[2].contains("K") {
            board.castling.set_white_king()
        }
        if parts[2].contains("q") {
            board.castling.set_black_queen()
        }
        if parts[2].contains("Q") {
            board.castling.set_white_queen()
        }

        match parts[3] {
            "-" => (),
            sq => {
                let square = Square::parse(sq);
                match square {
                    Ok(s) => board.en_passant.toggle_bit(s),
                    Err(e) => {
                        return Err(format!("invalid en_passant part: {}, err({})", parts[3], e))
                    }
                }
            }
        }
        if let Ok(halfmove) = parts[4].parse::<u64>() {
            board.halfmove = halfmove;
        } else {
            return Err(format!("invald halfmove: {}", parts[4]));
        }
        if let Ok(fullmove) = parts[5].parse::<u64>() {
            board.fullmove = fullmove;
        } else {
            return Err(format!("invald fullmove: {}", parts[5]));
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
                let piece = self.get_piece(rank, file);
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
                let piece = self.get_piece(rank, file);
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
                assert_eq!(board.kings, Bitboard::new(576460752303423496));
            }
            Err(_) => assert!(false),
        }
    }
}
