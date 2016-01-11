extern crate regex;

use regex::Regex;

struct Gradient {
    start: Vec<u32>,
    steps: Vec<u32>,
    n: u32,
    total: u32,
}

impl Gradient {
    fn to_parts(hex: &String) -> Vec<u32> {
        vec![0, 0, 0]
    }

    fn to_steps(start: &[u32], stop: &[u32]) -> Vec<u32> {
        vec![17, 17, 17]
    }

    pub fn new(start: String, stop: String, n: u32) -> Gradient {
        let from = Gradient::to_parts(&start);
        let to = Gradient::to_parts(&stop);
        let steps = Gradient::to_steps(&from[..], &to[..]);

        Gradient{
            start: from,
            steps: steps,
            n: 0,
            total: n,
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
