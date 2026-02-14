#[derive(Debug, Clone, Copy, PartialEq)]

/// A piece is an combination of four binary values : color, shape, height and hollowness.
/// A piece may be "empty".
pub struct Piece(pub u8);

impl Piece {
    pub fn new(bright: bool, square: bool, tall: bool, hollow: bool) -> Self {
        let mut field = 0;
        field |= if bright { 0x01 } else { 0x00 };
        field |= if square { 0x02 } else { 0x00 };
        field |= if tall { 0x04 } else { 0x00 };
        field |= if hollow { 0x08 } else { 0x00 };

        Self(field)
    }

    pub fn is_bright(&self) -> bool {
        (self.0 & 0x01) == 0x01
    }

    pub fn is_square(&self) -> bool {
        (self.0 & 0x02) == 0x02
    }

    pub fn is_tall(&self) -> bool {
        (self.0 & 0x04) == 0x04
    }

    pub fn is_hollow(&self) -> bool {
        (self.0 & 0x08) == 0x08
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.is_tall() { "+" } else { "-" })?;
        f.write_str(if self.is_hollow() { "h" } else { "f" })?;
        f.write_str(if self.is_square() {
            if self.is_bright() { "■" } else { "□" }
        } else if self.is_bright() {
            "●"
        } else {
            "○"
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Piece::from(0b0000), Piece::new(false, false, false, false));
        assert_eq!(Piece::from(0b1111), Piece::new(true, true, true, true));
        assert_eq!(Piece::from(0b1100), Piece::new(false, false, true, true));
        assert_eq!(Piece::from(0b1011), Piece::new(true, true, false, true));
    }
}
