use crate::Piece;

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
