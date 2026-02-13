#[derive(Debug, Clone, Copy)]
pub struct Piece(u8);

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
