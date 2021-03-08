use std::fmt;
use std::ops;

#[rustfmt::skip]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

pub fn id(rank: u64, file: u64) -> u64 {
    rank * 8 + file
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new(v: u64) -> Self {
        Self(v)
    }
    pub fn bit(&self, n: u64) -> u64 {
        (self.0 >> n) & 1
    }
    pub fn toggle_bit(&mut self, n: u64) {
        self.0 ^= 1 << n
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitboard({})\n", self.0)?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                write!(f, "{} ", self.bit(id(rank, file)))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h")?;
        Ok(())
    }
}
impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitboard({})\n", self.0)?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                write!(f, "{} ", self.bit(id(rank, file)))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h")?;
        Ok(())
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        return Self(self.0 & rhs.0);
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        return Self(self.0 | rhs.0);
    }
}

impl ops::BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        return Self(self.0 ^ rhs.0);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_bit() {
        let a = Bitboard::new(3);
        assert_eq!(a.bit(0), 1);
        assert_eq!(a.bit(1), 1);
        assert_eq!(a.bit(2), 0);
        let a = Bitboard::new(4);
        assert_eq!(a.bit(0), 0);
        assert_eq!(a.bit(1), 0);
        assert_eq!(a.bit(2), 1);

        let mut a = Bitboard::new(4);
        a.toggle_bit(2);
        assert_eq!(a.0, 0);
        a.toggle_bit(2);
        a.toggle_bit(3);
        assert_eq!(a.0, 4 + 8);
    }

    #[test]
    fn test_bitboard_operations() {
        let a = Bitboard(1);
        let b = Bitboard(3);
        assert_eq!(a & b, Bitboard(1));
        assert_eq!(a | b, Bitboard(3));
        assert_eq!(a ^ b, Bitboard(2));
    }
}
