#[derive(Debug, Clone, Copy)]
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
        } else {
            if self.is_bright() { "●" } else { "○" }
        })?;

        Ok(())
    }
}

pub struct Board(pub [[Option<Piece>; 4]; 4]);

impl Board {
    pub fn new() -> Self {
        Board([[None; 4]; 4])
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        self.0[x][y]
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        self.0[x][y] = piece;
    }
}

pub struct Stack(pub [Option<Piece>; 16]);

impl Stack {
    pub fn new() -> Self {
        let mut arr = [None; 16];

        for i in 0..16 {
            arr[i] = Some((i as u8).into());
        }

        Stack(arr)
    }

    pub fn pick(&mut self, i: usize) -> Option<Piece> {
        if i > 16 {
            return None;
        }

        let piece = match self.0[i] {
            Some(piece) => piece,
            None => {
                return None;
            }
        };

        self.0[i] = None;
        Some(piece)
    }
}
