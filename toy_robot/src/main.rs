enum Direction {
    North,
    East,
    South,
    West,
    Empty,
}

struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Default for Robot {
    fn default() -> Robot {
        Robot{x: 0, y: 0, d: Direction::Empty}
    }
}

impl Robot {
    fn place(&self, x: i32, y: i32, d: Direction) {
        println!("Woohoo");
    }
}

fn main() {
    let robot: Robot = Default::default();

    robot.place(1, 2, Direction::North);
}
