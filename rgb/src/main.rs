struct Gradient {
    start: [i32; 3],
    steps: [i32; 3],
    n: i32,
    total: i32,
}

impl Gradient {
    fn new(start: String, stop: String, steps: i32) -> Gradient {
        Gradient{
            start: [0,0,0],
            steps: [17,17,17],
            n: 0,
            total: steps
        }
    }
}

impl Iterator for Gradient {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.n {
            n if n == self.total => None,
            n => {
                let r = self.start[0] as i32 + (self.steps[0] as i32 * n);
                let g = self.start[1] as i32 + (self.steps[1] as i32 * n);
                let b = self.start[2] as i32 + (self.steps[2] as i32 * n);

                self.n += 1;

                Some(format!("{:02x}{:02x}{:02x}", r, g, b))
            }
        }
    }
}

#[test]
fn it_works() {
    let g = Gradient::new("000000".to_string(), "ffffff".to_string(), 16);

    assert_eq!(g.take(2).collect::<Vec<String>>(), vec!["000000".to_string(), "111111".to_string()]);
}

fn main() {
    let g = Gradient::new("000000".to_string(), "ffffff".to_string(), 10000);

    g.collect::<Vec<String>>();
}
