fn evaluate(program: Vec<char>) {
    let stack: Vec<i32> = vec![];

    for c in program {
        match c {
            '*' => println!("Multiply last two on stack"),
            '+' => println!("Sum last two on stack"),
            '-' => println!("Subtract last two on stack"),
            '/' => println!("Divide last two on stack"),
            _   => println!("Find digit -> Add to stack. Else -> Die"),
        }
    }
}

fn main() {
    let program: Vec<char> = "3 4 * 2 * 1 /".chars().filter(|&c| c != ' ').collect();

    evaluate(program);
}
