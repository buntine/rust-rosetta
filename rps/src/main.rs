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

impl Game {
    fn new() -> Game {
        Game{frequencies: (0, 0, 0)}
    }

    fn play(&self, human: Move) -> GameResult {
    }
}

fn main() {
   let mut game = Game::new();

   let result = match game.play() {
       GameResult::Human => "Human wins!",
       GameResult::Computer => "Computer wins!",
       GameResult::Draw => "Draw!",
   };

   println!("{}", result);
}
