use std::collections::HashMap;

use rand::prelude::SliceRandom;
use Position::{X, Y, Z};

#[derive(Debug)]
struct GameStats {
    player: Player,
    wins: i32,
    losses: i32,
    draws: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    Ex,
    Oh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Position {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
struct Game {
    board: HashMap<(Position, Position), Player>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum GameResult {
    Win(Player),
    Draw,
}

impl Player {
    pub fn toggle(self) -> Player {
        match self {
            Player::Ex => Player::Oh,
            Player::Oh => Player::Ex,
        }
    }
}

type Tile = (Position, Position);

impl Game {
    const ALL_TILES: [Tile; 9] = [
        (X, X),
        (X, Y),
        (X, Z),
        (Y, X),
        (Y, Y),
        (Y, Z),
        (Z, X),
        (Z, Y),
        (Z, Z),
    ];

    const WINNING_LINES: [[Tile; 3]; 8] = [
        [(X, X), (X, Y), (X, Z)], // h1
        [(X, X), (Y, X), (Z, X)], // v1
        [(X, X), (Y, Y), (Z, Z)], // d1
        [(X, Y), (Y, Y), (Z, Y)], // v2
        [(X, Z), (Y, Z), (Z, Z)], // v3
        [(Y, X), (Y, Y), (Y, Z)], // h2
        [(Z, X), (Y, Y), (X, Z)], // d3
        [(Z, X), (Z, Y), (Z, Z)], // h3
    ];

    fn new() -> Game {
        return Game {
            board: HashMap::new(),
        };
    }

    fn get_empty_tiles(&self) -> Vec<(Position, Position)> {
        Game::ALL_TILES
            .iter()
            .filter(|tile| !self.board.contains_key(&tile))
            .map(|x| *x)
            .collect()
    }

    // TODO: impl as display trait
    #[allow(dead_code)]
    fn print(self, game_num: i32) {
        println!("game num {}", game_num);
        for row in self.board {
            println!("{:?}", row);
        }
        println!("--------------");
    }

    fn take_turn(&mut self, player: Player) {
        // TODO: this will splode
        self.board.insert(
            *self
                .get_empty_tiles()
                .choose(&mut rand::thread_rng())
                .unwrap(),
            player,
        );
    }

    fn check(&self, player: Player) -> Option<GameResult> {
        // check horizontal, vert, diag for given player
        let is_winning_result = Game::WINNING_LINES
            .iter()
            .map(|line| {
                line.iter()
                    .map(|pos| self.board.get(pos))
                    .all(|piece| piece.map_or(false, |p| *p == player))
            })
            .any(std::convert::identity);

        if is_winning_result {
            return Some(GameResult::Win(player));
        } else if self.get_empty_tiles().is_empty() {
            return Some(GameResult::Draw);
        }

        return None;
    }
}

fn main() {
    let mut player = Player::Ex;
    let mut ex_stats = GameStats {
        wins: 0,
        losses: 0,
        draws: 0,
        player: Player::Ex,
    };
    let mut oh_stats = GameStats {
        wins: 0,
        losses: 0,
        draws: 0,
        player: Player::Oh,
    };

    for _ in 0..1000 {
        player = player.toggle();
        let mut game = Game::new();
        let result;

        loop {
            match game.check(player) {
                Some(r) => {
                    result = r;
                    break;
                }
                _ => player = player.toggle(),
            }
            game.take_turn(player);
        }

        match result {
            GameResult::Win(p) => match p {
                Player::Ex => {
                    ex_stats.wins += 1;
                    oh_stats.losses += 1;
                }
                Player::Oh => {
                    oh_stats.wins += 1;
                    ex_stats.losses += 1;
                }
            },
            GameResult::Draw => {
                ex_stats.draws += 1;
                oh_stats.draws += 1;
            }
        }
    }

    println!("{:?}", ex_stats);
    println!("{:?}", oh_stats);
}
