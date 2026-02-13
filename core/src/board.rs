use crate::Piece;

pub struct Board(pub [[Option<Piece>; 4]; 4]);

impl Board {
    pub fn new() -> Self {
        Board([[None; 4]; 4])
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x >= 4 || y >= 4 {
            return None;
        }

        self.0[x][y]
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        if x >= 4 || y >= 4 {
            return;
        }

        self.0[x][y] = piece;
    }
}
