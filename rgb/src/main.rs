struct Gradient {
    start: Vec<i32>,
    steps: Vec<f32>,
    n: i32,
    total: i32,
}

impl Gradient {
    fn to_parts(hex: &str) -> Vec<i32> {
        [0, 2, 4].iter()
                 .map(|&n| i32::from_str_radix(&hex[n..(n + 2)], 16).unwrap())
                 .collect()
    }

    fn to_steps(start: &[i32], stop: &[i32], n: i32) -> Vec<f32> {
        start.iter()
             .zip(stop.iter())
             .map(|(&a, &b)| (a as f32 - b as f32) / (n as f32 - 1.0))
             .collect()
    }

    pub fn new(start: &str, stop: &str, n: i32) -> Gradient {
        let from = Gradient::to_parts(&start);
        let to = Gradient::to_parts(&stop);
        let steps = Gradient::to_steps(&from[..], &to[..], n);

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
                    .map(|(&a, &b)| a - (n as f32 * b) as i32)
                    .map(|i| format!("{:02x}", i))
                    .collect();

                self.n += 1;

                Some(rgb)
            }
        }
    }
}

fn main() {
    // For benchmarking...
    let g = Gradient::new("000000", "ffffff", 10000);
    g.collect::<Vec<String>>();
}

#[test]
fn it_works() {
    let ga = Gradient::new("000000", "ffffff", 16);
    let gb = Gradient::new("ff334c", "d8ff1a", 10);
    let result_a = vec!["000000", "111111", "222222", "333333", "444444",
                        "555555", "666666", "777777", "888888", "999999",
                        "aaaaaa", "bbbbbb", "cccccc", "dddddd", "eeeeee",
                        "ffffff"];
    let result_b = vec!["ff334c", "fb4947", "f76041", "f2773c", "ee8d36",
                        "eaa431", "e5bb2b", "e1d126", "dde820", "d8ff1a"];

    assert_eq!(ga.collect::<Vec<String>>(), result_a.iter().map(|r| r.to_string()).collect::<Vec<String>>());
    assert_eq!(gb.collect::<Vec<String>>(), result_b.iter().map(|r| r.to_string()).collect::<Vec<String>>());
} 
