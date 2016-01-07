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
            start: [1,2,3],
            stop: [1,2,3],
            steps: [1,2,3],
            n: 0,
            total: 16
        }
    }
}

impl Iterator for Gradient {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.n += 1;
        Some("FF00FF".to_string())
    }
}

#[test]
fn it_works() {
  let g = Gradient::new("000000".to_string(), "FFFFFF".to_string(), 16);

  assert_eq!(g.take(2).collect::<Vec<String>>(), vec!["FF00FF".to_string(), "FF00FF".to_string()]);
}

fn main() {
    println!("Hello, world!");
}
