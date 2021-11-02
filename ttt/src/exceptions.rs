// use std::fmt::Display;

use crate::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TicTacToeError {
    FullBoardError(Board),
    ExisitingTileError(Tile),
}

impl fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            TicTacToeError::FullBoardError(_) => "The board is full".to_owned(),
            TicTacToeError::ExisitingTileError(t) => {
                format!("There is an existing piece at {:?}", t)
            }
        };
        write!(f, "{}", msg)
    }
}
