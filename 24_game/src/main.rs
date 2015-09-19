extern crate rand;

use std::io;
use rand::{thread_rng, sample};

struct Stack {
    content: Vec<i32>,
    digits: Vec<i32>,
}

impl Stack {
    fn push(&mut self, d: i32) -> Result<i32, &str> {
        self.content.push(d);
        Ok(d)
    }

    fn pop(&mut self) -> Option<i32> {
        self.content.pop()
    }

    fn consume_digit(&mut self, d: i32) {
        let i = self.digits.iter().position(|&i| i == d).expect("Cannot consume!");
        self.digits.remove(i);
    }

    fn valid_digit(&self, d: i32) -> bool {
        self.digits.contains(&d)
    }

    fn reveal<'a>(&self) -> Result<i32, &'a str> {
        match self.content.len() {
            1 => match self.digits.len() {
                0 => Ok(self.content[0]),
                _ => Err("Did not use all digits!"),
            },
            _ => Err("Invalid formula!"),
        }
    }

    fn add_digit(&mut self, c: char) -> Result<i32, &str> {
        match c.to_digit(10) {
            Some(n) => match self.valid_digit(n as i32) {
                true => {
                    self.consume_digit(n as i32);
                    self.push(n as i32)
                },
                false => Err("Invalid or duplicate digit!"),
            },
            None => Err("Invalid character"),
        }
    }

    fn apply<F>(&mut self, f: F) -> Result<i32, &str>
        where F : Fn(i32, i32) -> i32 {

        let r = self.pop();
        let l = self.pop();

        match l {
            Some(n) => match r {
                Some(i) => self.push(f(n, i)),
                None => Err("Stack empty"),
            },
            None => Err("Stack empty"),
        }
    }
}

fn evaluate<'a>(program: Vec<char>, digits: Vec<i32>) -> Result<i32, &'a str> {
    let mut stack = Stack {
        content: vec![],
        digits: digits,
    };

    for c in program {
        let answer: Result<i32, &str> = match c {
            '*' => stack.apply(|l, r| l * r),
            '+' => stack.apply(|l, r| l + r),
            '-' => stack.apply(|l, r| l - r),
            '/' => stack.apply(|l, r| l / r),
            _ => stack.add_digit(c),
        };

        if answer.is_err() {
            // TODO: Fix this. Work out how to return "answer" without compiler error.
            //return answer;
            println!("Actual: {}", answer.unwrap_err());
            return Err("Program error");
        }
    }

    stack.reveal()
}

fn main() {
    let goal: i32 = 24;
    let mut rng = thread_rng();

    println!("GUESSING GAME!

RULES

  * Using only the given digits, create an expression that evaluates to 24.
  * All digits must be used.
  * Each digit may only be used once.
  * Expressions must be in Reverse-Polish Notation (RPN).
  * Only +, -, * and / are allowed.
  * Order of the digits is not important.\n");

    loop {
        let mut program = String::new();
        let digits = sample(&mut rng, 1..9, 4);

        println!("Your digits are: {}, {}, {} and {}\n", digits[0], digits[1], digits[2], digits[3]);
        println!("Enter an expression:\n");

        io::stdin().read_line(&mut program)
            .ok()
            .expect("Could not read input.");

        let parsed = program
            .trim()
            .chars()
            .filter(|&c| c != ' ')
            .collect();

        match evaluate(parsed, digits) {
            Ok(n) => if n == goal {
                println!("\nWell done!\n");
                break;
            } else {
                println!("\nNice try, but {} is not {}!\n", n, goal);
            },
            Err(s) => {
                println!("\nOops: {}\n", s);
            },
        }
    }
}
