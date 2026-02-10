use std::io::{Read, stdin};

use crate::ordi::Player;
use crate::Piece;

pub struct Human {
    buff: [u8; 64]
}

impl Player for Human {
    fn give_piece_to_other_player(&mut self) -> Piece {
        loop {
            let success = stdin().read(&mut self.buff);
            match success {
                Ok(_) => todo!(),
                Err(_) => continue,
            }
        }
        Piece(0)
    }
    
    fn play_piece(&mut self) -> u8 {
        loop {
            let success = stdin().read(&mut self.buff);
            match success {
                Ok(_) => todo!(),
                Err(_) => continue,
            }
        }
        0
    }
}