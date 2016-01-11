extern crate regex;

use regex::Regex;

struct Gradient {
    start: Vec<u32>,
    steps: Vec<u32>,
    n: u32,
    total: u32,
}

impl Gradient {
    fn to_parts(hex: &str) -> Vec<u32> {
        let re = Regex::new(r"[0-9a-fA-F]{2}").unwrap();

        re.split(&hex)
          .map(|c| u32::from_str_radix(c, 16).unwrap_or(0))
          .collect()
    }

    fn to_steps(start: &[u32], stop: &[u32]) -> Vec<u32> {
        vec![17, 17, 17]
    }

    pub fn new(start: String, stop: String, n: u32) -> Gradient {
        let from = Gradient::to_parts(&start[..]);
        let to = Gradient::to_parts(&stop[..]);
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
    let result = vec!["000000", "111111", "222222", "333333", "444444",
                      "555555", "666666", "777777", "888888", "999999",
                      "aaaaaa", "bbbbbb", "cccccc", "dddddd", "eeeeee",
                      "ffffff"];

    assert_eq!(g.collect::<Vec<String>>(), result.iter().map(|r| r.to_string()).collect::<Vec<String>>());
}
