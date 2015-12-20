use std::io;

#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
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

    fn pick(&self) -> Move {
        Move::Paper
    }

    fn play(&mut self, human: &Move) -> GameResult {
        let computer = self.pick();
        let best_answer = human.beaten_by();

        self.increment_frequency(&best_answer);

        if best_answer == computer {
            GameResult::Computer
        } else if computer.beaten_by() == *human {
            GameResult::Human
        } else {
            GameResult::Draw
        }
    }
}

fn main() {
   let mut game = Game::new();

   loop {
       println!("Enter a guess (r, p, s);");

       let mut input = String::new();

       io::stdin()
           .read_line(&mut input)
           .ok()
           .expect("Cuold not read input.");

       let human = match input.trim() {
           "r" => Move::Rock,
           "p" => Move::Paper,
           "s" => Move::Scissors,
           _ => { continue; },
       };

       println!("Winner: {:?}", game.play(&human));
   }
}
