extern crate rand;

use std::iter::repeat;
use rand::{thread_rng, Rng};

fn generate_expression(size: &usize) -> Vec<char> {
    let opens = repeat('[').take(*size);
    let closes = repeat(']').take(*size);
    let mut expr: Vec<char> = opens.chain(closes).collect();

    // Shuffle in place.
    thread_rng().shuffle(&mut expr[..]);

    expr
}

fn well_formed(expr: &Vec<char>) -> bool {
    let mut count = 0;

    for b in expr {
        match *b {
            '[' => count += 1,
            ']' => count -= 1,
            _ => panic!("Invalid expression!"),
        }

        if count < 0 { break; }
    }

    count == 0
}

fn main() {
    for n in (1..10) {
        let expr = generate_expression(&n);
        let wf = well_formed(&expr);

        println!("'{}' is {}",
                 expr.into_iter().collect::<String>(),
                 if wf {"well formed"} else {"NOT well formed"});
    }
}
