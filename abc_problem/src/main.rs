fn can_spell(w: &str, blocks: &[&str]) -> bool {
    true
}

fn main() {
    let words = ["A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "CONFUSE"];
    let blocks = ["BO", "XK", "DQ", "CP", "NA", "GT", "RE",
                  "TG", "QD", "FS", "JW", "HU", "VI", "AN",
                  "OB", "ER", "FS", "LY", "PC", "ZM"];

    for w in words.iter() {
        println!("Can I spell {}: {}", w, can_spell(w, &blocks));
    }
}
