use std::fs::read_to_string;

#[macro_export]
macro_rules! input {
    ( $x:expr ) => {{
        Input { path: $x }
    }};
}

#[macro_export]
macro_rules! debug {
    ($x:expr) => {
        print!("{:?}\n", $x)
    };
}

#[macro_export]
macro_rules! run_day {
    ($day:path) => {{
        use $day::*;
        println!(
            "{}\n    part1:    {:?}\n    part2:    {:?}",
            stringify!($day),
            _1(),
            _2()
        );
    }};
}

#[derive(Debug)]
pub struct Input {
    pub path: &'static str,
}

impl Input {
    pub fn as_string(&self) -> Vec<String> {
        let content = read_to_string(self.path).expect("Something went wrong reading the file");
        content.split('\n').map(From::from).collect()
    }

    pub fn as_group(&self) -> Vec<String> {
        let content = read_to_string(self.path).expect("Something went wrong reading the file");
        content.split("\n\n").map(From::from).collect()
    }

    pub fn as_int(&self) -> Vec<i32> {
        let content = read_to_string(self.path).expect("Something went wrong reading the file");
        content
            .split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}
