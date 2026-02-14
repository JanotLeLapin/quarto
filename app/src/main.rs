use quarto_core::{Board, Game, Piece, Stack};
use quarto_players::{Player, random::RandomBot};

const PLAYER_COUNT: usize = 2;

pub enum Outcome {
    Win(usize),
    Draw,
    Illegal(usize),
}

pub fn game_iter(
    game: &mut Game,
    player_idx: usize,
    players: &mut [Box<dyn Player>; PLAYER_COUNT],
) -> Option<Outcome> {
    let current_player = players.get_mut(player_idx).unwrap();
    let piece = current_player.give_piece(game);

    if game.stack.0 == 0 {
        return Some(Outcome::Draw);
    }

    if !game.stack.pick(piece) {
        return Some(Outcome::Illegal(player_idx));
    }

    let next_idx = (player_idx + 1) % PLAYER_COUNT;
    let next_player = players.get_mut(next_idx).unwrap();
    let (x, y) = next_player.play_piece(game, piece);

    if game.board.get_piece(x, y).is_some() {
        return Some(Outcome::Illegal(next_idx));
    }
    game.board.set_piece(x, y, Some(piece));

    if game.board.is_win(x, y) {
        return Some(Outcome::Win(next_idx));
    }

    None
}

pub fn game_loop(players: &mut [Box<dyn Player>; 2]) -> Outcome {
    let player_count = players.len();

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    loop {
        for i in 0..player_count {
            if let Some(outcome) = game_iter(&mut game, i, players) {
                return outcome;
            }
        }
    }
}

pub fn main() {
    println!("hello, world!");

    for piece in 0..16 {
        let piece = Piece(piece as u8);
        println!(
            "{}: bright = {}, square = {}, tall = {}, hollow = {}",
            piece,
            piece.is_bright(),
            piece.is_square(),
            piece.is_tall(),
            piece.is_hollow()
        );
    }

    let p1 = RandomBot::default();
    let p2 = RandomBot::default();

    let mut players: [Box<dyn Player>; 2] = [Box::new(p1), Box::new(p2)];

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    let outcome = 'outer: loop {
        for i in 0..PLAYER_COUNT {
            println!("{}", game.board);
            println!("player {i}'s pick");
            if let Some(outcome) = game_iter(&mut game, i, &mut players) {
                break 'outer outcome;
            }
        }
    };

    match outcome {
        Outcome::Win(i) => println!("player {i} won"),
        Outcome::Draw => println!("draw"),
        Outcome::Illegal(i) => println!("player {i} attempted illegal move"),
    }

    println!("{}", game.board);
}
