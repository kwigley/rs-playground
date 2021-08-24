use rand::Rng;

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

// TODO: represent board with a hash map
#[derive(Debug, Clone)]
struct Game {
    board: Vec<Option<Player>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Game {
    fn new() -> Game {
        return Game {
            board: vec![Option::None; 9],
        };
    }
    // TODO: impl as display trait
    fn print(self, game_num: i32) {
        println!("game num {}", game_num);
        for row in self.board {
            println!("{:?}", row);
        }
        println!("--------------");
    }
    fn take_turn(&mut self, player: Player) {
        // TODO: improve board selection
        // randomly select a postion until valid
        let mut rng = rand::thread_rng();
        let pos = rng.gen_range(0..9);
        match self.board[pos] {
            Some(_) => self.take_turn(player),
            None => self.board[pos] = Some(player),
        }
    }
    fn check(&self) -> Option<GameResult> {
        for player in [Player::Ex, Player::Oh] {
            for i in 0..3 {
                // check vertical
                // x _ _ --> 0 3 6
                // x _ _
                // x _ _
                let vertical = self
                    .board
                    .iter()
                    .skip(i)
                    .step_by(3)
                    .all(|v| *v == Some(player));

                // check horizontal
                // x x x --> 0 1 3
                // _ _ _
                // _ _ _
                let horizontal = self
                    .board
                    .iter()
                    .skip(i * 3)
                    .take(3)
                    .all(|v| *v == Some(player));

                if vertical || horizontal {
                    return Some(GameResult::Win(player));
                }
            }
            // check diag (left->right)
            // x _ _ --> 0 4 8
            // _ x _
            // _ _ x
            if self.board[0] == Some(player)
                && self.board[4] == Some(player)
                && self.board[8] == Some(player)
            {
                return Some(GameResult::Win(player));
            }
            // check diag (right->left)
            // _ _ x --> 2 4 6
            // _ x _
            // x _ _
            if self.board[2] == Some(player)
                && self.board[4] == Some(player)
                && self.board[6] == Some(player)
            {
                return Some(GameResult::Win(player));
            }
        }

        // TODO: test for checking winning case before draw
        if self.board.iter().all(|x| x.is_some()) {
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

    for _ in 0..1_000_000 {
        player = player.toggle();
        let mut game = Game::new();
        let result;

        loop {
            match game.check() {
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
