use crate::Piece;

pub struct Stack(pub u16);

impl Stack {
    pub fn new() -> Self {
        Stack(0xFFFF)
    }

    pub fn has(&self, piece: Piece) -> bool {
        if piece.0 > 0x0F {
            return false;
        }

        let bit = 1 << piece.0;
        (self.0 & bit) == bit
    }

    pub fn pick(&mut self, piece: Piece) -> bool {
        if !self.has(piece) {
            return false;
        }

        self.0 &= !(1 << piece.0);
        true
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let mut stack = Stack::new();

        assert_eq!(true, stack.has(Piece(0b0010)));
        assert_eq!(true, stack.has(Piece(0b0101)));
        assert_eq!(false, stack.has(Piece(0b11111111)));

        _ = stack.pick(Piece(0b0010));
        assert_eq!(false, stack.has(Piece(0b0010)));
        assert_eq!(true, stack.has(Piece(0b0101)));
        assert_eq!(false, stack.has(Piece(0b11111111)));
    }
}
