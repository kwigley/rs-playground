// use std::fmt::Display;

use crate::TicTacToeBoard;

#[derive(Clone, Debug, PartialEq)]
pub enum TicTacToeError {
    FullBoardError(TicTacToeBoard),
}

// impl Display for TicTacToeError {
//     fn disp
// }
