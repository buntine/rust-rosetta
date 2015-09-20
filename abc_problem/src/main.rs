fn can_spell(w: &str, blocks: &Vec<&str>) -> bool {
    fn solve(chars: Vec<char>, avail_blocks: &Vec<&str>) -> bool {
        if chars.is_empty() {
            true
        } else if avail_blocks.is_empty() {
            false
        } else {
            let pos = avail_blocks.iter().position(|&b| b.chars().any(|c| c == chars[0]));

            match pos {
                Some(n) => {
                    let remaining_blocks = avail_blocks
                            .iter()
                            .enumerate()
                            .filter(|&(i, _)| i != n)
                            .map(|(_, &b)| b).collect();

                    solve(chars[1..].to_vec(), &remaining_blocks)
                },
                None => false
            }
        }
    }

    solve(w.chars().collect(), blocks)
}

fn main() {
    let words = ["A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "CONFUSE"];
    let blocks = vec!["BO", "XK", "DQ", "CP", "NA", "GT", "RE",
                      "TG", "QD", "FS", "JW", "HU", "VI", "AN",
                      "OB", "ER", "FS", "LY", "PC", "ZM"];

    for w in words.iter() {
        println!("Can I spell {}: {}", w, can_spell(w, &blocks));
    }
}
