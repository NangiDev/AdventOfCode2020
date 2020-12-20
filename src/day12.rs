use crate::{input, run_day, utils::Input};

#[derive(Debug)]
pub struct Waypoint {
    pub x: i32,
    pub y: i32,
}

impl Waypoint {
    pub fn new() -> Self {
        Self { x: 10, y: -1 }
    }

    pub fn move_north(self: &mut Waypoint, step: i32) {
        self.y -= step;
    }

    pub fn move_south(self: &mut Waypoint, step: i32) {
        self.y += step;
    }

    pub fn move_west(self: &mut Waypoint, step: i32) {
        self.x -= step;
    }

    pub fn move_east(self: &mut Waypoint, step: i32) {
        self.x += step;
    }

    pub fn rotate_right(self: &mut Waypoint, deg: i32) {
        match deg {
            90 => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            }
            180 => {
                self.x *= -1;
                self.y *= -1;
            }
            270 => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            }
            _ => {}
        }
    }

    pub fn rotate_left(self: &mut Waypoint, deg: i32) {
        match deg {
            90 => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            }
            180 => {
                self.x *= -1;
                self.y *= -1;
            }
            270 => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Ferry {
    pub x: i32,
    pub y: i32,
    pub waypoint: Waypoint,
    dx: i32,
    dy: i32,
    deg: i32,
}

impl Ferry {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            waypoint: Waypoint::new(),
            dx: 1,
            dy: 0,
            deg: 0,
        }
    }

    pub fn move_to_waypoint(self: &mut Ferry, step: i32) {
        self.x += step * self.waypoint.x;
        self.y += step * self.waypoint.y;
    }

    pub fn move_forward(self: &mut Ferry, step: i32) {
        self.x += step * self.dx;
        self.y += step * self.dy;
    }

    pub fn move_north(self: &mut Ferry, step: i32) {
        self.y -= step;
    }

    pub fn move_south(self: &mut Ferry, step: i32) {
        self.y += step;
    }

    pub fn move_west(self: &mut Ferry, step: i32) {
        self.x -= step;
    }

    pub fn move_east(self: &mut Ferry, step: i32) {
        self.x += step;
    }

    pub fn rotate_right(self: &mut Ferry, deg: i32) {
        self.deg += deg;

        if self.deg > 360 {
            self.deg -= 360;
        }

        self.set_degrees();
    }

    pub fn rotate_left(self: &mut Ferry, deg: i32) {
        self.deg -= deg;

        if self.deg < 0 {
            self.deg += 360;
        }

        self.set_degrees();
    }

    fn set_degrees(self: &mut Ferry) {
        match self.deg {
            90 => {
                self.dx = 0;
                self.dy = 1;
            }
            180 => {
                self.dx = -1;
                self.dy = 0;
            }
            270 => {
                self.dx = 0;
                self.dy = -1;
            }
            _ => {
                self.dx = 1;
                self.dy = 0;
            }
        }
    }
}

pub fn move_ferry_with_instructions(instructions: Vec<String>) -> Ferry {
    let mut ferry: Ferry = Ferry::new(0, 0);

    for inst in &instructions {
        let letter: &str = &inst[..1];
        let number: i32 = inst[1..].parse::<i32>().unwrap();

        match letter {
            "N" => ferry.move_north(number),
            "S" => ferry.move_south(number),
            "E" => ferry.move_east(number),
            "W" => ferry.move_west(number),
            "L" => ferry.rotate_left(number),
            "R" => ferry.rotate_right(number),
            "F" => ferry.move_forward(number),
            _ => {}
        }
    }

    ferry
}

pub fn move_ferry_with_waypoint_navigation(instructions: Vec<String>) -> Ferry {
    let mut ferry: Ferry = Ferry::new(0, 0);

    for inst in &instructions {
        let letter: &str = &inst[..1];
        let number: i32 = inst[1..].parse::<i32>().unwrap();

        match letter {
            "N" => ferry.waypoint.move_north(number),
            "S" => ferry.waypoint.move_south(number),
            "E" => ferry.waypoint.move_east(number),
            "W" => ferry.waypoint.move_west(number),
            "L" => ferry.waypoint.rotate_left(number),
            "R" => ferry.waypoint.rotate_right(number),
            "F" => ferry.move_to_waypoint(number),
            _ => {}
        }
    }

    ferry
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day12.txt");
    let ferry = move_ferry_with_instructions(input.as_string());
    ferry.x.abs() + ferry.y.abs()
}
pub fn _2() -> i32 {
    let input = input!("./src/input_files/day12.txt");
    let ferry = move_ferry_with_waypoint_navigation(input.as_string());
    ferry.x.abs() + ferry.y.abs()
}

pub fn print() {
    run_day!(crate::day12);
}
