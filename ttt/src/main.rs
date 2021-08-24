#[derive(Debug, Clone)]
enum Selection {
    Ex,
    Oh,
    None,
}

#[derive(Debug, Clone)]
struct TicTacToe {
    board: Vec<Vec<Selection>>,
}

#[derive(Debug, Clone)]
struct Player {
    wins: i32,
    losses: i32,
    ties: i32,
}

impl Player {
    fn new() -> Player {
        return Player {
            wins: 0,
            losses: 0,
            ties: 0,
        };
    }
}

impl TicTacToe {
    fn new() -> TicTacToe {
        return TicTacToe {
            board: vec![vec![Selection::None; 3]; 3],
        };
    }
    fn print(self) {
        self.board.into
    }
}

fn main() {
    let player1 = Player::new();
    let player2 = Player::new();

    for _ in 0..100 {
        let game = TicTacToe::new();
        game.print();
    }

    println!("{:?}", player1);
    println!("{:?}", player2);
}
