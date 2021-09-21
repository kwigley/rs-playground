use std::fmt;

use ttt::Game;
use ttt::GameResult;
use ttt::Player;

#[derive(Debug)]
struct Stats {
    oh_wins: i32,
    ex_wins: i32,
    draws: i32,
    total: i32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Games: {}\nX wins: {}\nO wins: {}\nDraws: {}",
            self.total, self.ex_wins, self.oh_wins, self.draws
        )
    }
}

fn main() {
    let mut stats = Stats {
        ex_wins: 0,
        oh_wins: 0,
        draws: 0,
        total: 0,
    };

    for _ in 0..1_000_000 {
        let game = Game::new();

        match game.play() {
            GameResult::Win(p) => match p {
                Player::Ex => {
                    stats.ex_wins += 1;
                }
                Player::Oh => {
                    stats.oh_wins += 1;
                }
            },
            GameResult::Draw => {
                stats.draws += 1;
            }
        }
        stats.total += 1;
    }

    println!("{}", stats);
}
