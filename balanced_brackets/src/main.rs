fn generate_expression(size: &i32) -> Vec<char> {
    vec!['[', ']', ']', '[']
}

fn well_formed(expr: &Vec<char>) -> bool {
    let mut count = 0;

    for b in expr {
        if count < 0 {
            break;
        }

        match *b {
            '[' => count += 1,
            _ => count -= 1,
        }
    }

    count == 0
}

fn main() {
    for n in (1..10) {
        let expr = generate_expression(&n);
        let wf = well_formed(&expr);
        let wf_str: String = expr.into_iter().collect();

        println!("'{}' is {}",
                 wf_str,
                 if wf {"well formed"} else {"NOT well formed"});
    }
}
