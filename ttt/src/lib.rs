mod exceptions;
use crate::exceptions::TicTacToeError;
use rand::prelude::SliceRandom;
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

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

// pub type TicTacToeBoard = HashMap<(osition, Position), Player>;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    board: HashMap<Tile, Player>,
}

impl Board {
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

    pub fn new() -> Self {
        Board {
            board: HashMap::new(),
        }
    }
    pub fn insert(&mut self, tile: Tile, player: Player) -> Result<(), TicTacToeError> {
        //TODO: test this behavior
        if self.board.contains_key(&tile) {
            Err(TicTacToeError::ExisitingTileError(tile))
        } else {
            self.board.insert(tile, player);
            Ok(())
        }
    }
    pub fn get(&self, tile: Tile) -> Option<Player> {
        self.board.get(&tile).copied()
    }
    fn empty_tiles(&self) -> Vec<Tile> {
        Board::ALL_TILES
            .iter()
            .filter(|tile| !self.board.contains_key(&tile))
            .map(|x| *x)
            .collect()
    }
    fn used_tiles(&self) -> Vec<Tile> {
        Board::ALL_TILES
            .iter()
            .filter(|tile| self.board.contains_key(&tile))
            .map(|x| *x)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
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

struct DisplayTile {
    tile: Tile,
}

impl Display for DisplayTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // using debug output for Position!
        write!(f, "({:?}, {:?})", self.tile.0, self.tile.1)
    }
}

impl Game {
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
            board: Board::new(),
            first_player: player,
        }
    }

    pub fn play(mut self) -> Result<GameResult, TicTacToeError> {
        loop {
            match self.check() {
                Some(r) => {
                    return Ok(r);
                }
                None => self.take_turn()?,
            }
        }
    }

    fn current_player(&self) -> Player {
        // get number of used tiles
        // if even -> current player = first player
        // if odd --> current player = other player
        if self.board.used_tiles().len() % 2 == 0 {
            self.first_player
        } else {
            self.first_player.toggle()
        }
    }

    fn take_turn(&mut self) -> Result<(), TicTacToeError> {
        let empties = self.board.empty_tiles();
        let tile = empties
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| TicTacToeError::FullBoardError(self.board.clone()))?;
        self.board.insert(*tile, self.current_player())
    }

    fn check(&self) -> Option<GameResult> {
        // check horizontal, vert, diag for given player
        let winning_player = Game::WINNING_LINES
            .iter()
            .map(|line| {
                let pieces: Vec<Option<Player>> = line
                    .iter()
                    .map(|pos| self.board.get(*pos)) //.map(|player| *player))
                    .collect();
                if let Some((head, tail)) = pieces.as_slice().split_first() {
                    tail.iter()
                        .fold(*head, |acc, &p| if acc == p { p } else { None })
                } else {
                    None
                }
            })
            .fold(None, |_, p| if p.is_some() { p } else { None });

        match winning_player {
            Some(player) => Some(GameResult::Win(player)),
            None if self.board.empty_tiles().is_empty() => Some(GameResult::Draw),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_ttt {
    use super::*;

    fn play_game(turns: &[(Tile, Player)]) -> Result<GameResult, TicTacToeError> {
        let mut game = Game::new();
        for turn in turns {
            game.board.insert(turn.0, turn.1)?;
        }
        Ok(game.check().unwrap())
    }

    // #[test]
    // fn test_winning_state() {
    //     let mut board = HashMap::new();
    //     // X X _
    //     // O O X
    //     // X O O
    //     board.insert((X, X), Player::Ex);
    //     board.insert((X, Y), Player::Ex);
    //     board.insert((Y, X), Player::Oh);
    //     board.insert((Y, Y), Player::Oh);
    //     board.insert((Y, Z), Player::Ex);
    //     board.insert((Z, X), Player::Ex);
    //     board.insert((Z, Y), Player::Oh);
    //     board.insert((Z, Z), Player::Oh);
    //     let game = Game {
    //         board,
    //         first_player: Player::Ex,
    //     };
    //     let result = game.play();
    //     assert_eq!(result, Ok(GameResult::Win(Player::Ex)))
    // }

    #[test]
    fn test_draw_state() {
        // X O X
        // X O X
        // O X O
        let turns = vec![
            ((X, X), Player::Ex),
            ((X, Y), Player::Oh),
            ((Y, X), Player::Ex),
            ((Y, Y), Player::Oh),
            ((Y, Z), Player::Ex),
            ((Z, X), Player::Oh),
            ((Z, Y), Player::Ex),
            ((Z, Z), Player::Oh),
            ((X, Z), Player::Ex),
        ];
        let result = play_game(&turns);
        assert_eq!(result, Ok(GameResult::Draw))
    }
}
