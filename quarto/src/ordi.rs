use crate::Piece;

pub trait Player {
    fn give_piece_to_other_player(&mut self) -> Piece;
    fn play_piece(&mut self) -> u8;
}