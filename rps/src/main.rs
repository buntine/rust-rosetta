#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Human,
    Computer,
    Draw,
}

struct Game {
    frequencies: (u32, u32, u32),
}

impl Move {
    fn beaten_by(&self) ->  Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

impl Game {
    fn new() -> Game {
        Game{frequencies: (0, 0, 0)}
    }

    fn increment_frequency(&mut self, m: &Move) {
        match *m {
            Move::Rock => { self.frequencies.0 += 1; },
            Move::Paper => { self.frequencies.1 += 1; },
            Move::Scissors => { self.frequencies.2 += 1; },
        };
    }

    fn play(&mut self, human: Move) -> GameResult {
        let computer = Move::Paper;
        let best_answer = human.beaten_by();

        self.increment_frequency(&best_answer);

        if best_answer == computer {
            GameResult::Computer
        } else if computer.beaten_by() == human {
            GameResult::Human
        } else {
            GameResult::Draw
        }
    }
}

fn main() {
   let mut game = Game::new();

   let result = match game.play(Move::Paper) {
       GameResult::Human => "Human wins!",
       GameResult::Computer => "Computer wins!",
       GameResult::Draw => "Draw!",
   };

   println!("{}", result);
}
