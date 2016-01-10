struct Gradient {
    start: Vec<i32>,
    steps: Vec<i32>,
    n: i32,
    total: i32,
}

impl Gradient {
    fn new(start: String, stop: String, steps: i32) -> Gradient {
        Gradient{
            start: vec![0, 0, 0],
            steps: vec![17,17,17],
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
                let rgb: String = self.start
                    .iter()
                    .zip(self.steps.iter())
                    .map(|(&a, &b)| a + (b * n))
                    .map(|i| format!("{:02x}", i))
                    .collect();

                self.n += 1;

                Some(rgb)
            }
        }
    }
}

fn main() {
    let g = Gradient::new("000000".to_string(), "ffffff".to_string(), 10000);

    g.collect::<Vec<String>>();
}

#[test]
fn it_works() {
    let g = Gradient::new("000000".to_string(), "ffffff".to_string(), 16);

    assert_eq!(g.take(2).collect::<Vec<String>>(), vec!["000000".to_string(), "111111".to_string()]);
}
