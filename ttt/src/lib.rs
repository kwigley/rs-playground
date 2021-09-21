use rand::prelude::SliceRandom;
use std::collections::HashMap;

use Position::{X, Y, Z};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Ex,
    Oh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: HashMap<(Position, Position), Player>,
    pub current_player: Player,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GameResult {
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
        [(X, X), (X, Y), (X, Z)],
        [(X, X), (Y, X), (Z, X)],
        [(X, X), (Y, Y), (Z, Z)],
        [(X, Y), (Y, Y), (Z, Y)],
        [(X, Z), (Y, Z), (Z, Z)],
        [(Y, X), (Y, Y), (Y, Z)],
        [(Z, X), (Y, Y), (X, Z)],
        [(Z, X), (Z, Y), (Z, Z)],
    ];

    pub fn new() -> Game {
        // TODO: this could splode
        // this could take a player as an arg and set as the current player
        // but let's just pick one at random
        return Game {
            board: HashMap::new(),
            current_player: *[Player::Ex, Player::Oh]
                .choose(&mut rand::thread_rng())
                .unwrap(),
        };
    }

    pub fn play(mut self) -> GameResult {
        loop {
            match self.check(self.current_player) {
                Some(r) => {
                    return r;
                }
                _ => self.current_player = self.current_player.toggle(),
            }
            self.take_turn(self.current_player);
        }
    }

    fn get_empty_tiles(&self) -> Vec<(Position, Position)> {
        Game::ALL_TILES
            .iter()
            .filter(|tile| !self.board.contains_key(&tile))
            .map(|x| *x)
            .collect()
    }

    fn take_turn(&mut self, player: Player) {
        // TODO: this could splode
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
