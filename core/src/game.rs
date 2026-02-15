use crate::Board;
use crate::Stack;

/// Defines a complete Quarto game state.
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub stack: Stack,
}
