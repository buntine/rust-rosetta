fn generate_expression(size: &i32) -> Vec<char> {
    vec!['[', '[', ']', ']']
}

fn well_formed(expr: &Vec<char>) -> bool {
    true
}

fn main() {
    for n in (1..10) {
        let expr = generate_expression(&n);
        let wf = well_formed(&expr);

        println!("{:?} is {}", expr, if wf {"well formed"} else {"NOT well formed"});
    }
}
