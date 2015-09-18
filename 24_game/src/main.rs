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
        self.digits.retain(|&n| n != d);
    }

    fn valid_digit(&self, d: i32) -> bool {
        self.digits.contains(&d)
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
            _   => stack.add_digit(c),
        };

        if answer.is_err() {
            println!("Actual: {}", answer.unwrap_err());
            return Err("Ass");
        }
    }

    match stack.content.len() {
        1 => Ok(stack.content[0]),
        _ => Err("Invalid formula"),
    }
}

fn main() {
    let program: Vec<char> = "3 4 * 2 * 1 /".chars().filter(|&c| c != ' ').collect();
    let digits: Vec<i32> = vec![1, 2, 3, 4];

    match evaluate(program, digits) {
        Ok(n) => match n {
            24 => println!("Well done!"),
            _  => println!("Nice try, but {} is not 24!", n),
        },
        Err(s) => {
            println!("Oops: {}", s);
        },
    }
}
