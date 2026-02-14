use quarto_core::{Board, Game, Stack};
use quarto_players::Player;

pub enum Outcome {
    Win(usize),
    Draw,
    Illegal(usize),
}

pub fn game_loop(players: &mut [Box<dyn Player>; 2]) -> Outcome {
    let player_count = players.len();

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    loop {
        for i in 0..player_count {
            let current_player = players.get_mut(i).unwrap();
            let piece_idx = current_player.give_piece(&game);

            if !game.stack.0.iter().any(|p| p.is_some()) {
                return Outcome::Draw;
            }

            let piece = match game.stack.pick(piece_idx) {
                Some(piece) => piece,
                None => return Outcome::Illegal(i),
            };

            let next_idx = (i + 1) % player_count;
            let next_player = players.get_mut(next_idx).unwrap();
            let (x, y) = next_player.play_piece(&game, piece);

            if game.board.get_piece(x, y).is_some() {
                return Outcome::Illegal(next_idx);
            }
            game.board.set_piece(x, y, Some(piece));

            if game.board.is_win(x, y) {
                return Outcome::Win(next_idx);
            }
        }
    }
}

pub fn main() {
    println!("hello, world!");

    let stack = Stack::new();

    for piece in stack.0 {
        let piece = piece.unwrap();
        println!(
            "{}: bright = {}, square = {}, tall = {}, hollow = {}",
            piece,
            piece.is_bright(),
            piece.is_square(),
            piece.is_tall(),
            piece.is_hollow()
        );
    }
}
