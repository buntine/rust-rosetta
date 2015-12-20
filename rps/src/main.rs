extern crate rand;

use std::io;
use std::slice;
use rand::Rng;

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
    frequencies: [u32; 3],
}

impl Move {
    fn beaten_by(&self) ->  Move {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn variants() -> slice::Iter<'static, Move> {
        static VARIANTS: &'static [Move] = &[Move::Rock, Move::Paper, Move::Scissors];

        VARIANTS.iter()
    }
}

impl Game {
    fn new() -> Game {
        Game{frequencies: [0, 0, 0]}
    }

    fn increment_frequency(&mut self, m: &Move) {
        match Move::variants().position(|n| n == m) {
            Some(i) => { self.frequencies[i] += 1 },
            None => { panic!("Unknown frequency.") },
        }
    }

    fn pick(&self) -> Move {
        let total = self.frequencies.iter().fold(0, |t, n| t + n);
        let guess = rand::thread_rng().gen_range(0, total + 1);
        let mut sum: u32 = 0;
         
        for (&f, c) in self.frequencies.iter().zip(Move::variants()) {
            sum += f;

            if guess <= sum {
                return c.beaten_by();
            }
        }

        return Move::Rock;
    }

    fn play(&mut self, human: &Move) -> GameResult {
        let computer = self.pick();

        self.increment_frequency(&human);

        if human.beaten_by() == computer {
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
           .expect("Could not read input.");

       let human = match input.trim() {
           "r" => Move::Rock,
           "p" => Move::Paper,
           "s" => Move::Scissors,
           _ => { continue; },
       };

       println!("Winner: {:?}", game.play(&human));
   }
}
