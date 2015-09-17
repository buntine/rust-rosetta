struct Stack {
    content: Vec<u32>,
}

impl Stack {
    fn push(&mut self, d: u32) -> Result<u32, &str> {
        self.content.push(d);
        return Ok(d);
    }

    fn pop(&mut self) -> Option<u32> {
        return self.content.pop();
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

        match l {
            Some(n) => {
                match r {
                    Some(i) => self.push(n * i),
                    None => Err("Stack empty"),
                }
            },
            None => Err("Stack empty"),
        }
    }

    fn add(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        match l {
            Some(n) => {
                match r {
                    Some(i) => self.push(n * i),
                    None => Err("Stack empty"),
                }
            },
            None => Err("Stack empty"),
        }
    }

    fn subtract(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        match l {
            Some(n) => {
                match r {
                    Some(i) => self.push(n * i),
                    None => Err("Stack empty"),
                }
            },
            None => Err("Stack empty"),
        }
    }

    fn divide(&mut self) -> Result<u32, &str> {
        let r = self.pop();
        let l = self.pop();

        match l {
            Some(n) => {
                match r {
                    Some(i) => self.push(n * i),
                    None => Err("Stack empty"),
                }
            },
            None => Err("Stack empty"),
        }
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

        if answer.is_err() {
            println!("{}", answer.unwrap_err());
            break;
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
