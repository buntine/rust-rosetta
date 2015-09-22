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

impl Direction {
    fn string(&self) -> String {
        let s = match *self { 
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
            _ => "Empty",
        };

        s.to_string()
    }
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
        match self.d {
            Direction::North => if self.t.valid_pos(self.x, self.y - 1) {self.y -= 1;},
            Direction::East => if self.t.valid_pos(self.x + 1, self.y) {self.x += 1;},
            Direction::South => if self.t.valid_pos(self.x, self.y + 1) {self.y += 1;},
            Direction::West => if self.t.valid_pos(self.x - 1, self.y) {self.y -= 1;},
            _ => (),
        }
    }

    fn report(&self) {
        println!("{}, {}, {}", self.x, self.y, self.d.string());
    }
}

fn main() {
    let mut robot: Robot = Default::default();

    robot.place(1, 2, Direction::North);
    robot.left();
}
