use std::collections::HashMap;

use ttt::Position::{X, Y, Z};
use ttt::{Game, GameResult, Player};

#[test]
fn test_winning_state() {
    let mut board = HashMap::new();
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
