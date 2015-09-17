struct Stack {
    content: Vec<u32>,
}

impl Stack {
    fn push(&mut self, d: u32) -> Result<u32, &str> {
        self.content.push(d);
        return Ok(d);
    }

    fn pop(&mut self) -> u32 {
        self.content.pop().expect("Stack empty!")
    }

    fn add_digit(&mut self, c: char) -> Result<u32, &str> {
        match c.to_digit(10) {
            Some(n) => self.push(n),
            None    => Err("Invalid character"),
        }
    }

    fn multiply(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        return self.push(l * r);
    }

    fn add(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        return self.push(l + r);
    }

    fn subtract(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        return self.push(l - r);
    }

    fn divide(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        return self.push(l / r);
    }
}

fn evaluate(program: Vec<char>) {
    let mut stack = Stack { content: vec![] };

    for c in program {
        let answer: Result<u32, &str> = match c {
            '*' => stack.multiply(),
            '+' => stack.add(),
            '-' => stack.subtract(),
            '/' => stack.divide(),
            _   => stack.add_digit(c),
        };

        match answer {
            Err(e) => {
                println!("{}", e);
                break;
            },
            _      => continue,
        };
    }

    for c in stack.content {
        println!("{}", c);
    }
}

fn main() {
    let program: Vec<char> = "3 4 * 2 * 1 /".chars().filter(|&c| c != ' ').collect();

    evaluate(program);
}
