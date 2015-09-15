fn main() {
    let mut doors = [false; 100];
    
    for p in 0..100 {
        doors[p] = !doors[p];
    }

    for d in 0..doors.len() {
        if doors[d] {
            println!("{}", d + 1);
        }
    }
}
