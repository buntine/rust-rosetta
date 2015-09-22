#[derive(Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Empty,
}

struct Table {
    w: i32,
    h: i32,
}

struct Robot {
    x: i32,
    y: i32,
    d: Direction,
    t: Table,
}

impl Default for Robot {
    fn default() -> Robot {
        let table = Table{w: 5, h: 5};

        Robot{x: 0, y: 0, d: Direction::Empty, t: table}
    }
}

impl Table {
    fn valid_pos(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w && y >= 0 && y < self.h
    }
}

impl Robot {
    fn is_placed(&self) -> bool {
        self.d == Direction::Empty
    }

    fn place(&mut self, x: i32, y: i32, d: Direction) {
        if self.t.valid_pos(x, y) {
            self.x = x;
            self.y = y;
            self.d = d;
        }
    }

    fn right(&mut self) {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => Direction::Empty,
        };

        self.d = d;
    }

    fn left(&mut self) {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            _ => Direction::Empty,
        };

        self.d = d;
    }

    fn step(&mut self) {
    }

    fn report(&self) {
    }
}

fn main() {
    let mut robot: Robot = Default::default();

    robot.place(1, 2, Direction::North);
    robot.left();
}
