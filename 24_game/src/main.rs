struct Stack {
    content: Vec<i32>,
}

impl Stack {
    fn push(&mut self, d: i32) {
        self.content.push(d);
    }

    fn multiply(&mut self) {
        println!("Multiply");
    }

    fn add(&mut self) {
        println!("Add");
    }

    fn subtract(&mut self) {
        println!("Subtract");
    }

    fn divide(&mut self) {
        println!("Divide");
    }
}

fn evaluate(program: Vec<char>) {
    let mut stack = Stack { content: vec![] };

    for c in program {
        match c {
            '*' => stack.multiply(),
            '+' => stack.add(),
            '-' => stack.subtract(),
            '/' => stack.divide(),
            '1' => stack.push(1),
            '2' => stack.push(2),
            '3' => stack.push(3),
            '4' => stack.push(4),
            '5' => stack.push(5),
            '6' => stack.push(6),
            '7' => stack.push(7),
            '8' => stack.push(8),
            '9' => stack.push(9),
            _   => println!("DIE..."),
        }
    }

    for c in stack.content {
        println!("{}", c);
    }
}

fn main() {
    let program: Vec<char> = "3 4 * 2 * 1 /".chars().filter(|&c| c != ' ').collect();

    evaluate(program);
}
