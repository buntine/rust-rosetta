struct Gradient;
struct GradientIter {
    start: Vec<i32>,
    steps: Vec<f32>,
    n: i32,
    total: i32,
}

impl Gradient {
    fn to_parts(&self, hex: &str) -> Vec<i32> {
        [0, 2, 4].iter()
                 .map(|&n| i32::from_str_radix(&hex[n..(n + 2)], 16).unwrap())
                 .collect()
    }

    fn to_steps(&self, start: &[i32], stop: &[i32], n: i32) -> Vec<f32> {
        start.iter()
             .zip(stop.iter())
             .map(|(&a, &b)| (a - b) as f32 / (n as f32 - 1.0))
             .collect()
    }

    fn gradient_iter(&self, start: &str, stop: &str, n: i32) -> GradientIter {
        let from = self.to_parts(&start);
        let to = self.to_parts(&stop);
        let steps = self.to_steps(&from[..], &to[..], n);

        GradientIter{
            start: from,
            steps: steps,
            n: 0,
            total: n,
        }
    }
}

impl Iterator for GradientIter {
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
    let ga = Gradient.gradient_iter("000000", "ffffff", 10000);
    ga.collect::<Vec<String>>();
}

#[test]
fn it_works() {
    let g = Gradient;
    let ga = g.gradient_iter("000000", "ffffff", 16);
    let gb = g.gradient_iter("ff334c", "d8ff1a", 10);
    let result_a = vec!["000000", "111111", "222222", "333333", "444444",
                        "555555", "666666", "777777", "888888", "999999",
                        "aaaaaa", "bbbbbb", "cccccc", "dddddd", "eeeeee",
                        "ffffff"];
    let result_b = vec!["ff334c", "fb4947", "f76041", "f2773c", "ee8d36",
                        "eaa431", "e5bb2b", "e1d126", "dde820", "d8ff1a"];

    assert_eq!(ga.collect::<Vec<String>>(), result_a.iter().map(|r| r.to_string()).collect::<Vec<String>>());
    assert_eq!(gb.collect::<Vec<String>>(), result_b.iter().map(|r| r.to_string()).collect::<Vec<String>>());
} 
