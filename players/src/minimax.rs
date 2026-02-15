use quarto_core::{Game, Piece};

use crate::Player;

pub enum DecisionTreeAction {
    Pick(Piece),
    Play((u8, u8)),
}

pub struct DecisionTreeNode {
    game: Game,
    depth: u8,
    score: f32,
    action: DecisionTreeAction,
    children: Vec<Box<DecisionTreeNode>>,
}

pub fn explore(node: &mut DecisionTreeNode, my_turn: bool, max_depth: u8) {
    if node.depth >= max_depth {
        node.score = 0.0;
        return;
    }

    match node.action {
        DecisionTreeAction::Pick(p) => {
            for y in 0..4 {
                for x in 0..4 {
                    if node.game.board.get_piece(x, y).is_some() {
                        continue;
                    }

                    let mut child_game = node.game.clone();
                    child_game.board.set_piece(x, y, Some(p));

                    let mut child = DecisionTreeNode {
                        game: child_game,
                        depth: node.depth + 1,
                        score: if my_turn {
                            f32::NEG_INFINITY
                        } else {
                            f32::INFINITY
                        },
                        action: DecisionTreeAction::Play((x as u8, y as u8)),
                        children: Vec::with_capacity(16 - node.depth as usize),
                    };

                    if child.game.board.is_win(x, y) {
                        child.score = if my_turn { 100.0 } else { -100.0 };
                    } else {
                        explore(&mut child, my_turn, max_depth);
                    }

                    node.score = if my_turn {
                        node.score.max(child.score)
                    } else {
                        node.score.min(child.score)
                    };
                    node.children.push(Box::new(child));
                }
            }
        }
        DecisionTreeAction::Play(_) => {
            for p in (0..16).map(|i| Piece(i as u8)) {
                if !node.game.stack.has(p) {
                    continue;
                }

                let mut child_game = node.game.clone();
                child_game.stack.pick(p);

                let mut child = DecisionTreeNode {
                    game: child_game,
                    depth: node.depth + 1,
                    score: if my_turn {
                        f32::INFINITY
                    } else {
                        f32::NEG_INFINITY
                    },
                    action: DecisionTreeAction::Pick(p),
                    children: Vec::with_capacity(16 - node.depth as usize),
                };

                explore(&mut child, !my_turn, max_depth);

                node.score = if my_turn {
                    node.score.max(child.score)
                } else {
                    node.score.min(child.score)
                };
                node.children.push(Box::new(child));
            }
        }
    }
}

pub struct MinimaxBot {
    max_depth: u8,
}

impl MinimaxBot {
    pub fn new(max_depth: u8) -> Self {
        Self { max_depth }
    }
}

impl Player for MinimaxBot {
    fn give_piece(&mut self, game: &Game) -> Piece {
        let mut tree = DecisionTreeNode {
            game: game.clone(),
            depth: 0,
            score: f32::NEG_INFINITY,
            action: DecisionTreeAction::Play((0, 0)),
            children: Vec::with_capacity(game.stack.0.count_ones() as usize),
        };

        explore(
            &mut tree,
            true,
            game.stack.0.count_zeros().max(4).min(self.max_depth as u32) as u8,
        );

        match tree
            .children
            .iter()
            .reduce(|a, b| if a.score > b.score { a } else { b })
        {
            Some(node) => match node.action {
                DecisionTreeAction::Pick(p) => p,
                DecisionTreeAction::Play(_) => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    fn play_piece(&mut self, game: &Game, given_piece: Piece) -> (usize, usize) {
        let mut tree = DecisionTreeNode {
            game: game.clone(),
            depth: 0,
            score: f32::NEG_INFINITY,
            action: DecisionTreeAction::Pick(given_piece),
            children: Vec::with_capacity(game.stack.0.count_ones() as usize),
        };

        explore(
            &mut tree,
            true,
            game.stack.0.count_zeros().max(4).min(self.max_depth as u32) as u8,
        );

        match tree
            .children
            .iter()
            .reduce(|a, b| if a.score > b.score { a } else { b })
        {
            Some(node) => match node.action {
                DecisionTreeAction::Pick(_) => unreachable!(),
                DecisionTreeAction::Play((x, y)) => (x as usize, y as usize),
            },
            None => unreachable!(),
        }
    }
}

impl Default for MinimaxBot {
    fn default() -> Self {
        Self::new(10)
    }
}
