#[derive(Eq, PartialEq)]
enum Direction { North, East, South, West, Unplaced, }

use Direction::*;

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
    fn from_string(d: String) -> Direction {
        match &d[..] {
            "NORTH" => North,
            "EAST" => East,
            "SOUTH" => South,
            "WEST" => West,
            _ => Unplaced,
        }
    }

    fn to_string(&self) -> String {
        let s = match *self { 
            North => "NORTH",
            East => "EAST",
            South => "SOUTH",
            West => "WEST",
            _ => "UNPLACED",
        };

        s.to_string()
    }
}

impl Default for Robot {
    fn default() -> Robot {
        let table = Table{w: 5, h: 5};

        Robot{x: 0, y: 0, d: Unplaced, t: table}
    }
}

impl Table {
    fn valid_pos(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w && y >= 0 && y < self.h
    }
}

impl Robot {
    fn place(&mut self, x: i32, y: i32, d: Direction) {
        if self.t.valid_pos(x, y) && d != Unplaced {
            self.x = x;
            self.y = y;
            self.d = d;
        }
    }

    fn right(&mut self) {
        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
            _ => Unplaced,
        };

        self.d = d;
    }

    fn left(&mut self) {
        let d = match self.d {
            North => West,
            West => South,
            South => East,
            East => North,
            _ => Unplaced,
        };

        self.d = d;
    }

    fn step(&mut self) {
        match self.d {
            North => if self.t.valid_pos(self.x, self.y + 1) {self.y += 1;},
            East => if self.t.valid_pos(self.x + 1, self.y) {self.x += 1;},
            South => if self.t.valid_pos(self.x, self.y - 1) {self.y -= 1;},
            West => if self.t.valid_pos(self.x - 1, self.y) {self.y -= 1;},
            _ => (),
        }
    }

    fn report(&self) {
        println!("{}, {}, {}", self.x, self.y, self.d.to_string());
    }
}

fn parse_place(args: &[&str]) -> (i32, i32, Direction) {
    (1, 2, East)
}

fn main() {
    let mut robot: Robot = Default::default();
    let commands = "MOVE
                    MOVE
                    LEFT
                    PLACE 1 2 EAST
                    MOVE
                    MOVE
                    LEFT
                    MOVE
                    REPORT";

    let parsed: Vec<&str> = commands.split("\n").map(|c| c.trim()).collect();

    for c in parsed {
        let args: Vec<&str> = c.split(" ").collect();

        if let Some(arg) = args.first() {
            match *arg {
                "PLACE" if args.len() == 4 => {
                    let (x, y, d) = parse_place(&args[1..]);
                    robot.place(x, y, d);
                },
                "MOVE" => robot.step(),
                "LEFT" => robot.left(),
                "RIGHT" => robot.right(),
                "REPORT" => robot.report(),
                _ => (),
            }
        }
    }
}
