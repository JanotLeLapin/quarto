use std::io::stdin;

use quarto_core::{Game, Piece};

use crate::Player;

pub struct Human {
    buff: String,
}

impl Player for Human {
    fn give_piece(&mut self, game: &Game) -> usize {
        println!("Type the piece you wish to give your opponent.");
        loop {
            self.buff.clear();
            let success = stdin().read_line(&mut self.buff);
            match success {
                Ok(_) => {
                    if self.buff == "help" {
                        println!(
                            "Type the piece you wish to give your opponent, represented by a 4 character long input.\nFormat :\nt/s (tall / small)\nb/w (black / white)\nh/f (hollow / full)\ns/c (square / circle)\n\nExample : tbhs, wsfs.\n"
                        );
                        continue;
                    }

                    let array = self.buff.as_bytes();

                    let tall = match array[0] {
                        b't' => true,
                        b's' => false,
                        other => {
                            println!(
                                "Invalid height input \"{}\". Please try again.",
                                other as char
                            );
                            continue;
                        }
                    };

                    let bright = match array[1] {
                        b'w' => true,
                        b'b' => false,
                        other => {
                            println!(
                                "Invalid color input \"{}\". Please try again.",
                                other as char
                            );
                            continue;
                        }
                    };

                    let hollow = match array[2] {
                        b'h' => true,
                        b'f' => false,
                        other => {
                            println!(
                                "Invalid hollowness input \"{}\". Please try again.",
                                other as char
                            );
                            continue;
                        }
                    };

                    let square = match array[3] {
                        b's' => true,
                        b'c' => false,
                        other => {
                            println!(
                                "Invalid shape input \"{}\". Please try again.",
                                other as char
                            );
                            continue;
                        }
                    };

                    let piece = Piece::new(bright, square, tall, hollow);
                    let piece_idx = match game
                        .stack
                        .0
                        .iter()
                        .enumerate()
                        .find(|(_, p)| Some(piece) == **p)
                        .map(|(i, _)| i)
                    {
                        Some(i) => i,
                        None => {
                            println!("Piece \"{piece}\" is missing from the stack.");
                            continue;
                        }
                    };

                    return piece_idx;
                }
                Err(_) => {
                    println!("Error parsing input. Please try again.");
                    continue;
                }
            }
        }
    }

    fn play_piece(&mut self, game: &Game, given_piece: Piece) -> (usize, usize) {
        println!("Type the coordinates you wish to place your piece ({given_piece}) in.");
        loop {
            self.buff.clear();
            let success = stdin().read_line(&mut self.buff);
            match success {
                Ok(_) => {
                    if self.buff == "help" {
                        println!("Coordinates are in the x,y or x, y format.\n");
                        continue;
                    }

                    let x_res = self.buff[..=1].parse::<usize>();
                    let y_res = self.buff[2..].parse::<usize>();

                    match x_res {
                        Ok(x) => match y_res {
                            Ok(y) => {
                                if x >= 4 || y >= 4 {
                                    println!(
                                        "Coordinates ({x}, {y}) are out of bounds. Please try again."
                                    );
                                    continue;
                                }

                                let piece_at_pos = game.board.get_piece(x, y);
                                if let Some(piece_at_pos) = piece_at_pos {
                                    println!(
                                        "Coordinates ({x}, {y}) are already used by piece {}. Please try again.",
                                        piece_at_pos
                                    );
                                    continue;
                                }

                                return (x, y);
                            }
                            Err(_) => {
                                println!("Error parsing y. Please try again.");
                                continue;
                            }
                        },
                        Err(_) => {
                            println!("Error parsing x. Please try again.");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Error parsing input. Please try again.");
                    continue;
                }
            }
        }
    }
}
