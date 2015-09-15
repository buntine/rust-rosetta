fn main() {
    let mut doors = [false; 100];
    let mut d;
    
    for p in 0..100 {
        d = p;
        while d < doors.len() {
            doors[d] = !doors[d];
            d += (p + 1);
        }
    }

    for (i, &d) in doors.iter().enumerate() {
        if d {
            println!("{}", i + 1);
        }
    }
}
