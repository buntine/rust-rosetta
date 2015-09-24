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

trait FirstChar {
    fn first_char(&self) -> Option<char>;
}

impl FirstChar for str {
    fn first_char(&self) -> Option<char> {
        let chars = self.chars().collect::<Vec<char>>();
        
        match chars.first() {
            Some(c) => Some(*c),
            _ => None,
        }
    }
}

impl Direction {
    fn from_string(d: &str) -> Option<Direction> {
        match d {
            "NORTH" => Some(North),
            "EAST" => Some(East),
            "SOUTH" => Some(South),
            "WEST" => Some(West),
            _ => None,
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
    let (x, y, d) = (args[0].first_char().expect("Invalid place"),
                     args[1].first_char().expect("Invalid place"),
                     Direction::from_string(args[2]));

    match (x.to_digit(10), y.to_digit(10), d) {
        (Some(xpos), Some(ypos), Some(dir)) => (xpos as i32, ypos as i32, dir),
        _ => (0, 0, Unplaced),
    }
}

fn main() {
    let programs = ["PLACE 0 0 NORTH\nMOVE\nREPORT",
                    "PLACE 0 0 NORTH\nLEFT\nREPORT",
                    "MOVE\nLEFT\nPLACE 1 2 EAST\nEAST\nMOVE\nMOVE\nLEFT\nMOVE\nREPORT",
                    "PLACE 0 0 EAST\nMOVE\nMOVE\nMOVE\nMOVE\nMOVE\nMOVE\nMOVE\nREPORT\nLEFT\nMOVE\nREPORT"];

    for p in programs.iter() {
        let mut robot: Robot = Default::default();
        let parsed: Vec<&str> = p.split("\n").map(|c| c.trim()).collect();

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
}
