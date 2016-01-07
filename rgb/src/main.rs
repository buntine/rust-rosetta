struct Gradient {
    start: [i8; 3],
    stop: [i8; 3],
    steps: [i8; 3],
    n: i32,
    total: i32,
}

impl Gradient {
    fn new(start: String, stop: String, steps: i32) -> Gradient {
        Gradient{
            start: [0,0,0],
            stop: [255,255,255],
            steps: [17,17,17],
            n: 0,
            total: 16
        }
    }
}

impl Iterator for Gradient {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.n {
            n if n == self.total => None,
            n => {
                Some(format!("{:x}{:02x}{:x}", 255, 0, 255))
            }
        }
    }
}

#[test]
fn it_works() {
  let g = Gradient::new("000000".to_string(), "ffffff".to_string(), 16);

  assert_eq!(g.take(2).collect::<Vec<String>>(), vec!["ff00ff".to_string(), "ff00ff".to_string()]);
}

fn main() {
    println!("Hello, world!");
}
