mod exceptions;
use crate::exceptions::TicTacToeError;
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

pub type TicTacToeBoard = HashMap<(Position, Position), Player>;

#[derive(Debug, Clone)]
pub struct Game {
    pub board: TicTacToeBoard,
    first_player: Player,
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
        let i: i32 = rand::random();
        let player = if i % 2 == 0 { Player::Ex } else { Player::Oh };
        Game {
            board: HashMap::new(),
            first_player: player,
        }
    }

    pub fn play(mut self) -> GameResult {
        loop {
            match self.check() {
                Some(r) => {
                    return r;
                },
                None => self.take_turn(player);
            }
        }
    }

    fn get_empty_tiles(&self) -> Vec<(Position, Position)> {
        Game::ALL_TILES
            .iter()
            .filter(|tile| !self.board.contains_key(&tile))
            .map(|x| *x)
            .collect()
    }

    fn take_turn(&mut self, player: Player) -> Result<(), TicTacToeError> {
        let empties = self.get_empty_tiles();
        let tile = empties
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| TicTacToeError::FullBoardError(self.board.clone()))?;
        self.board.insert(*tile, player);
        Ok(())
    }

    fn check(&self) -> Option<GameResult> {
        // check horizontal, vert, diag for given player
        let winning_player = Game::WINNING_LINES
            .iter()
            .map(|line| {
                let pieces: Vec<Option<&Player>> =
                    line.iter().map(|pos| self.board.get(pos)).collect();
                let head = pieces.first().and_then(|x| *x);
                let tail = &pieces[1..];
                tail.iter()
                    .fold(head, |acc, &p| if acc == p { p } else { None })
            })
            .fold(None, |_, p| if p.is_some() { p } else { None });

        match winning_player {
            Some(player) => Some(GameResult::Win(*player)),
            None if self.get_empty_tiles().is_empty() => Some(GameResult::Draw),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_ttt {
    use super::*;

    #[test]
    fn test_winning_state() {
        let mut board = HashMap::new();
        // X X _
        // O O X
        // X O O
        board.insert((X, X), Player::Ex);
        board.insert((X, Y), Player::Ex);
        board.insert((Y, X), Player::Oh);
        board.insert((Y, Y), Player::Oh);
        board.insert((Y, Z), Player::Ex);
        board.insert((Z, X), Player::Ex);
        board.insert((Z, Y), Player::Oh);
        board.insert((Z, Z), Player::Oh);
        let game = Game {
            board,
            current_player: Player::Oh,
        };
        let result = game.play();
        assert_eq!(result, GameResult::Win(Player::Ex))
    }

    #[test]
    fn test_draw_state() {
        // X O _
        // X O X
        // O X O
        let mut board = HashMap::new();
        board.insert((X, X), Player::Ex);
        board.insert((X, Y), Player::Oh);
        board.insert((Y, X), Player::Ex);
        board.insert((Y, Y), Player::Oh);
        board.insert((Y, Z), Player::Ex);
        board.insert((Z, X), Player::Oh);
        board.insert((Z, Y), Player::Ex);
        board.insert((Z, Z), Player::Oh);
        let game = Game {
            board,
            current_player: Player::Oh,
        };
        let result = game.play();
        assert_eq!(result, GameResult::Draw)
    }
}
