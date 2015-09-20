fn can_spell(w: &str, blocks: &Vec<&str>) -> bool {
    fn solve(chars: Vec<&char>, avail_blocks: &Vec<&str>) -> bool {
        if chars.len() == 0 {
            true
        } else if avail_blocks.len() == 0 {
            false
        } else {
            match avail_blocks.iter().position(|&b| b.char_at(0) == *chars[0] || b.char_at(1) == *chars[0]) {
                Some(i) => solve(chars.tail(), avail_blocks),
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
