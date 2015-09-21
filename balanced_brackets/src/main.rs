use std::iter::repeat;

fn generate_expression(size: &usize) -> Vec<char> {
    let opens = repeat('[').take(*size);
    let closes = repeat(']').take(*size);
    let expr = opens.chain(closes);

    expr.collect()
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

        println!("'{}' is {}",
                 expr.into_iter().collect::<String>(),
                 if wf {"well formed"} else {"NOT well formed"});
    }
}
